## Context

`katana-ui-core` は現在、crate 名と repository 名は KUC に寄っているが、実装はまだ Floem を core dependency として持つ。
既存 OpenSpec 変更単位（OpenSpec change）も、画面部品（widget）単位の抽出計画が中心で、root 側の UI 分離計画とは粒度が違う。

この change は親設計の正本である。01〜24 の atoms / molecules と部品カタログの実装正本は `establish-kuc-atoms-molecules-catalog` へ分ける。
そのため、この change の完了は 01〜24 の部品実装完了を意味しない。

repo 内にコピーした root 計画では、KUC は「KatanA 専用の画面部品（widget）集」ではなく、起動・窓・描画面・イベント・描画モデル・テーマ・画面部品を持つフレームワーク非依存（framework-neutral）な UI Core と定義されている。
この change は、その方針を KUC repo の OpenSpec として実装可能な単位に固定する。

参照元:

- `docs/architecture/ui-separation/root-plan-source.md`
- `docs/adr/0002-katana-ui-core-rename.md`
- `docs/ui-separation-plan.md`

## Goals / Non-Goals

**Goals:**

- 中核 crate（core crate）から Floem / GPUI / egui の公開 API（public API）露出を消す。
- `runtime` / `window` / `surface` を KUC の標準 API として持つ。
- `UiTree` / `UiNode` / `ThemeSnapshot` / `UiEvent` を変換層（adapter）の入力契約として固定する。
- Floem は最初に整備する主系変換層（primary adapter）候補として扱うが、中核依存（core dependency）にはしない。
- 互換変換層（compatibility adapter）として egui / gpui の入口を持てる構造にする。
- Storybook は `katana-ui-core` の中核（core）model だけで検証し、リリース検査（release gate）では adapter 経由にしない。
- UI をゼロから確立する現段階では、KUC core と primary adapter 候補を先に固める。egui / GPUI 互換 crate は本実装ではなく、skeleton / compile gate / support policy までをこの change の範囲で整備する。
- JSX / TSX 互換ではなく、純 Rust の部品（component）合成 API として React に近い使い心地を目指す。
- component 構造と見た目設定（style）を分離し、CSS のように後から class / rule / declaration を差し替えられることを KUC 独自実装の強みにする。

**Non-Goals:**

- KatanA 本体、KDV、KLE、KCF の実装をこの repo から直接変更しない。
- 主系変換層（primary adapter）の最終選定をこの change だけで確定しない。選定は P4-0-001 で作成する `docs/adr/katana-ui-primary-adapter.md` に従う。
- egui / GPUI 互換 adapter の本実装をこの段階で完了扱いにしない。
- 既存画面部品（widget）変更単位（change）の履歴を一括 rewrite しない。
- platform menu、IME、drag & drop を KUC 標準 API に入れない。必要な場合は変換層（adapter）の逃がし口（escape hatch）とする。

## Decisions

### 1. Core は neutral DTO / trait だけを返す

KUC の公開 API（public API）は、Floem View / GPUI Element / egui Ui を返さない。
画面は `UiTree`、要素は `UiNode`、差分は `UiTreeDiff`、命令は `UiCommand`、テーマは `ThemeSnapshot` として扱う。

理由:

- core compile に framework dependency を引き込まないため。
- 主系変換層（primary adapter）が変わっても、KUC の利用側 API を壊しにくくするため。

代替案:

- core が feature gate で Floem View を返す案は不採用。feature の組み合わせで core の意味が変わり、依存漏れを検出しにくい。

### 2. runtime / window / surface は画面部品（widget）の外側ではなく KUC の責務にする

KUC は `Application::new().window(...).run()` のような入口、複数窓（multi-window）、全画面（fullscreen）、アイコン（icon）、描画面の寸法情報（surface metrics）を中立 API（neutral API）として持つ。

理由:

- root 計画では KUC を画面部品集ではなく UI Core として扱うため。
- KatanA 側が起動・窓・描画面の framework 詳細を知らない構成にするため。

