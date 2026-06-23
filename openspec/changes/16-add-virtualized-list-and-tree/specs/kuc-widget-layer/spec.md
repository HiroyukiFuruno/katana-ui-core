## MODIFIED Requirements

### Requirement: Molecules compose atoms without stealing state

KUC molecules MUST compose atoms and additional model state without moving child component state into an uncontrolled global store.
Molecule contracts MUST explicitly define options, actions, events, state, presets, tests, numeric layout / rendering contracts, preview behavior, settings behavior, and Storybook pages.
Molecules MUST preserve parent and child state identities separately in automated tests and Storybook logs.
`List`, `SelectionList`, `TreeView`, `CommandPalette`, and `DiagnosticsList` MUST accept a shared `VirtualizationConfig` option without sharing global state; each molecule keeps its own virtual range in its own state.

#### Scenario: shared virtualization config does not share state

- **WHEN** two molecules use the same `VirtualizationConfig`
- **THEN** each molecule's state ids and virtual ranges remain independent
- **AND** updating one does not implicitly update the other

#### Scenario: virtualization opt-in is uniform across molecules

- **WHEN** a contract test runs the virtualization scenario across all eligible molecules
- **THEN** each molecule passes the same set of visible-range and accessibility assertions
- **AND** failures point at the specific molecule rather than the shared engine
