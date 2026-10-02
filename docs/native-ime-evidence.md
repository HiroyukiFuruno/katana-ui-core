# Native IME evidence contract

Issue #81 の Windows / Linux 検証は、実機を保有していないため仮説を明示したうえで進める。GitHub Actions の三 OS runner は、KUC の registry-only verifier と native producer が各 OS でコンパイルできることを検証する。これは IME を操作した実測証跡ではない。

## 仮説

| OS | 想定する native 経路 | 仮説 |
| --- | --- | --- |
| macOS | winit の native IME event と macOS の input method service | winit が preedit / commit を `WindowEvent::Ime` として渡し、producer がそのイベントを記録できる |
| Windows | winit の native IME event と Windows IMM / Input Method service | 同じ入力スレッド上で Windows IMM の preedit / commit を受け取り、`GetKeyboardLayout` でそのスレッドの keyboard layout を照会したうえで、winit の IME event に変換され、producer が同じ transcript contract を生成できる |
| Linux | winit の native IME event と IBus または X11 input method service | IBus/X11 の preedit / commit が winit の IME event に変換され、producer が同じ transcript contract を生成できる |

これらは設計上の仮説であり、Windows / Linux の実機入力成功を意味しない。`native_event_observed` や `origin=os-native` は producer の出力宣言に過ぎず、artifact 自体を読む verifier の trust anchor にはならない。

## CI の境界

`.github/workflows/native-ime-contract.yml` は `macos-latest`、`ubuntu-latest`、`windows-latest` で次を実行する。

- core verifier を `--no-default-features --features native-ime-evidence` でテストする。
- egui の Unicode session contract を `--features egui,native-ime-evidence` でテストする。
- native producer の不正run拒否テストを実行する。
- native producer を `cargo check --locked` と `cargo clippy --all-targets --all-features --locked -- -D warnings` で検証する。
- Ubuntu では winit の compile に必要な Wayland / X11 headers と、Unicode render session 用の Noto CJK / Noto Color Emoji fonts を導入する。session は純粋な render contract であり、Xvfb や native IME 実測を要求しない。

workflow が upload するのは各 runner の compile report だけである。compile report は native IME evidence ではなく、実測 transcript や画像、入力結果を含めない。したがって、この job の成功は「三 OS で compile contract が成立した」ことだけを示す。

## Digest と証跡の信頼境界

native evidence artifact の `artifact_sha256` は payload の整合性と同一 run binding を確認するために使う。期待する digest は artifact と同じ untrusted channel から取得して trust してはならない。期待値は producer / runner の外部 attestation channel から供給し、core verifier の `attested_artifact_sha256` に渡す。

この repository では Windows / Linux の実測 artifact、外部 attestation、IME service の実機ログは未取得である。実機を利用できるまで、上記の OS 仮説と CI compile report を別々の証跡として保持する。

## Producer CLI

native producer の実行は次の形式で行う。`--revision`、`--challenge`、`--producer-id`、`--runner-id` は実行元が発行した値を渡し、`--output` は producer が書き込む artifact のパスにする。

```sh
cargo run --manifest-path crates/katana-ui-core-native-ime-evidence/Cargo.toml -- \
  --revision=REVISION --challenge=CHALLENGE --producer-id=PRODUCER \
  --runner-id=RUNNER --output=/tmp/kuc-native-ime.json
```

registry binary `kuc-native-ime-verify` は`0.4.1` 公開後に、次のように導入できる予定である。公開前の検証ではworkspace binaryを同じ引数で利用する。

```sh
cargo install katana-ui-core --version 0.4.1 \
  --features native-ime-evidence --bin kuc-native-ime-verify
kuc-native-ime-verify ARTIFACT.json TRUSTED_EXPECTATIONS.json
```

`TRUSTED_EXPECTATIONS.json` は artifact とは別の外部 attestation channel から作る。期待値 JSON の schema は次の通りで、`attested_artifact_sha256` は producer / runner の trusted attestation が発行した値を入れる。

```json
{
  "revision": "REVISION",
  "platform": "windows",
  "run_id": "RUN_ID",
  "challenge": "CHALLENGE",
  "producer_id": "PRODUCER",
  "runner_id": "RUNNER",
  "attested_artifact_sha256": "64_lowercase_hex_characters"
}
```

CLI は artifact と expectations の path が同一である場合も拒否する。artifact JSON や同じ untrusted upload channel の report から expectations を生成して実行してはならない。

## Consumer verification

consumer は artifact を `NativeImeEvidenceArtifact` へ deserialize し、期待値を現在の run binding と外部 attestation から組み立てて verifier に渡す。`attested_artifact_sha256` は artifact JSON、CI compile report、同じ upload channel の metadata から取得してはならない。自己計算した digest を期待値へ戻す実装も、self-attestation になるため禁止する。

```rust,no_run
use katana_ui_core::native_ime_evidence::{
    NativeImeEvidenceArtifact, NativeImeVerificationExpectations,
};

let artifact: NativeImeEvidenceArtifact = serde_json::from_slice(&artifact_bytes)?;
// trusted_digest は runner / producer 外部 attestation channel から取得する。
let trusted_digest = external_attestation.artifact_sha256.clone();
let expected = NativeImeVerificationExpectations {
    revision: external_attestation.revision.clone(),
    platform: external_attestation.platform.clone(),
    run_id: external_attestation.run_id.clone(),
    challenge: external_attestation.challenge.clone(),
    producer_id: external_attestation.producer_id.clone(),
    runner_id: external_attestation.runner_id.clone(),
    attested_artifact_sha256: trusted_digest,
};
artifact.verify(&expected)?;
```

期待する binding は artifact から読み戻さず、信頼した実行元から供給する。verifier は artifact 内の宣言だけで OS-native origin を証明しない。

## 配布バイナリ

v0.4.1 の GitHub Release には macOS / Windows / Linux の `kuc-native-ime-tools-*.zip` を添付する。producer、verifier、source revision / binary SHA-256 を含むmanifestを収録する。manifestの `native_ime_measured: false` は配布ビルドが実IMEの証跡でないことを示す。実行元は公開したbinary digestとrevisionを固定し、新しいchallengeを発行して実行する。

commit は最終描画textの UTF-8 byte range と照合する。hit-test は対象 scalar の text range、measurement 内の target bounds、query 座標を同時に検証する。

measurement は retained frame の描画surface寸法を使う。文字画像の寸法では末尾caretの領域を含められないため、caretとhit-testを実surfaceの座標で検証する。input-methodは最初の非空native preeditで照会して固定し、以後の非空preeditとcommitで一致を確認する。照会はウィンドウ生成前から計測したrunの残り時間に制限し、Linuxのibusとfcitxの待機も同じ期限を共有する。

公開workflowは三OSの配布ZIPが全て生成された後、同一revision/versionとbinary SHA-256を照合する。全assetをdraft Releaseへ添付してから公開し、crates.io publishへ進む。
