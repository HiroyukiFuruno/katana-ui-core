## ADDED Requirements

### Requirement: tabs Storybook page satisfies the UI harness contract

`tabs` page MUST satisfy the Storybook UI harness contract as an independent leaf change.
The page MUST expose a dedicated preview, at least four meaningful presets, at least four Inspector options, state/action/event evidence or a passive UI contract, theme-token rendering, and automated tests.

#### Scenario: tabs page is tracked as a leaf Storybook change

- **WHEN** Storybook harness guard runs
- **THEN** it finds `tabs` in `requirements.rs` and Storybook menu
- **AND** it finds `storybook-page-tabs` leaf change

#### Scenario: tabs page does not pass by generic rendering alone

- **WHEN** `tabs` has only a generic renderer or label-only preset changes
- **THEN** the page is not considered ready
- **AND** the missing dedicated preview, preset, option, Inspector, state/action/event, or rendering contract is reported

#### Scenario: tabs page mirrors Katana workspace tab lifecycle

- **WHEN** the `tabs` Storybook page is opened
- **THEN** it exposes add, close, move, group, pin, unpin, and overflow controls
- **AND** pinned tabs are rendered at the leading edge and are not closable until unpinned
- **AND** each lifecycle operation updates Storybook state/action/event evidence
