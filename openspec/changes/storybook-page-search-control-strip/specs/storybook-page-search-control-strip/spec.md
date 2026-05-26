## ADDED Requirements

### Requirement: search-control-strip Storybook page satisfies the UI harness contract

`search-control-strip` page MUST satisfy the Storybook UI harness contract as an independent leaf change.
The page MUST expose a dedicated preview, at least four meaningful presets, at least four Inspector options, state/action/event evidence or a passive UI contract, theme-token rendering, and automated tests.

#### Scenario: search-control-strip page is tracked as a leaf Storybook change

- **WHEN** Storybook harness guard runs
- **THEN** it finds `search-control-strip` in `requirements.rs` and Storybook menu
- **AND** it finds `storybook-page-search-control-strip` leaf change

#### Scenario: search-control-strip page does not pass by generic rendering alone

- **WHEN** `search-control-strip` has only a generic renderer or label-only preset changes
- **THEN** the page is not considered ready
- **AND** the missing dedicated preview, preset, option, Inspector, state/action/event, or rendering contract is reported
