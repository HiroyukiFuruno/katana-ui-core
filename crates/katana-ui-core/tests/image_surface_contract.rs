use katana_ui_core::atom::ImageSurface;
use katana_ui_core::render_model::{
    UiImageSurfaceFit, UiImageSurfaceHighlight, UiImageSurfaceProps, UiImageSurfaceValidationError,
    UiNode, UiNodeKind, UiRect,
};

fn sample_rgba() -> Vec<u8> {
    vec![255, 0, 0, 255, 0, 0, 255, 255]
}

#[test]
fn image_surface_atom_carries_rgba_surface_and_highlights()
-> Result<(), UiImageSurfaceValidationError> {
    let highlight =
        UiImageSurfaceHighlight::current_search_hit(UiRect::new(4, 6, 20, 8), "current search hit");
    let node = UiNode::from(
        ImageSurface::from_rgba("Preview", "surface-sha", 2, 1, sample_rgba())?
            .content_scale(200)
            .fit(UiImageSurfaceFit::Contain)
            .accessibility_label("Rendered preview surface")
            .highlight_rect(highlight.clone()),
    );

    assert_eq!(UiNodeKind::ImageSurface, node.kind());
    assert_eq!("surface-sha", node.props().image_surface.fingerprint);
    assert_eq!(2, node.props().image_surface.width);
    assert_eq!(1, node.props().image_surface.height);
    assert_eq!(8, node.props().image_surface.rgba.len());
    assert_eq!(200, node.props().image_surface.content_scale);
    assert_eq!(UiImageSurfaceFit::Contain, node.props().image_surface.fit);
    assert_eq!(
        "Rendered preview surface",
        node.props().image_surface.accessibility_label
    );
    assert_eq!(&highlight, &node.props().image_surface.highlight_rects[0]);
    assert_eq!("Rendered preview surface", node.props().accessibility_label);
    Ok(())
}

#[test]
fn ui_node_builder_accepts_image_surface_props() -> Result<(), UiImageSurfaceValidationError> {
    let props = UiImageSurfaceProps::new("surface-sha", 2, 1, sample_rgba())?
        .accessibility_label("Preview surface");
    let node = UiNode::new(UiNodeKind::ImageSurface, "Preview").image_surface(props);

    assert_eq!("surface-sha", node.props().image_surface.fingerprint);
    assert_eq!(
        "Preview surface",
        node.props().image_surface.accessibility_label
    );
    Ok(())
}

#[test]
fn image_surface_rejects_rgba_payload_that_does_not_match_extent() -> Result<(), String> {
    let result = UiImageSurfaceProps::new("surface-sha", 2, 1, vec![255, 0, 0, 255]);
    let error = result.map(|_| ()).map_err(|error| error.to_string());

    assert_eq!(
        Err("rgba length mismatch: expected 8 bytes, got 4 bytes".to_string()),
        error
    );
    Ok(())
}
