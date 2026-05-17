## ADDED Requirements

### Requirement: KUC is a framework-neutral UI Core

`katana-ui-core` MUST フレームワーク非依存（framework-neutral）な UI Core を提供し、Floem 専用の画面部品（widget）crate になってはならない。
中核 crate（core crate）は中立 DTO / trait ベースの API を公開し、公開 API（public API）で Floem View、GPUI Element、egui Ui、その他の framework-native view type を公開してはならない。

#### Scenario: core API is inspected

- **WHEN** `crates/katana-ui-core` 配下の公開 API を確認する
- **THEN** UI 構造、イベント（event）、テーマ（theme）、runtime、window、surface は KUC 所有の DTO / trait 型で受け渡しされる
- **AND** 関数シグネチャ（signature）に framework-native view type を要求しない

#### Scenario: core compiles without a framework dependency

- **WHEN** 中核 crate（core crate）を変換層 crate（adapter crate）なしで compile する
- **THEN** build は `floem`, `gpui`, or `egui` を link せずに成功する

### Requirement: KUC owns the root UI modules

`katana-ui-core` MUST root UI Core modules として `runtime`, `window`, `surface`, `atom`, `molecule`, `layout`, `theme`, `event`, `render_model`, `accessibility`, and `adapter_contract` を定義する。
既存の `primitive` と `composite` modules は、framework-specific public surfaces を残さず `atom` と `molecule` へ移行しなければならない。

#### Scenario: module skeleton is checked

- **WHEN** KUC の中核 crate（core crate）module tree を確認する
- **THEN** 各 root UI Core module が存在する、または明示的な移行 task を持つ
- **AND** 既存（legacy）の `primitive` / `composite` code は `atom` / `molecule` への移行経路を文書化している

### Requirement: KUC remains Katana domain-neutral

`katana-ui-core` MUST NOT `katana`, `katana-language-editor`, `katana-document-viewer`, `katana-markdown-model` のような Katana domain crate、または document / editor / forge 固有概念へ依存してはならない。

#### Scenario: domain dependency is introduced

- **WHEN** Katana domain crate への依存が中核 crate（core crate）へ追加される
- **THEN** 依存漏れ検査（dependency leak guard）が失敗する
- **AND** その統合は利用側 crate（consumer crate）または変換層境界（adapter boundary）へ移す

### Requirement: render model is the cross-adapter contract

KUC MUST `UiTree`, `UiNode`, `UiNodeId`, `UiNodeKind`, `UiProps`, `UiTreeDiff`, `UiCommand`, and `RenderContext` を変換層（adapter）と利用側（consumer）の中立描画モデル（neutral render model）として使う。
UI ごとの状態（state）MUST component 内部 model で管理する。
同じ種類・同じラベルの UI が複数ある場合でも、それぞれの `UiNodeId` と `UiStateId` MUST 一意でなければならない。
KUC MUST JSX / TSX 互換を目指さず、純 Rust の部品（component）合成 API として利用できなければならない。

#### Scenario: a widget is represented

- **WHEN** atom または molecule が中核 API（core API）で作成される
- **THEN** 中立 `UiTree` または `UiNode` として表現できる
- **AND** 変換層 crate（adapter crate）は framework-specific core APIs を呼ばずにそのモデル（model）を消費できる

#### Scenario: duplicate buttons keep separate state

- **WHEN** 同じ label の Button atom を同じ tree に複数作成する
- **THEN** 各 Button は異なる `UiNodeId` と `UiStateId` を持つ
- **AND** 変換層（adapter）は外部 state store を要求しない

#### Scenario: pure Rust component composition is used

- **WHEN** consumer が KUC の atom、molecule、layout、panel を Rust の builder API で組み合わせる
- **THEN** JSX / TSX / React runtime を使わずに `UiTree` を作成できる
- **AND** style は component 構造と分離された後付け解決として扱われる

### Requirement: theme and accessibility are first-class

KUC MUST color、font、spacing、radius、shadow、border、z-index の theme token と `ThemeSnapshot` を公開する。
KUC MUST accessibility DTOs を中核モデル（core model）に含め、変換層（adapter）が別メタデータ（metadata）を作らず accessibility を描画できるようにする。

#### Scenario: theme is applied through an adapter

- **WHEN** 変換層（adapter）が KUC UI tree を描画する
- **THEN** `ThemeSnapshot` から theme values を受け取る
- **AND** 中立モデル（neutral model）から accessibility metadata を読める

### Requirement: CSS-like style can be changed after component construction

KUC MUST component 構造、component 内部状態（state）、見た目設定（style）を分離する。
見た目設定（style）は CSS のように class / rule / declaration として後付けで解決でき、同じ `UiTree` に別の `StyleSheet` を適用して見た目を変えられなければならない。
この性質を満たせない場合、独自 UI core として継続せず GPUI など既存 UI framework を base にする方が合理的である。

#### Scenario: style sheet is replaced after UI creation

- **WHEN** consumer が同じ `UiTree` に異なる `StyleSheet` を適用する
- **THEN** `UiNodeId` と `UiStateId` は維持される
- **AND** background、foreground、border、spacing、radius などの resolved style は `StyleSheet` に応じて変わる
- **AND** component の内部 state を style 変更のために外側へ移さない

### Requirement: Panel carries explicit theme configuration

KUC の表示枠（panel）MUST `ThemeSnapshot` を受け取り、描画モデル（render model）へ見た目テーマ（theme）の識別子を渡す。
Storybook の左ナビ表示枠と右プレビュー表示枠も、暗黙の既定値ではなく明示された見た目テーマ（theme）で構成しなければならない。

#### Scenario: panel theme is configured

- **WHEN** consumer が KUC の `Panel` を作成する
- **THEN** `Panel` は `ThemeSnapshot` を受け取る
- **AND** 生成される `UiNode` は空でない theme id を持つ

#### Scenario: Storybook uses themed panels

- **WHEN** Storybook が component catalog を表示する
- **THEN** root、navigation、preview の各表示枠（panel）は `katana-ui-core` の `Panel` で表現される
- **AND** 各表示枠（panel）は同じ `ThemeSnapshot` から theme id を受け取る
