use super::super::{StoryCatalog, StoryExample};
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{UiAction, UiCallbackLog};
use katana_ui_core::render_model::UiStateId;
use katana_ui_core::{atom, layout, molecule};
use molecule::{
    CollapsiblePanelAction, CollapsiblePanelWidth, MotionPrimitiveKind, MotionSpec, PanelMode,
    PanelSide, ReducedMotionPolicy, RowHeightProvider, SkeletonClusterPreset, VirtualizationConfig,
};

const PANEL_MIN_WIDTH: u16 = 180;
const PANEL_MAX_WIDTH: u16 = 360;
const PANEL_DEFAULT_WIDTH: u16 = 240;
const PANEL_RESIZED_WIDTH: u16 = 320;
const VIRTUAL_TOTAL_ROWS: usize = 10_000;
const VIRTUAL_ROW_HEIGHT: u32 = 28;
const VIRTUAL_VIEWPORT_HEIGHT: u32 = 168;
const VIRTUAL_SCROLL_OFFSET: u32 = 1_260;
const VIRTUAL_FOCUSED_INDEX: usize = 120;
const MOTION_DURATION_MS: u16 = 180;
const MOTION_DISTANCE_PX: u16 = 24;
const MOTION_PHASE: u16 = 2;

pub(super) fn examples() -> Vec<StoryExample> {
    vec![
        collapsible_panel_story(),
        virtualization_story(),
        skeleton_cluster_story(),
        motion_story(),
    ]
}

fn collapsible_panel_story() -> StoryExample {
    let mut panel = molecule::CollapsiblePanel::new("Collapsible panel", panel_width())
        .side(PanelSide::Leading)
        .resize_handle(true)
        .expand_on_hover(true)
        .content(atom::Text::new(
            "Explorer panel: mode=Expanded width=240 pinned=true expand_on_hover=true resize_handle=true",
        ))
        .content(atom::Text::new(
            "Chat history panel: child slot shows recent threads and pinned sessions",
        ))
        .content(atom::Text::new(
            "TOC panel: child slot shows document headings and current section",
        ))
        .content(atom::Text::new(
            "Floating overlay: mode=FloatingOverlay layout_width=0 overlay_z=80",
        ))
        .content(atom::Text::new(
            "IconOnly: mode=IconOnly layout_width=56 child slot keeps icon navigation",
        ));
    let target = panel.state_id().clone();
    let resized = panel.apply_action(CollapsiblePanelAction::Resize(PANEL_RESIZED_WIDTH));
    let overlay = panel.apply_action(CollapsiblePanelAction::SetMode(PanelMode::FloatingOverlay));
    let icon_only = panel.apply_action(CollapsiblePanelAction::SetMode(PanelMode::IconOnly));
    let unpinned = panel.apply_action(CollapsiblePanelAction::Unpin);
    let hover_opened = panel.apply_action(CollapsiblePanelAction::HoverTrigger);
    let logs = vec![
        UiCallbackLog::new(
            target.clone(),
            "collapsible_panel_resize",
            "width=240 resize_handle=true",
            format!("events={resized:?}"),
        ),
        UiCallbackLog::new(
            target.clone(),
            "collapsible_panel_overlay",
            "mode=Expanded pinned=true",
            format!("events={overlay:?}"),
        ),
        UiCallbackLog::new(
            target.clone(),
            "collapsible_panel_icon_only",
            "mode=FloatingOverlay width=320",
            format!("events={icon_only:?}"),
        ),
        UiCallbackLog::new(
            target.clone(),
            "collapsible_panel_hover",
            "mode=IconOnly expand_on_hover=true",
            format!("events={hover_opened:?}"),
        ),
        UiCallbackLog::new(
            target,
            "collapsible_panel_pin",
            "pinned=true",
            format!("events={unpinned:?}"),
        ),
    ];
    StoryCatalog::interactive_story("collapsible-panel", panel, logs)
}

fn panel_width() -> CollapsiblePanelWidth {
    CollapsiblePanelWidth::new(
        PANEL_MIN_WIDTH,
        PANEL_MAX_WIDTH,
        PANEL_DEFAULT_WIDTH,
        PANEL_DEFAULT_WIDTH,
        Some("storybook.panel.width"),
    )
}

