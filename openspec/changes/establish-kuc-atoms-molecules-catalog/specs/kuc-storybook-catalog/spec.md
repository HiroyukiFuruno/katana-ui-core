## ADDED Requirements

### Requirement: Storybook はフィードバック用の実画面である

KUC Storybook MUST be a screen where each UI's option、action、event、state、preset、preview、settings can be operated and inspected.
KUC Storybook MUST NOT use static lists or screenshot storage as completion evidence.
Each UI page MUST show that preview body、state summary、event history、action history change after settings or preview operation.
Storybook MUST NOT make users responsible for verification.
Repository tests and quality gates MUST verify correctness.

#### Scenario: UI page を触る

- **WHEN** 利用者が Storybook で UI page を開く
- **THEN** option、action、event、state、preset、preview、settings を同じ画面で扱える
- **AND** 操作後に preview 本体と履歴が更新される
- **AND** 利用者の役割はフィードバックであり、正しさの検証ではない

### Requirement: Panel と scrollbar は独立した操作対象である

Navigation、Preview、Details、TreeView preview MUST each own independent scroll state and scrollbar model.
The scrollbar model MUST include visibility、track bounds、thumb bounds、offset、drag state.
Parent panel and child panel scroll state MUST NOT be mixed.

#### Scenario: Panel scrollbar を操作する

- **WHEN** Preview panel の scrollbar thumb を drag する
- **THEN** Preview panel の offset だけが変わる
- **AND** Navigation と Details の offset は変わらない

### Requirement: v0.1.0 app UI DoD

v0.1.0 MUST use the ability for `katana` and `katana-chat-ui` to build app UI only with `katana-ui-core` atoms、molecules、panel、event、state、layout contracts as the DoD.
Storybook MUST be treated as the interactive feedback surface.
Completion judgment MUST use automated tests、numeric layout/rendering contracts、input regression、guards as primary evidence.
Users MUST NOT be expected to verify release correctness in Storybook.

#### Scenario: 利用側 UI の構築条件を確認する

- **WHEN** v0.1.0 の完了判定を行う
- **THEN** `katana` と `katana-chat-ui` が必要とする UI を KUC 公開契約だけで組み立てられることを確認する
- **AND** Storybook だけを完了根拠にしない

### Requirement: Storybook is an interactive feedback surface

KUC Storybook MUST be an interactive surface for users and developers to try KUC components and provide feedback.
KUC Storybook MUST NOT be a static sample gallery that only lists or screenshots components.
It MUST NOT be the sole proof that components are correct.
Each selected UI page MUST let developers inspect layout, option, action, event, state, rendering changes, and panel-local scroll behavior.
Automated tests MUST remain the verification authority.

#### Scenario: Storybook role is inspected

- **WHEN** developers read the Storybook documentation
- **THEN** Storybook is described as an interactive feedback surface
- **AND** automated tests and guards are described as the quality gate
- **AND** the documentation rejects static all-list pages as completion evidence

### Requirement: Storybook navigation uses KUC TreeView

The Storybook left pane MUST use KUC TreeView to show provided components.
The tree MUST group entries by atomic design category and nested component category.
The tree MUST include atoms, molecules, and Storybook-internal sections.
Selecting a TreeView item MUST update preview, settings, state summary, event history, and action history context.
The central content MUST focus on the selected component detail and MUST NOT render an all-components card grid on every page.
The navigation TreeView MUST expose settings for vertical line visibility/style/width, directory and file icons, font role, theme, empty-area context menu, default open state, and directory toggle trigger area.
The navigation TreeView MUST support category open/close state, selected-row rendering, and aligned disclosure icon, file/folder icon, and row text within the same row box.

#### Scenario: component list is displayed

- **WHEN** Storybook opens
- **THEN** the left pane displays atoms and molecules in a nested TreeView
- **AND** selecting a TreeView item changes the preview page
- **AND** clicking a category row opens or closes its child rows
- **AND** the center panel shows selected component details instead of a repeated all-components grid

#### Scenario: TreeView row alignment is inspected

- **WHEN** Storybook renders a navigation row
- **THEN** the disclosure icon, directory/file icon, and label text share the same vertical center
- **AND** the click target matches the visible row bounds

#### Scenario: navigation tree options are changed

- **WHEN** a user changes TreeView settings on the Storybook page
- **THEN** the navigation preview reflects line, icon, font, theme, context menu, default open, and trigger-area options
- **AND** action and event history records the changed option

### Requirement: Component pages include preview and settings

