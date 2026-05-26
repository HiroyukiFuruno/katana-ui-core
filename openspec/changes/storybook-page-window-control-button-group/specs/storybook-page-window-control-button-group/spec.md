## ADDED Requirements

### Requirement: window-control-button-group Storybook page satisfies the UI harness contract

`window-control-button-group` page MUST satisfy the Storybook UI harness contract as an independent leaf change.
The page MUST expose a dedicated preview, at least four meaningful presets, at least four Inspector options, state/action/event evidence or a passive UI contract, theme-token rendering, and automated tests.

#### Scenario: window-control-button-group page is tracked as a leaf Storybook change

- **WHEN** Storybook harness guard runs
- **THEN** it finds `window-control-button-group` in `requirements.rs` and Storybook menu
- **AND** it finds `storybook-page-window-control-button-group` leaf change

#### Scenario: window-control-button-group page does not pass by generic rendering alone

- **WHEN** `window-control-button-group` has only a generic renderer or label-only preset changes
- **THEN** the page is not considered ready
- **AND** the missing dedicated preview, preset, option, Inspector, state/action/event, or rendering contract is reported
