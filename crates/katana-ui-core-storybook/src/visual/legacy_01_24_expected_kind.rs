use katana_ui_core::render_model::UiNodeKind;

pub(super) fn expected_kind(page: &str) -> UiNodeKind {
    match page {
        "theme-tokens" => UiNodeKind::Card,
        "text" => UiNodeKind::Text,
        "icon" => UiNodeKind::Icon,
        "loading-dots" => UiNodeKind::LoadingDots,
        "spinner" => UiNodeKind::Spinner,
        "svg-button" => UiNodeKind::SvgButton,
        "text-button" => UiNodeKind::TextButton,
        "icon-text-button" => UiNodeKind::IconTextButton,
        "toggle" => UiNodeKind::Toggle,
        "segmented-toggle" => UiNodeKind::SegmentedToggle,
        "select-box" => UiNodeKind::SelectBox,
        "color-swatch" => UiNodeKind::ColorSwatch,
        "text-input" => UiNodeKind::Input,
        "text-area" => UiNodeKind::TextArea,
        "search-box" => UiNodeKind::SearchBox,
        "tooltip" => UiNodeKind::Tooltip,
        "badge" => UiNodeKind::Badge,
        "key-cap" => UiNodeKind::KeyCap,
        "card" => UiNodeKind::Card,
        "accordion" => UiNodeKind::Accordion,
        "split-pane" => UiNodeKind::SplitPane,
        "modal" => UiNodeKind::Modal,
        "modal-overlay" => UiNodeKind::ModalOverlay,
        "popover" => UiNodeKind::Popover,
        "color-picker-rgba" => UiNodeKind::ColorPicker,
        "code-diff" => UiNodeKind::CodeDiff,
        _ => UiNodeKind::Text,
    }
}
