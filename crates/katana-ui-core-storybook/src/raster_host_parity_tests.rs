use crate::{StoryCatalog, raster_host as wrapper};
use katana_ui_core::{
    raster_host as core,
    render_model::{UiNode, UiNodeKind},
    theme::ThemeSnapshot,
};
use std::any::TypeId;

const RENDER_WIDTH: usize = 240;
const RENDER_HEIGHT: usize = 96;
const BACKGROUND: u32 = 0xff_ff_ff;
const CATALOG_RENDER_WIDTH: usize = 1280;
const CATALOG_RENDER_HEIGHT: usize = 960;
const CATALOG_PIXEL_COUNT: usize = CATALOG_RENDER_WIDTH * CATALOG_RENDER_HEIGHT;

fn render_area() -> core::UiTreeRenderArea {
    core::UiTreeRenderArea {
        x: 0,
        y: 0,
        width: RENDER_WIDTH,
        height: RENDER_HEIGHT,
        scroll_y: 0.0,
    }
}

fn catalog_render_area() -> core::UiTreeRenderArea {
    core::UiTreeRenderArea {
        x: 0,
        y: 0,
        width: CATALOG_RENDER_WIDTH,
        height: CATALOG_RENDER_HEIGHT,
        scroll_y: 0.0,
    }
}

#[test]
fn wrapper_uses_the_public_core_types() {
    assert_eq!(
        TypeId::of::<wrapper::Canvas>(),
        TypeId::of::<core::Canvas>()
    );
    assert_eq!(
        TypeId::of::<wrapper::UiTreeSurfaceHost>(),
        TypeId::of::<core::UiTreeSurfaceHost>()
    );
}

#[test]
fn wrapper_and_public_host_keep_raster_and_hit_results_identical() {
    let theme = ThemeSnapshot::light();
    let root = UiNode::new(UiNodeKind::Text, "KDV public raster host")
        .stable_node_id("kdv-public-raster-host");
    let mut wrapped_canvas = wrapper::Canvas::new(RENDER_WIDTH, RENDER_HEIGHT, BACKGROUND);
    let mut direct_canvas = core::Canvas::new(RENDER_WIDTH, RENDER_HEIGHT, BACKGROUND);
    let wrapped_host = wrapper::UiTreeSurfaceHost::new(theme.clone());
    let direct_host = core::UiTreeSurfaceHost::new(theme);

    wrapped_host.render(&mut wrapped_canvas, &root, render_area());
    direct_host.render(&mut direct_canvas, &root, render_area());

    assert_eq!(wrapped_canvas.pixels(), direct_canvas.pixels());
    assert!(wrapped_canvas.non_background_pixels(BACKGROUND) > 0);
    assert_eq!(
        wrapped_host.document_node_hits(&root, render_area()),
        direct_host.document_node_hits(&root, render_area())
    );
}

#[test]
fn public_raster_host_renders_every_storybook_catalog_tree() {
    let host = core::UiTreeSurfaceHost::new(ThemeSnapshot::light());
    let area = catalog_render_area();

    for example in StoryCatalog.examples() {
        let mut canvas = core::Canvas::new(CATALOG_RENDER_WIDTH, CATALOG_RENDER_HEIGHT, BACKGROUND);
        host.render(&mut canvas, example.tree.root(), area);

        assert_eq!(
            CATALOG_PIXEL_COUNT,
            canvas.pixels().len(),
            "{}",
            example.page
        );
        let host_actions = host.host_action_hits(example.tree.root(), area);
        let node_hits = host.document_node_hits(example.tree.root(), area);
        let _ = core::UiTreeSurfaceHost::interaction_target_for_hits_at(
            &host_actions,
            &node_hits,
            0.0,
            0.0,
        );
        let _ = core::UiTreeSurfaceHost::hits_at(&host_actions, 0.0, 0.0);
        let _ = core::UiTreeSurfaceHost::cursor_at(&host_actions, 0.0, 0.0);
    }
}
