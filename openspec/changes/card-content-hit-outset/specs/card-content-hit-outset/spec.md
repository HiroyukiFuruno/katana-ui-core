## ADDED Requirements

### Requirement: Independent node hit outset
KUC SHALL expose a nonnegative `UiNode::hit_outset_left(u16)` builder, defaulting to zero. It SHALL expand the same node identity's node and node-action hit rectangles to the left, before viewport and scroll clipping, without changing drawing, text wrapping, layout measurement or scroll measurement. Expansion SHALL saturate at coordinate zero and SHALL NOT propagate to descendants or inline link spans.

#### Scenario: Card body keeps a full hit width
- **WHEN** a Card in a 560px viewport has child_indent 16 and its text body has hit_outset_left 16
- **THEN** the body draws at the existing indented origin and the same stable node has a 560px viewport hit width
- **AND** pixel output, measured body height and following-node position are unchanged

#### Scenario: Scroll clipping remains enforced
- **WHEN** an expanded body is inside a scrolling viewport
- **THEN** drawing and scroll measurement remain unchanged and node/action hits remain clipped to the viewport

#### Scenario: Legacy input
- **WHEN** serialized UiNode JSON omits hit_outset_left
- **THEN** it deserializes as zero and preserves existing hit geometry