fn virtualization_story() -> StoryExample {
    let mut list = molecule::VirtualizedList::new("Virtualized list", virtualization_config());
    let target = list.state_id().clone();
    let scroll = list.apply_action(&UiAction::set_value(
        target.clone(),
        VIRTUAL_SCROLL_OFFSET.to_string(),
    ));
    let focus = list.apply_action(&UiAction::set_selected_index(target, VIRTUAL_FOCUSED_INDEX));
    let logs = vec![
        UiCallbackLog::new(
            UiStateId::new("state:VirtualizedList:storybook"),
            "virtualized_scroll",
            "offset=0",
            format!("events={:?}", scroll.callback_log),
        ),
        UiCallbackLog::new(
            UiStateId::new("state:VirtualizedList:storybook"),
            "virtualized_focus_keep",
            "focused=None",
            format!("events={:?}", focus.callback_log),
        ),
    ];
    StoryCatalog::interactive_story("virtualization", list, logs)
}

fn virtualization_config() -> VirtualizationConfig {
    VirtualizationConfig {
        enabled: true,
        total_count: VIRTUAL_TOTAL_ROWS,
        viewport_offset: 0,
        viewport_height: VIRTUAL_VIEWPORT_HEIGHT,
        overscan: 2,
        row_height_provider: RowHeightProvider::Fixed {
            height: VIRTUAL_ROW_HEIGHT,
        },
        keep_focused_in_window: true,
        focused_index: None,
    }
}

fn skeleton_cluster_story() -> StoryExample {
    let list = skeleton_cluster_preset("list loading", SkeletonClusterPreset::ListRow);
    let message = skeleton_cluster_preset("message loading", SkeletonClusterPreset::Message);
    let card = skeleton_cluster_preset("card loading", SkeletonClusterPreset::Card);
    let paragraph = skeleton_cluster_preset("paragraph loading", SkeletonClusterPreset::Paragraph);
    let code_block =
        skeleton_cluster_preset("code block loading", SkeletonClusterPreset::CodeBlock);
    let image_card =
        skeleton_cluster_preset("image card loading", SkeletonClusterPreset::ImageCard);
    let logs = vec![
        UiCallbackLog::new(
            card.state_id().clone(),
            "skeleton_cluster_preset_apply",
            "preset=ListRow children=2 live_region=Loading list loading reduced_motion=false",
            "preset=Card children=2 live_region=Loading card loading reduced_motion=false event=skeleton_cluster_changed",
        ),
        UiCallbackLog::new(
            message.state_id().clone(),
            "skeleton_cluster_changed",
            "preset=Message children=3 live_region=Loading message loading reduced_motion=false",
            "preset=ImageCard children=3 live_region=Loading image card loading reduced_motion=false",
        ),
    ];
    StoryCatalog::interactive_story(
        "skeleton-cluster",
        layout::Column::new()
            .child(list)
            .child(message)
            .child(card)
            .child(paragraph)
            .child(code_block)
            .child(image_card),
        logs,
    )
}

fn skeleton_cluster_preset(
    label: &'static str,
    preset: SkeletonClusterPreset,
) -> molecule::SkeletonCluster {
    molecule::SkeletonCluster::new(label).preset(preset)
}

fn motion_story() -> StoryExample {
    let spec = MotionSpec::new(
        MotionPrimitiveKind::Slide,
        MOTION_DURATION_MS,
        MOTION_DISTANCE_PX,
        ReducedMotionPolicy::Respect,
    );
    let mut motion = molecule::MotionPrimitive::new("Motion primitive", spec);
    let target = motion.state_id().clone();
    let reduced = motion.apply_action(&UiAction::reduced_motion(target.clone(), true));
    let tick = motion.apply_action(&UiAction::animation_tick(target, MOTION_PHASE));
    let logs = vec![
        UiCallbackLog::new(
            UiStateId::new("state:MotionPrimitive:storybook"),
            "motion_reduce",
            "instant=false",
            format!("events={:?}", reduced.callback_log),
        ),
        UiCallbackLog::new(
            UiStateId::new("state:MotionPrimitive:storybook"),
            "motion_tick",
            "phase=0",
            format!("events={:?}", tick.callback_log),
        ),
    ];
    StoryCatalog::interactive_story("motion", motion, logs)
}
