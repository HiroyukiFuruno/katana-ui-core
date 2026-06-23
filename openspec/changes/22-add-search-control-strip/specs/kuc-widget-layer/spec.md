## MODIFIED Requirements

### Requirement: Search input, search controls, and result launcher remain separate

KUC MUST keep simple query input, search operation controls, and result launching as separate public contracts.
`SearchBox` MUST remain the simple query input.
`SearchControlStrip` MUST own search options and navigation events.
`CommandPalette` / `CommandResultRow` MUST own result row selection and execution events.

#### Scenario: consumer builds workspace search

- **WHEN** a consumer builds a workspace search modal
- **THEN** it composes `SearchControlStrip` with `CommandResultRow` or a list-like molecule
- **AND** it keeps workspace search execution outside KUC

#### Scenario: consumer builds editor find

- **WHEN** KLE builds find / replace UI around its editor
- **THEN** it can use `SearchControlStrip`
- **AND** editor mutation remains in KLE or the host, not KUC
