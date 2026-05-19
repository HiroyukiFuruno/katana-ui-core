## MODIFIED Requirements

### Requirement: Runtime exposes Application, Window, and Surface neutral APIs

KUC runtime MUST expose `Application`, `Window`, `Surface` neutral APIs.
`WindowConfig` MUST cover title, size, min_size, max_size, icon, decorations, fullscreen.
`WindowCommand` MUST include `SetTitle`, `SetSize`, `SetPosition`, `Focus`, `Minimize`, `Maximize`, `Restore`, and `Close`.
Adapter contract MUST cover window controls dispatch.
Draggable region transfer is adapter / consumer responsibility and MUST NOT be required by `WindowControlButtonGroup`.

#### Scenario: WindowControlButtonGroup emits window control intent

- **WHEN** a `WindowControlButtonGroup` control activates `Close`
- **THEN** KUC emits `ControlPressed { which: Close }`
- **AND** the consumer or adapter maps it to `WindowCommand::Close`

#### Scenario: draggable regions remain out of scope

- **WHEN** a consumer needs a draggable title area
- **THEN** the consumer or adapter defines draggable regions outside `WindowControlButtonGroup`
- **AND** KUC does not expose draggable region props in this change
