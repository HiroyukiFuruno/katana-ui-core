## MODIFIED Requirements

### Requirement: Runtime exposes Application, Window, and Surface neutral APIs

KUC runtime MUST expose `Application`, `Window`, `Surface` neutral APIs.
`WindowConfig` MUST cover title, size, min_size, max_size, icon, decorations, fullscreen.
`WindowCommand` MUST include `SetTitle`, `SetSize`, `SetPosition`, `Focus`, `Minimize`, `Maximize`, `Restore`, `Close`, `EnterFullscreen`, `ExitFullscreen`.
Adapter contract MUST cover window controls dispatch and draggable region transfer to support the `TitleBar` molecule.

#### Scenario: TitleBar dispatches EnterFullscreen

- **WHEN** a `TitleBar` control activates `EnterFullscreen`
- **THEN** the runtime sends `WindowCommand::EnterFullscreen` to the adapter
- **AND** the adapter switches to native fullscreen and reports back via `WindowEvent`

#### Scenario: adapter receives draggable regions

- **WHEN** a `TitleBar` is mounted with computed drag regions
- **THEN** the adapter receives the regions through the documented adapter contract method
- **AND** native drag-to-move works on those regions only