### 3. Floem は変換層 crate（adapter crate）に移す

`floem`、`floem_reactive`、`floem_renderer` は `crates/katana-ui-core-floem` 側へ移す。
Storybook は `katana-ui-core-storybook` として、中核（core）model だけを検証する。

理由:

- KUC core が Floem なしで compile できることが repository-level done の中心条件だから。
- Floem を最初の変換層（adapter）候補にしても、中核（core）の依存方向は変えないため。

### 4. 既存 `primitive` / `composite` は段階移行にする

既存の `primitive` は `atom`、`composite` は `molecule` へ段階移行する。
一括削除ではなく、公開面を neutral model に寄せながら module 境界を整える。

理由:

- 既存画面部品（widget）の Storybook とテストを活かしながら移行できる。
- 破壊範囲を小さく保てる。

### 5. 品質ゲートは依存漏れを直接検査する

`just check` は、中核 crate（core crate）が `floem` / `gpui` / `egui` / `katana-*` domain crate を含まないことを検査する。
検査を通すための除外追加ではなく、依存方向を直す。

理由:

- UI 分離の目的は「検査が通ること」ではなく、core の責務を守ることだから。

### 6. KUC 継続の Go / No-Go 条件

KUC を独自 UI core として続ける条件は、純 Rust の部品（component）合成、component 内部 state、後付け見た目設定（style）の 3 点を中核 API として提供できることである。
これを満たせないなら、GPUI など既存 UI framework を base にして必要な部品だけを作る方が合理的である。

## Migration Plan

1. workspace と crate 名を現状確認し、中核（core）/ 変換層（adapter）/ Storybook の配置を決める。
2. 中核 crate（core crate）に中立 module skeleton を作り、既存 module を段階移行できる入口を作る。
3. theme / layout / event / render model / accessibility / adapter contract を先に固める。
4. runtime / window / surface を無処理の変換層（Noop adapter）で動く形にする。
5. Floem 変換層 crate（adapter crate）を作り、既存 Floem 実装と overlay lifecycle を移す。
6. egui / gpui 互換変換層（compatibility adapter）の skeleton と品質ゲートを追加する。
7. Storybook を `crates/katana-ui-core-storybook` の中核（core）model 検証に切り替える。
8. dependency leak guard、release dry-run、docs を更新する。

## Risks / Trade-offs

- [Risk] 既存画面部品（widget）変更単位（change）が Floem 前提で残る → [Mitigation] 履歴として残し、新規実装はこの親 change の境界を優先する。
- [Risk] runtime / window / surface が大きくなりすぎる → [Mitigation] 中立化（neutral）粒度は title / size / close / focus / fullscreen / multi-window / icon に限定する。
- [Risk] 主系変換層（primary adapter）未確定で品質ゲートがぶれる → [Mitigation] Floem は主系（primary）候補として整備し、`docs/adr/katana-ui-primary-adapter.md` で確定するまでは互換変換層（compatibility adapter）降格可能な扱いにする。
- [Risk] Storybook が framework dependency を再び引き込む → [Mitigation] Storybook は `katana-ui-core` だけを参照し、Floem / GPUI / egui を禁止する repo-local guard で検査する。

## Open Questions

- 主系変換層（primary adapter）は Floem / GPUI / egui / adapter agnostic のどれにするか。これは P4-0-001 の `docs/adr/katana-ui-primary-adapter.md` で決める。
- 互換変換層（compatibility adapter）の release がどこまで primary release を止めるか。最低ラインは compile test とし、Storybook は `katana-ui-core` core-only 確認だけを必須にする。KUC 側の `docs/release.md` と `docs/compat-adapters.md` に記録する。
- platform menu / IME / drag & drop の逃がし口（escape hatch）は `AdapterExtension` を入口にし、具体型は `PlatformMenuRequest`、`ImeRequest`、`DragDropRequest` とする。標準 API には入れず、`adapter_contract` 拡張として扱う。
