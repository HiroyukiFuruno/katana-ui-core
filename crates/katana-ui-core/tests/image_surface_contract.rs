use katana_ui_core::atom::ImageSurface;
use katana_ui_core::render_model::{
    UiCommonProps, UiImageSurfaceFit, UiImageSurfaceHighlight, UiImageSurfaceProps,
    UiImageSurfaceRenderPlan, UiImageSurfaceTransform, UiImageSurfaceValidationError, UiNode,
    UiNodeKind, UiRect, UiTree, UiTreeSemantics,
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

#[test]
fn image_surface_validation_and_display_cover_all_failure_kinds() {
    let cases = [
        (
            UiImageSurfaceProps::new("", 1, 1, vec![0; 4]),
            "image surface fingerprint is empty",
        ),
        (
            UiImageSurfaceProps::new("zero", 0, 1, Vec::new()),
            "image surface extent must be non-zero",
        ),
        (
            UiImageSurfaceProps::new("overflow", u32::MAX, u32::MAX, Vec::new()),
            "rgba length overflow for image surface extent 4294967295x4294967295",
        ),
    ];

    for (result, message) in cases {
        assert!(result.is_err(), "surface must be rejected");
        let Err(error) = result else {
            continue;
        };
        assert_eq!(message, error.to_string());
    }
}

#[test]
fn image_surface_fit_variants_and_invalid_exact_dimensions_are_typed() {
    assert_eq!(
        [
            UiImageSurfaceFit::Original,
            UiImageSurfaceFit::Contain,
            UiImageSurfaceFit::Cover,
            UiImageSurfaceFit::Stretch,
        ],
        [
            UiImageSurfaceFit::Original,
            UiImageSurfaceFit::default(),
            UiImageSurfaceFit::Cover,
            UiImageSurfaceFit::Stretch,
        ]
    );

    let props = UiImageSurfaceProps::new("surface", 1, 1, vec![0; 4]);
    assert!(props.is_ok(), "valid surface");
    let Ok(props) = props else {
        return;
    };
    let props = props.display_size_exact(f32::NAN, -1.0);
    assert_eq!(0, props.display_width);
    assert_eq!(0, props.display_height);
    assert_eq!(0, props.display_width_milli);
    assert_eq!(0, props.display_height_milli);
}

#[test]
fn image_surface_atom_projects_integer_exact_and_common_display_contracts() {
    let surface = ImageSurface::from_rgba("Preview", "surface", 2, 1, sample_rgba());
    assert!(surface.is_ok(), "valid image surface");
    let Ok(surface) = surface else {
        return;
    };
    let node = UiNode::from(
        surface
            .display_size(640, 320)
            .display_size_exact(640.5, 320.25)
            .common(UiCommonProps::default().selectable(true)),
    );

    assert_eq!(641, node.props().image_surface.display_width);
    assert_eq!(321, node.props().image_surface.display_height);
    assert_eq!(640_500, node.props().image_surface.display_width_milli);
    assert_eq!(320_250, node.props().image_surface.display_height_milli);
    assert!(node.props().common.selectable);
}

#[test]
fn image_surface_zoom_factor_has_a_nonzero_lower_bound() {
    assert_eq!(0.01, UiImageSurfaceTransform::new(0, 0, 0).zoom_factor());
    assert_eq!(1.75, UiImageSurfaceTransform::new(175, 0, 0).zoom_factor());
}

#[test]
fn semantic_fingerprint_changes_when_image_surface_descriptor_changes()
-> Result<(), UiImageSurfaceValidationError> {
    let base = surface_fingerprint(ImageSurface::from_rgba(
        "Preview",
        "same-surface",
        2,
        1,
        sample_rgba(),
    )?);
    let resized = surface_fingerprint(ImageSurface::from_rgba(
        "Preview",
        "same-surface",
        1,
        2,
        sample_rgba(),
    )?);
    let scaled = surface_fingerprint(
        ImageSurface::from_rgba("Preview", "same-surface", 2, 1, sample_rgba())?.content_scale(200),
    );
    let covered = surface_fingerprint(
        ImageSurface::from_rgba("Preview", "same-surface", 2, 1, sample_rgba())?
            .fit(UiImageSurfaceFit::Cover),
    );
    let highlighted = surface_fingerprint(
        ImageSurface::from_rgba("Preview", "same-surface", 2, 1, sample_rgba())?.highlight_rect(
            UiImageSurfaceHighlight::current_search_hit(UiRect::new(0, 0, 1, 1), "hit"),
        ),
    );

    assert_ne!(base, resized);
    assert_ne!(base, scaled);
    assert_ne!(base, covered);
    assert_ne!(base, highlighted);
    Ok(())
}

#[test]
fn image_surface_render_plan_collects_nested_surface_without_rgba_copy()
-> Result<(), UiImageSurfaceValidationError> {
    let highlight = UiImageSurfaceHighlight::search_hit(UiRect::new(1, 2, 3, 4), "match");
    let transform = UiImageSurfaceTransform::new(175, -12, 24);
    let surface = ImageSurface::from_rgba("Preview", "surface-sha", 2, 1, sample_rgba())?
        .content_scale(200)
        .fit(UiImageSurfaceFit::Cover)
        .accessibility_label("Rendered preview")
        .selection_text("selected text")
        .highlight_rect(highlight.clone())
        .transform(transform);
    let tree = UiTree::new(UiNode::new(UiNodeKind::Column, "root").child(surface));

    let plans = UiImageSurfaceRenderPlan::collect_from_tree(&tree);

    assert_eq!(1, plans.len());
    assert_eq!("surface-sha", plans[0].fingerprint);
    assert_eq!(
        (2, 1, 8),
        (plans[0].width, plans[0].height, plans[0].rgba_byte_len)
    );
    assert_eq!(200, plans[0].content_scale);
    assert_eq!(UiImageSurfaceFit::Cover, plans[0].fit);
    assert_eq!("Rendered preview", plans[0].accessibility_label);
    assert_eq!("selected text", plans[0].selection_text);
    assert_eq!(vec![highlight], plans[0].highlight_rects);
    assert_eq!(transform, plans[0].transform);
    Ok(())
}

fn surface_fingerprint(surface: ImageSurface) -> String {
    UiTreeSemantics::fingerprint(&UiTree::new(surface))
}
