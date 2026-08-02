use super::canvas::Canvas;
use super::dedicated_fallback;
use super::dedicated_node_labels;
use super::palette::VisualPalette;
use super::text::TextRenderer;
use katana_ui_core::facade::UiCoreFacade;
use katana_ui_core::render_model::{UiNode, UiNodeKind};
use katana_ui_core::theme::ThemeSnapshot;

const DRAW_KIND_X: usize = 8;
const DRAW_KIND_ROW_STEP: usize = 52;

#[test]
fn fallback_renderer_and_labels_cover_every_public_node_kind() {
    let facade = UiCoreFacade::new(ThemeSnapshot::dark());
    let text = TextRenderer::load(&facade, facade.default_font_role());
    let palette = VisualPalette::from_theme(facade.theme());
    let kinds = [
        UiNodeKind::Text,
        UiNodeKind::Icon,
        UiNodeKind::ImageSurface,
        UiNodeKind::Chip,
        UiNodeKind::Button,
        UiNodeKind::Input,
        UiNodeKind::TextArea,
        UiNodeKind::Checkbox,
        UiNodeKind::Radio,
        UiNodeKind::Badge,
        UiNodeKind::Divider,
        UiNodeKind::Spacer,
        UiNodeKind::KeyCap,
        UiNodeKind::LoadingDots,
        UiNodeKind::Spinner,
        UiNodeKind::ProgressBar,
        UiNodeKind::ColorSwatch,
        UiNodeKind::Toggle,
        UiNodeKind::SlideControl,
        UiNodeKind::SvgButton,
        UiNodeKind::TextButton,
        UiNodeKind::IconTextButton,
        UiNodeKind::DragHandle,
        UiNodeKind::DropIndicator,
        UiNodeKind::Card,
        UiNodeKind::List,
        UiNodeKind::Menu,
        UiNodeKind::Tooltip,
        UiNodeKind::Modal,
        UiNodeKind::Tabs,
        UiNodeKind::CloseableTabStrip,
        UiNodeKind::CloseableTabGroupHeader,
        UiNodeKind::CloseableTab,
        UiNodeKind::Toolbar,
        UiNodeKind::FormField,
        UiNodeKind::Breadcrumb,
        UiNodeKind::Accordion,
        UiNodeKind::CodeDiff,
        UiNodeKind::ColorPicker,
        UiNodeKind::ComboBox,
        UiNodeKind::CommandPalette,
        UiNodeKind::CommandResultRow,
        UiNodeKind::DiagnosticsList,
        UiNodeKind::DynamicArrayEditor,
        UiNodeKind::EmptyState,
        UiNodeKind::Banner,
        UiNodeKind::MenuButton,
        UiNodeKind::ModalOverlay,
        UiNodeKind::NotificationToast,
        UiNodeKind::ToastStackManager,
        UiNodeKind::Popover,
        UiNodeKind::HoverCard,
        UiNodeKind::DragPreview,
        UiNodeKind::SearchBox,
        UiNodeKind::SearchControlStrip,
        UiNodeKind::SegmentedToggle,
        UiNodeKind::SelectBox,
        UiNodeKind::SelectionList,
        UiNodeKind::SideMenu,
        UiNodeKind::StatusBar,
        UiNodeKind::AttachmentChip,
        UiNodeKind::ChipGroup,
        UiNodeKind::TreeView,
        UiNodeKind::ContextMenu,
        UiNodeKind::ShortcutCombo,
        UiNodeKind::ShortcutCheatsheet,
        UiNodeKind::SettingsList,
        UiNodeKind::CollapsibleSidebar,
        UiNodeKind::CollapsiblePanel,
        UiNodeKind::VirtualizedList,
        UiNodeKind::VirtualizedTree,
        UiNodeKind::Skeleton,
        UiNodeKind::SkeletonCluster,
        UiNodeKind::MotionPrimitive,
        UiNodeKind::WindowControlButtonGroup,
        UiNodeKind::StartupStatePanel,
        UiNodeKind::Panel,
        UiNodeKind::Row,
        UiNodeKind::Column,
        UiNodeKind::Stack,
        UiNodeKind::Grid,
        UiNodeKind::ScrollArea,
        UiNodeKind::SplitPane,
        UiNodeKind::AlignCenter,
        UiNodeKind::AlignNode,
    ];
    let kind_count = kinds.len();
    let mut canvas = Canvas::new(480, kind_count * DRAW_KIND_ROW_STEP, palette.background);

    for (index, kind) in kinds.into_iter().enumerate() {
        draw_kind(&mut canvas, &text, &palette, kind, index);
    }

    assert_eq!(kind_count, 85);
    assert!(
        canvas.non_background_pixels(palette.background) > kind_count * 100,
        "every fallback branch must draw visible pixels"
    );
    assert_eq!(
        "node",
        dedicated_node_labels::label_for(UiNodeKind::AlignNode)
    );
}

fn draw_kind(
    canvas: &mut Canvas,
    text: &TextRenderer,
    palette: &VisualPalette,
    kind: UiNodeKind,
    index: usize,
) {
    let node = UiNode::new(kind, dedicated_node_labels::label_for(kind));
    dedicated_fallback::draw(
        canvas,
        text,
        &node,
        palette,
        DRAW_KIND_X,
        index * DRAW_KIND_ROW_STEP,
    );
}
