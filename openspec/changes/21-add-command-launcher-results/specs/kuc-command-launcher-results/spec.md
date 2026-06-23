## ADDED Requirements

### Requirement: Command launcher exposes typed result rows

`CommandPalette` は、`id`、`label`、`secondary_label`、`icon`、`shortcut`、`provider_id`、`group_id`、`disabled`、`disabled_reason` を持つ typed result row を受け取れるようにすることを MUST（必須）とする。

#### Scenario: result row keeps visual metadata

- **WHEN** consumer が icon、secondary label、shortcut を持つ result row を渡す
- **THEN** KUC の render model はそれらを失わずに保持する
- **AND** row の実行内容そのものは KUC に入らない

### Requirement: Query and highlight state are observable

`CommandPalette` は query と highlighted row を state として公開し、query 変更と highlight 移動を event として返すことを MUST（必須）とする。

#### Scenario: query changes highlighted row

- **WHEN** `SetQuery("theme")` action が適用される
- **THEN** `QueryChanged("theme")` event が発火する
- **AND** highlighted row は有効な最初の候補へ移動する

#### Scenario: keyboard moves highlight

- **WHEN** Arrow Down / Arrow Up / Home / End が入力される
- **THEN** highlighted row が deterministic に移動する
- **AND** disabled row を highlight できるが execute はできない

### Requirement: Execution is emitted as consumer event

KUC は command を直接実行しないことを MUST（必須）とする。
選択された result row の id は `ResultExecuted` event として consumer へ返すことを MUST（必須）とする。

#### Scenario: highlighted row executes

- **WHEN** highlighted row が有効で、Enter が入力される
- **THEN** `ResultExecuted { id }` event が発火する
- **AND** consumer が domain action を実行する

#### Scenario: disabled row does not execute

- **WHEN** highlighted row が disabled で、Enter が入力される
- **THEN** `ResultExecuted` は発火しない
- **AND** `disabled_reason` は render model に残る

### Requirement: Virtualization keeps highlighted row reachable

大量結果で `VirtualizationConfig` が有効な場合でも、highlighted row は keyboard 操作で到達可能であることを MUST（必須）とする。

#### Scenario: highlighted row is outside current virtual range

- **WHEN** highlighted row が現在の virtual range 外に移動する
- **THEN** virtual range は highlighted row を含むように更新される
- **AND** accessibility の set size と position は総件数基準で保持される
