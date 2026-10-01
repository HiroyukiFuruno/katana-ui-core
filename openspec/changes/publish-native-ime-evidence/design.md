# Native evidence の信頼境界

## 既存実装からの分離

`unicode_evidence/capture.rs` の capture は regression 用として維持する。そこから得た artifact に native フラグを追加して実入力証跡へ昇格させない。

`eframe::App::raw_input_hook` と `egui::Context::input` は egui への入力列を観測できるが、OS IME と synthetic RawInput を区別する根拠にはならない。producer が所有する native event adapter において `winit::WindowEvent::Ime` を観測し、変換・root 処理・描画へ渡した順序を記録する。winit event enum 自体も生成可能なので、信頼対象は任意の caller の申告ではなく、revision と実行 provenance が固定された producer / runner とする。

## Producer / verifier の責務

- producer は fresh run identity を生成し、platform と実際に選択された input method を取得する。fixture、過去 artifact、任意 caller 提供の native フラグを入力として受け取らない。
- native event transcript は non-empty preedit と日本語 commit を順序付きで記録する。空 preedit は backend の composition clear event になりうるため、実入力の証拠に数えない。
- glyph・計測・caret・hit test・AccessKit・root evidence は producer の同一 root/frame から取得する。過去 capture や別 root の証跡を結合しない。
- font は実際に読み込んだ file bytes の SHA-256 と catalog fingerprint を含める。
- verifier は期待 revision、platform、run identity / challenge、trusted producer / runner provenance を外部の期待条件として受け取り、artifact 自身の自己申告だけで照合しない。
- hash は完全性と結合の根拠であり、OS 入力元の証明とは区別する。native provenance が欠けた artifact は hash が正しくても拒否する。
- canonical schema で全 required field、scalar、RGBA crop signature、measurement、caret、hit test、AccessKit、root/frame/receipt の結合を検証する。

## GUI dependency boundary

default core / raster-host の依存に eframe / winit を追加しない。native runner の配置は、registry-only verifier の公開方法と合わせて決める。非公開 Storybook binary のみを追加して Issue の consumer 条件を満たした扱いにしない。

## Acceptance

三OSで同一 revision の producer を起動し、それぞれ OS IME で preedit / commit を発生させる。実行環境ごとの入力方法の選択は、native event trace が取得できることを確認してから固定する。単なる screenshot、synthetic test の成功、CI の三OS compile 成功を代用しない。

## 実環境なしでの実装継続（2026-10-02）

Windows / Linux の実環境は用意できないという最新ユーザー指示に従い、公式 OS / winit API に基づく仮説で adapter と verifier を実装する。接続情報待ちを実装開始の条件にしない。三OS compile と契約回帰は実装の検証に使い、実IME artifact 取得とは区別する。macOS は利用可能な環境で実測を試す。

信頼の入口は producer が出力した canonical artifact bytes の digest を trusted runner が外部で固定する方式とする。verifier へ渡す期待 digest は artifact と同じ非信頼ファイルから取得しない。digest は native event を単独で証明せず、信頼する producer 実行とその出力を結合する。
