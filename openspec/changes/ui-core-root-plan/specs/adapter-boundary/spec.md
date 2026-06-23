## ADDED Requirements

### Requirement: framework implementations stay outside active core workspace

framework-specific UI 実装 MUST KUC active workspace に置いてはならない。
中核 crate（core crate）はそれらの framework crate に直接依存してはならない。

#### Scenario: framework-native code is needed

- **WHEN** KUC component を framework-native runtime / renderer で描画する必要がある
- **THEN** 実装は KUC active workspace の外側に置く
- **AND** `crates/katana-ui-core` は framework-native dependencies なしで build できる

#### Scenario: external runtime support is needed

- **WHEN** external runtime / renderer support を追加する
- **THEN** KUC active workspace は中立 DTO / trait / action / event / state contract だけを提供する
- **AND** 未対応挙動（unsupported behavior）は KUC active tree ではなく該当 external runtime / renderer 側で記録する

### Requirement: external runtimes consume the neutral model

external runtime / renderer MUST `UiTree`, `UiNode`, `ThemeSnapshot`, and `EventSink` を消費できる。
KUC core MUST 利用側（consumer）に framework-native values を作らせてから KUC core APIs へ入らせてはならない。

#### Scenario: external renderer renders a button

- **WHEN** external renderer が `UiTree` 内の中立 Button atom を受け取る
- **THEN** その node を framework-native button view へ mapping する
- **AND** イベント（event）は中立 event sink から返す

### Requirement: external runtime is not a core dependency

external runtime / renderer MUST KUC core の release gate とは独立して維持されなければならない。
KUC active workspace MUST core、Storybook、consumer app に限定する。

#### Scenario: external runtime changes

- **WHEN** external runtime / renderer の選定が変わる
- **THEN** KUC active release gate はその選定を参照しない
- **AND** 中核 API（core API）は変わらない

### Requirement: Storybook uses core output

Storybook MUST `katana-ui-core` の中核（core）model だけで KUC component catalog を検証する。
Storybook MUST framework-specific UI または external runtime / renderer を経由してはならない。

#### Scenario: Storybook page is added

- **WHEN** KUC component 向け Storybook page を実装する
- **THEN** `katana-ui-core` の `UiTree` / `UiNode` / state identity を検証する
- **AND** Storybook source は framework-specific UI / external runtime / renderer を import しない

### Requirement: external runtime failures do not block core release

Storybook smoke coverage は `katana-ui-core` の core-only 確認だけを必須とする。
external runtime / renderer の失敗 MUST NOT core API compatibility に影響しない限り KUC release を止めない。

#### Scenario: external runtime fails independently

- **WHEN** KUC core、consumer app contract、Storybook core gate が通り、external runtime / renderer だけが失敗する
- **THEN** KUC release は `docs/release.md` の core-only release policy で判断する
- **AND** failure は core breakage ではなく external runtime scope として報告する
