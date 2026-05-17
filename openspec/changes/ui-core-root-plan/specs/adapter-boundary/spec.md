## ADDED Requirements

### Requirement: framework implementations live outside core

Floem / GPUI / egui 実装 MUST 変換層 crate（adapter crate）または adapter-only module に置く。
中核 crate（core crate）はそれらの framework crate に直接依存してはならない。

#### Scenario: Floem code is needed

- **WHEN** KUC component を Floem で描画する必要がある
- **THEN** 実装は `katana-ui-core-floem` に置く
- **AND** `crates/katana-ui-core` は Floem dependencies なしで build できる

#### Scenario: 互換変換層が必要になる

- **WHEN** egui または GPUI support を追加する
- **THEN** `katana-ui-core-egui` または `katana-ui-core-gpui` 経由で追加する
- **AND** 未対応挙動（unsupported behavior）は該当 adapter crate の README、または `docs/compat-adapters.md` に記録する

### Requirement: adapters consume the neutral model

変換層 crate（adapter crate）MUST `UiTree`, `UiNode`, `ThemeSnapshot`, and `EventSink` を framework-native views or commands へ変換する。
変換層（adapter）MUST 利用側（consumer）に framework-native values を作らせてから KUC core APIs へ入らせてはならない。

#### Scenario: adapter renders a button

- **WHEN** 変換層（adapter）が `UiTree` 内の中立 Button atom を受け取る
- **THEN** その node を framework-native button view へ mapping する
- **AND** イベント（event）は中立 event sink から返す

### Requirement: Floem is a primary candidate, not a core dependency

Floem MUST 最初に実装する変換層（adapter）候補として扱う。ただし、P4-0 が主系変換層（primary adapter）を選ぶまでは、主系（primary）状態は差し替え可能でなければならない。
Floem が primary に選ばれない場合、互換変換層（compatibility adapter）として維持できなければならない。

#### Scenario: P4-0 selects another primary adapter

- **WHEN** P4-0 が GPUI、egui、または adapter agnostic mode を primary として選ぶ
- **THEN** `katana-ui-core-floem` は互換変換層（compatibility adapter）status へ降格する
- **AND** 中核 API（core API）は変わらない

### Requirement: Storybook uses core output

Storybook MUST `katana-ui-core` の中核（core）model だけで KUC component catalog を検証する。
Storybook MUST Floem / GPUI / egui または adapter crate を経由してはならない。

#### Scenario: Storybook page is added

- **WHEN** KUC component 向け Storybook page を実装する
- **THEN** `katana-ui-core` の `UiTree` / `UiNode` / state identity を検証する
- **AND** Storybook source は Floem / GPUI / egui / adapter crate を import しない

### Requirement: compatibility adapters have bounded release responsibility

互換変換層（compatibility adapter）MUST compile test を持つ。
Storybook smoke coverage は `katana-ui-core` の core-only 確認だけを必須とし、互換変換層には必須化しない。
互換変換層の失敗 MUST NOT core API compatibility または選択済み主系変換層（primary adapter）に影響しない限り primary release を止めない。

#### Scenario: compatibility adapter fails independently

- **WHEN** core と primary adapter が通り、egui または GPUI compatibility adapter だけが失敗する
- **THEN** 継続可否は `docs/release.md` と `docs/compat-adapters.md` の release policy で判断する
- **AND** failure は core breakage ではなく compatibility scope として報告する
