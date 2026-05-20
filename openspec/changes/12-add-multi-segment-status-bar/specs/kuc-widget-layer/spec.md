## MODIFIED Requirements

### Requirement: Molecules compose atoms without stealing state

KUC molecules MUST compose atoms and additional model state without moving child component state into an uncontrolled global store.
Molecule contracts MUST explicitly define options, actions, events, state, presets, automated tests, numeric layout/rendering contracts, preview behavior, settings behavior, and Storybook pages.
Molecules MUST preserve parent and child state identities separately in automated tests and Storybook logs.
`StatusBar` MUST support both `SingleMessage` (severity + message + actions + dismiss) and `MultiSegment` (leading/center/trailing segments with optional popovers and progress overlays) within one molecule via a typed `mode` option. The same molecule MUST NOT host both modes simultaneously.

#### Scenario: status bar exposes multi-segment when needed

- **WHEN** a consumer needs multiple segments (e.g., file info, lint summary, encoding/line/column)
- **THEN** the consumer sets `mode = MultiSegment` and provides `segments`
- **AND** the consumer does not implement a parallel status bar elsewhere

#### Scenario: status bar exposes single message when sufficient

- **WHEN** a consumer only needs one severity message
- **THEN** the consumer keeps `mode = SingleMessage` (default)
- **AND** existing presets remain valid without code changes
