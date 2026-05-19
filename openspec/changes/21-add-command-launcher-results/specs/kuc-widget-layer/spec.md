## ADDED Requirements

### Requirement: Command launcher remains a molecule

KUC は command launcher を atoms / molecules の組み合わせとして提供することを MUST（必須）とする。
画面全体の modal、app-level command registry、domain action 実行は提供しないことを MUST（必須）とする。

#### Scenario: consumer builds a modal palette

- **WHEN** consumer が `CommandPalette` molecule を modal 内に配置する
- **THEN** KUC は query、row、highlight、execute event を提供する
- **AND** modal の配置、overlay、command 実行は consumer が持つ

#### Scenario: consumer builds a slash launcher

- **WHEN** consumer が composer 直下に小型 launcher を配置する
- **THEN** 同じ result row と keyboard contract を使える
- **AND** chat session や slash command registry は KUC に入らない