Every component page MUST include a preview area and a settings area.
Settings MUST allow option values to be changed on screen and reflected in preview.
Every component page MUST also expose state summary, event history, action history, and requirement status.
Every component page MUST show a readable component contract for option, action, event, state, preset, preview, settings, test, and visual coverage.
Placeholder text-only pages MUST fail the Storybook gate.
Storybook MUST NOT treat a visual-only drawing as a completed widget page.
When a widget is drawn as an interactive control, the rendered hit target MUST be clickable in the Storybook window and MUST update action, event, state, and rendering evidence.
Changing settings MUST update typed options, component state, Inspector rows, and the selected preview body.
Changing only labels, chips, or log text without a preview-body rendering change MUST NOT count as interaction evidence.

#### Scenario: option value is changed

- **WHEN** a user changes an option in settings
- **THEN** the preview updates to reflect the option
- **AND** the state and action logs record the change
- **AND** the selected preview body changes when the option affects rendering

#### Scenario: rendered control is operated

- **WHEN** a user clicks a rendered Button or another selected component action target
- **THEN** Storybook records the screen action and emitted event
- **AND** Inspector state and preview rendering change from the previous frame

### Requirement: Storybook panels have independent vertical scroll

The Storybook Navigation, Preview, and Details panels MUST each own vertical scroll state.
Scrolling one panel MUST NOT mutate the scroll state of another panel.
The Storybook runtime MUST expose panel-local scroll state in reports or tests so regressions can be detected.

#### Scenario: panel scroll is changed

- **WHEN** the user scrolls the Preview panel
- **THEN** the Preview visible range changes
- **AND** Navigation and Details visible ranges remain unchanged

#### Scenario: nested TreeView preview scroll is changed

- **WHEN** the TreeView component preview scrolls its own node list
- **THEN** the TreeView preview scroll thumb changes
- **AND** the Storybook parent panel scroll state remains unchanged

### Requirement: Presets use KUC Tabs

Every component page MUST provide multiple presets where meaningful.
Preset switching MUST use KUC Tabs.
Preset switching MUST update preview, settings initial values, state summary, and current preset display in the action and event history.
Preset tabs MUST have measured width, height, gap, and active-state indicator values so numeric rendering contracts can detect degraded tab styling.
Preset tabs MUST follow the Katana app tab direction: no visual gaps, equal active/inactive height, and a selected bottom-edge accent.
The active preset tab MUST NOT grow larger than inactive tabs and MUST NOT look like an isolated button in a button row.
Preset state MUST be index-based so every visible tab can become selected.
The selected tab highlight MUST be shown on the bottom edge.
The Storybook viewport MUST support vertical scrolling for content that exceeds the visible window.
The Storybook viewport MUST expose scrollbar visibility as state so visible and hidden scrollbar modes can be checked.

#### Scenario: preset is switched

- **WHEN** a user switches a component preset tab
- **THEN** the preview and settings reflect the selected preset
- **AND** the selected preset is visible in the state summary

#### Scenario: preset tab layout is measured

- **WHEN** Storybook layout/rendering contract tests run
- **THEN** tab bounds, zero-gap spacing, equal active/inactive height, active bottom indicator pixels, and scroll viewport differences are checked by automated tests
- **AND** an unstyled, overlapping, or button-like tab strip fails the gate

#### Scenario: scrollbar visibility is changed

- **WHEN** the scrollbar visibility option is toggled
- **THEN** the viewport scrollbar appears or disappears from the rendered Storybook panel
- **AND** the state is available to automated contract tests

#### Scenario: later preset tabs are selected

- **WHEN** a user clicks the third or fourth preset tab
- **THEN** that preset becomes the selected tab
- **AND** the preview state changes from the previous preset

### Requirement: Storybook window supports desktop resizing

The Storybook main window MUST be resizable so macOS zoom/maximize can be used for manual inspection.
Modal windows MAY keep fixed sizing when their placement behavior is under test.

#### Scenario: main window is opened

- **WHEN** Storybook opens its main window
- **THEN** the window options allow resize and macOS zoom/maximize operation
- **AND** this behavior is covered by an automated contract test

### Requirement: Storybook exposes state, event, and action history

Storybook MUST show component state, event history, and action history for interactive components.
Settings changes MUST be recorded as actions; hidden state mutation without an action record MUST NOT be used for Storybook interaction.

#### Scenario: component is operated

- **WHEN** a user clicks, types, opens, closes, selects, drags, or changes a component
- **THEN** Storybook records the action, emitted event, target state id, and before/after summary
- **AND** the record is visible on the component page

### Requirement: Storybook may use internal organisms

Storybook MUST keep any larger internal structures, such as shell, navigation tree, preview workspace, and settings inspector, inside the Storybook implementation boundary.
These internal organisms MUST NOT become public widget API in this change.

#### Scenario: internal shell is implemented

- **WHEN** Storybook needs a larger structure for navigation or inspection
- **THEN** it can implement an internal organism
- **AND** public KUC widget scope remains atoms and molecules
