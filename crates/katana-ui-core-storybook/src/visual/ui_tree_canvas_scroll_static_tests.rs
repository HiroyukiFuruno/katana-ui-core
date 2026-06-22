#[test]
fn scroll_area_offset_renderer_does_not_allocate_offset_sized_canvas() {
    let source = include_str!("ui_tree_canvas_scroll.rs");

    assert!(
        !source.contains("Canvas::new(viewport.width, content_height"),
        "scroll rendering must not allocate a backing canvas proportional to scroll offset"
    );
    assert!(
        !source.contains("Canvas::new(area.width, source_y")
            && !source.contains("Canvas::new(area.width, source_y.saturating_add"),
        "partial node rendering must not allocate a temp canvas proportional to scroll offset"
    );
}

#[test]
fn scroll_area_zero_offset_uses_same_virtualized_path() {
    let source = include_str!("ui_tree_canvas_scroll.rs");

    assert!(
        !source.contains("if scroll_y > 0.0"),
        "zero-offset scroll area must not bypass the virtualized visible-node renderer"
    );
}

#[test]
fn scroll_renderer_uses_shared_canvas_metrics() {
    let source = include_str!("ui_tree_canvas_scroll.rs");

    for forbidden in ["const TEXT_HEIGHT", "const NODE_GAP", "fn dimension_px"] {
        assert!(
            !source.contains(forbidden),
            "scroll renderer must use ui_tree_canvas_hit_metrics instead of redefining {forbidden}"
        );
    }
}

#[test]
fn scroll_renderer_caches_node_height_measurements() {
    let source = include_str!("ui_tree_canvas_scroll.rs");

    assert!(
        !source.contains("let mut height_cache = MeasuredNodeHeightCache::default()"),
        "scroll rendering must not recreate node-height cache for every scroll frame"
    );
    assert!(
        source.contains("measured_scroll_node_height("),
        "scroll rendering must reuse renderer-scoped measured node heights instead of remeasuring each node"
    );
}

#[test]
fn overlay_stack_partial_render_keeps_child_area_viewport_sized() {
    let source = include_str!("ui_tree_canvas_renderer_methods.rs");

    assert!(
        source.contains("visible_height = visible_bottom.saturating_sub(visible_top)"),
        "MediaFrame Stack rendering must derive a visible band height for partial redraws"
    );
    assert!(
        !source.contains("height: frame_height,\n                scroll_y: area.scroll_y"),
        "MediaFrame Stack children must not receive the full media height during partial redraw"
    );
}
