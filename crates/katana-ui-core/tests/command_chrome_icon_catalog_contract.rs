use katana_ui_core::molecule::command_chrome::CommandChromeIcon;
use katana_ui_core::render_model::UiSvgPaintPolicy;

#[test]
fn all_command_chrome_icons_include_image() {
    assert!(CommandChromeIcon::all().contains(&CommandChromeIcon::Image));
}

#[test]
fn all_command_chrome_icons_have_complete_props_and_stable_output() {
    let icons = CommandChromeIcon::all();

    for icon in icons {
        let first = icon.icon_props();
        let second = icon.icon_props();

        assert!(!first.svg_source.trim().is_empty());
        assert!(first.svg_source.trim().starts_with("<svg"));
        assert!(!first.view_box.trim().is_empty());
        assert!(!first.path_summary.trim().is_empty());
        assert!(!first.role.trim().is_empty());
        assert_eq!(UiSvgPaintPolicy::CurrentColor, first.paint_policy);
        assert_eq!(first, second);
    }
}

#[test]
fn image_icon_has_the_generic_current_color_contract() {
    let props = CommandChromeIcon::Image.icon_props();

    assert_eq!("0 0 16 16", props.view_box);
    assert!(!props.role.trim().is_empty());
    assert!(!props.path_summary.trim().is_empty());
    assert_eq!(UiSvgPaintPolicy::CurrentColor, props.paint_policy);
    assert!(props.svg_source.contains("fill=\"currentColor\""));
}

#[test]
fn all_command_chrome_icons_define_non_empty_viewbox_and_summary_for_registry() {
    for icon in CommandChromeIcon::all() {
        let value = icon.icon_props();

        assert_eq!("0 0 16 16", value.view_box);
        assert!(value.path_summary.len() > 3);
    }
}

#[test]
fn command_icons_are_local_monochrome_path_only_sources() {
    for icon in CommandChromeIcon::all() {
        let source = icon.icon_props().svg_source;

        assert_eq!(1, source.matches("<path ").count(), "{icon:?}");
        assert!(source.contains("d=\""), "{icon:?}");
        assert!(!source.contains("<text"), "{icon:?}");
        assert!(!source.contains("<rect"), "{icon:?}");
        assert!(!source.contains("<image"), "{icon:?}");
        assert!(!source.contains("<use"), "{icon:?}");
    }
}

#[test]
fn emphasis_and_code_icons_use_distinct_conventional_geometry() {
    let strong = CommandChromeIcon::EmphasisStrong.icon_props();
    let italic = CommandChromeIcon::EmphasisItalic.icon_props();
    let strike = CommandChromeIcon::Strike.icon_props();
    let inline_code = CommandChromeIcon::InlineCode.icon_props();
    let heading_one = CommandChromeIcon::HeadingOne.icon_props();
    let code_block = CommandChromeIcon::CodeBlock.icon_props();

    assert_ne!(strong.svg_source, italic.svg_source);
    assert_ne!(strong.svg_source, strike.svg_source);
    assert_ne!(italic.svg_source, strike.svg_source);
    assert_ne!(inline_code.svg_source, code_block.svg_source);
    assert_ne!(heading_one.svg_source, strong.svg_source);

    assert!(strong.svg_source.contains("c"));
    assert!(italic.svg_source.contains("M6.1"));
    assert!(strike.svg_source.contains("M5.1") && strike.svg_source.contains("M2 7"));
    assert!(inline_code.svg_source.contains("m6.2") && inline_code.svg_source.contains("Zm3.6"));
    assert!(heading_one.svg_source.contains("M2 2h2v5"));
    assert!(code_block.svg_source.contains("m6.4") && code_block.svg_source.contains("M8.1"));

    for props in [strong, italic, strike, inline_code, heading_one, code_block] {
        assert!(
            ![
                "three-strong-bars",
                "diagonal-emphasis",
                "single-center-strike-line",
            ]
            .iter()
            .any(|summary| *summary == props.path_summary)
        );
        assert_eq!("0 0 16 16", props.view_box);
        assert_eq!(UiSvgPaintPolicy::CurrentColor, props.paint_policy);
    }
}
