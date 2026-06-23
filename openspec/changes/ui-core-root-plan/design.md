## Context

`katana-ui-core` は framework-neutral core として、framework-native runtime / renderer を core dependency に持たない構成へ移行する。
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

- 中核 crate（core crate）から framework-specific UI の公開 API（public API）露出を消す。
- `runtime` / `window` / `surface` を KUC の標準 API として持つ。
- `UiTree` / `UiNode` / `ThemeSnapshot` / `UiEvent` を external runtime / renderer が消費できる中立契約として固定する。
- external runtime / renderer は KUC active workspace と release gate に含めない。
- Storybook は `katana-ui-core` の中核（core）model だけで検証し、リリース検査（release gate）では framework-native runtime / renderer 経由にしない。
- UI をゼロから確立する現段階では、KUC core、consumer app contract、Storybook core catalog を先に固める。
- JSX / TSX 互換ではなく、純 Rust の部品（component）合成 API として React に近い使い心地を目指す。
- component 構造と見た目設定（style）を分離し、CSS のように後から class / rule / declaration を差し替えられることを KUC 独自実装の強みにする。

**Non-Goals:**

- KatanA 本体、KDV、KLE、KCF の実装をこの repo から直接変更しない。
- external runtime / renderer の最終選定は KUC active tree では管理しない。KUC は framework-neutral core contract のみを管理する。
- external runtime / renderer の本実装をこの段階で完了扱いにしない。
- 既存画面部品（widget）変更単位（change）の履歴を一括 rewrite しない。
- platform menu、IME、drag & drop を KUC 標準 API に入れない。必要な場合は external runtime の逃がし口（escape hatch）とする。

## Decisions

### 1. Core は neutral DTO / trait だけを返す

KUC の公開 API（public API）は、Adapter View / Adapter Element / adapter Ui を返さない。
画面は `UiTree`、要素は `UiNode`、差分は `UiTreeDiff`、命令は `UiCommand`、テーマは `ThemeSnapshot` として扱う。

理由:

- core compile に framework dependency を引き込まないため。
- external runtime / renderer が変わっても、KUC の利用側 API を壊しにくくするため。

代替案:

- core が feature gate で Adapter View を返す案は不採用。feature の組み合わせで core の意味が変わり、依存漏れを検出しにくい。

### 2. runtime / window / surface は画面部品（widget）の外側ではなく KUC の責務にする

KUC は `Application::new().window(...).run()` のような入口、複数窓（multi-window）、全画面（fullscreen）、アイコン（icon）、描画面の寸法情報（surface metrics）を中立 API（neutral API）として持つ。

理由:

- root 計画では KUC を画面部品集ではなく UI Core として扱うため。
- KatanA 側が起動・窓・描画面の framework 詳細を知らない構成にするため。

### 3. framework-native runtime / renderer は active workspace に置かない

`framework runtime crate`、`framework renderer crate` は KUC active workspace に含めない。
Storybook は `katana-ui-core-storybook` として、中核（core）model だけを検証する。

理由:

- KUC core が framework-native runtime / renderer なしで compile できることが repository-level done の中心条件だから。
- external runtime / renderer を選んでも、中核（core）の依存方向は変えないため。

### 4. 既存 `primitive` / `composite` は段階移行にする

既存の `primitive` は `atom`、`composite` は `molecule` へ段階移行する。
一括削除ではなく、公開面を neutral model に寄せながら module 境界を整える。

理由:

- 既存画面部品（widget）の Storybook とテストを活かしながら移行できる。
- 破壊範囲を小さく保てる。

### 5. 品質ゲートは依存漏れを直接検査する

`just check` は、中核 crate（core crate）が framework-native runtime / renderer / `katana-*` domain crate を含まないことを検査する。
検査を通すための除外追加ではなく、依存方向を直す。

理由:

- UI 分離の目的は「検査が通ること」ではなく、core の責務を守ることだから。

### 6. KUC 継続の Go / No-Go 条件

KUC を独自 UI core として続ける条件は、純 Rust の部品（component）合成、component 内部 state、後付け見た目設定（style）の 3 点を中核 API として提供できることである。
これを満たせないなら、既存 UI framework を base にして必要な部品だけを作る方が合理的である。

## Migration Plan

1. workspace と crate 名を現状確認し、中核（core）/ Storybook / consumer app の配置を決める。
2. 中核 crate（core crate）に中立 module skeleton を作り、既存 module を段階移行できる入口を作る。
3. theme / layout / event / render model / accessibility / external runtime contract を先に固める。
4. runtime / window / surface を Noop runtime で動く形にする。
5. `examples/kuc-consumer-app` で public API だけを使う汎用 app shell を構築する。
6. Storybook を `crates/katana-ui-core-storybook` の中核（core）model 検証に切り替える。
7. dependency leak guard、release dry-run、docs を更新する。

## Risks / Trade-offs

- [Risk] 既存画面部品（widget）変更単位（change）が Adapter 前提で残る → [Mitigation] 履歴として残し、新規実装はこの親 change の境界を優先する。
- [Risk] runtime / window / surface が大きくなりすぎる → [Mitigation] 中立化（neutral）粒度は title / size / close / focus / fullscreen / multi-window / icon に限定する。
- [Risk] repo 外 runtime / renderer 未確定で品質ゲートがぶれる → [Mitigation] KUC active tree の release gate は core public API、Storybook core smoke、consumer smoke に限定する。
- [Risk] Storybook が framework dependency を再び引き込む → [Mitigation] Storybook は `katana-ui-core` だけを参照し、framework-specific UI を禁止する repo-local guard で検査する。

## Open Questions

- external runtime / renderer はどれにするか。これは KUC active tree では管理しない。
- platform menu / IME / drag & drop の逃がし口（escape hatch）は external runtime contract を入口にし、具体型は `PlatformMenuRequest`、`ImeRequest`、`DragDropRequest` とする。標準 API には入れず、拡張 contract として扱う。
