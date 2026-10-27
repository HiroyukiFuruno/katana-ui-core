## ADDED Requirements

### Requirement: 文書状態から独立したglobal action
KUC SHALL provide public retained-root configuration for host-defined opaque action identity, menu label, shortcut, global scope, and enabled state. KUC SHALL own widgets, focus, shortcut detection, rasterization, and hit testing.

#### Scenario: 文書なしと通常とread-onlyのroot
- **WHEN** host registers an enabled action with primary+Shift+F and invokes its menu entry or shortcut in a root with no document, a normal document, or a read-only document
- **THEN** each route emits the same opaque action identity through authenticated root event transport exactly once

### Requirement: キー競合と配送認証
KUC SHALL preserve document primary+F search, document its shortcut behavior during IME and competing input focus, reject disabled actions, stale frames, events from another root, and repeated forwarding.

#### Scenario: 不正または競合する入力
- **WHEN** a shortcut conflicts with IME or input focus, an action is disabled, or its frame/root/forward receipt is invalid
- **THEN** no host effect occurs and existing input behavior remains intact

#### Scenario: モーダルを同じglobal actionで閉じる
- **WHEN** a host opens a selection dialog after the global action and the configured shortcut is pressed again while its query has focus
- **THEN** the default focus policy emits the same action once so the host can close the dialog without duplicating KUC key handling

#### Scenario: 入力優先の明示policyと継続IME
- **WHEN** a host selects the document-only focus policy and another input is focused, or an IME composition remains active or commits in the current frame
- **THEN** the shortcut does not emit a global action and the input retains its existing behavior

#### Scenario: 同frameのfocus移動を優先する
- **WHEN** the document-only policy receives a primary pointer press and the global shortcut in the same frame, in either focus direction
- **THEN** the shortcut is suppressed for that frame and the next frame uses the committed focus; the default Any policy is unaffected
