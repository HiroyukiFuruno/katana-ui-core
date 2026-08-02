# Design

`GenericGrid` owns a boolean visual preference because line visibility is a
generic two-dimensional grid concern. Conversion to `UiGridProps` copies the
value without interpreting its source. Both constructor and deserialization
defaults are `true`, preserving the `v0.2.0` rendering behavior. The public
render-props field addition is released as `v0.3.0` because external consumers
may construct `UiGridProps` with struct literals.

KDV maps its sheet artifact into this flag. KatanA reads only `UiGridProps` and
does not infer spreadsheet semantics.
