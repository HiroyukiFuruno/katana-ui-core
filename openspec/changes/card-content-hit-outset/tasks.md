## Implementation

- [x] Confirm master clean, stash zero and existing worktrees; create release/kuc-card-content-offset.
- [x] Add public hit outset and apply before clipping in core and Storybook.
- [x] Verify public API serialization, Card default compatibility, drawing/layout and scroll/hit contracts: raster-host integration tests 8 passed; no-default-features serialization test 1 passed.
- [x] Verify formatting and strict lint: just fmt-check; cargo clippy -p katana-ui-core --no-default-features --features raster-host --test card_content_hit_contract --lib --locked -- -D warnings passed.
- [ ] Parent release workflow: review, version, full release checks, Draft PR, review/fixes, Ready, CI, merge and public release.
