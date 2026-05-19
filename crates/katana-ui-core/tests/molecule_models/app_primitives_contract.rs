use katana_ui_core::atom::Text;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::UiAction;
use katana_ui_core::render_model::{UiNodeKind, UiSize, UiTree, UiVariant};
use katana_ui_core::widget::atoms::{
    KeyCombo, KeyKind, KeyModifiers, RuntimePlatform, ShortcutCombo,
};
use katana_ui_core::widget::molecules::{
    AppShell, AppShellSlot, AppShellSlotKind, CollapsibleSidebar, MotionPrimitive,
    MotionPrimitiveKind, MotionSpec, ReducedMotionPolicy, ResizableWidth, RowHeightProvider,
    SettingsControl, SettingsDirtyVisualization, SettingsField, SettingsList, SettingsListEvent,
    SettingsSection, SettingsValue, ShortcutCheatsheet, ShortcutCheatsheetAction,
    ShortcutCheatsheetEvent, ShortcutCheatsheetGroup, ShortcutCheatsheetItem, SidebarEvent,
    SidebarMode, Skeleton, SkeletonAnimation, SkeletonCluster, SkeletonShape, SplashEvent,
    SplashScreen, SplashSize, SplashStatus, TitleBar, TitleBarEvent, TitleBarStyle,
    VirtualizationConfig, VirtualizedEvent, VirtualizedList, VirtualizedTree, WindowControlKind,
    WindowControlsPosition,
};

#[test]
fn shortcut_combo_and_cheatsheet_keep_typed_query_and_selection_event() {
    let combo = KeyCombo::new(
        KeyModifiers {
            command: true,
            ..KeyModifiers::default()
        },
        KeyKind::Char('s'),
    );
    let mut sheet =
        ShortcutCheatsheet::new("Shortcuts").group(ShortcutCheatsheetGroup::new("File").item(
            ShortcutCheatsheetItem::new("save", "Save file", combo.clone()),
        ));

    let query = sheet.apply_action(ShortcutCheatsheetAction::SetQuery("Save".to_string()));
    assert_eq!(
        Some(ShortcutCheatsheetEvent::QueryChanged("Save".to_string())),
        query
    );
    assert_eq!(1, sheet.visible_items().len());

    let select = sheet.apply_action(ShortcutCheatsheetAction::SelectShortcut("save".to_string()));
    assert_eq!(
        Some(ShortcutCheatsheetEvent::ShortcutSelected {
            id: "save".to_string(),
            combo: combo.clone()
        }),
        select
    );

    let tree = UiTree::new(sheet);
    assert_eq!(UiNodeKind::ShortcutCheatsheet, tree.root().kind());
    assert_eq!("⌘S", tree.root().children()[0].props().shortcut.combo);
    assert_eq!(
        "Command + S",
        ShortcutCombo::new("Save file", combo).accessibility_text(RuntimePlatform::MacOS)
    );
}

#[test]
fn settings_list_filters_resets_and_collapses_with_typed_events() {
    let field = SettingsField::new(
        "theme",
        "Theme",
        SettingsControl::Select {
            options: Vec::new(),
            selected: "Dark".to_string(),
        },
    )
    .reset_to_default(SettingsValue::Text("Light".to_string()));
    let section = SettingsSection::new("appearance", "Appearance")
        .collapsible(true)
        .field(field);
    let mut list = SettingsList::new("Settings")
        .dirty_visualization(SettingsDirtyVisualization::Marker)
        .section(section);

    assert_eq!(1, list.visible_fields().len());
    let reset = UiAction::clear_value(list.state_id().clone());
    assert!(list.apply_action(&reset).handled);
    assert!(matches!(
        list.last_event(),
        Some(SettingsListEvent::FieldReset { field_id }) if field_id == "theme"
    ));

    let collapse = UiAction::set_selected_index(list.state_id().clone(), 0);
    assert!(list.apply_action(&collapse).handled);
    assert!(matches!(
        list.last_event(),
        Some(SettingsListEvent::SectionCollapsed { section_id, collapsed })
            if section_id == "appearance" && *collapsed
    ));
    assert!(list.visible_fields().is_empty());
}

#[test]
fn sidebar_and_app_shell_expose_width_mode_and_slot_contract() {
    let width = ResizableWidth {
        min: 160,
        max: 320,
        current: 240,
        persist_id: "main-sidebar".to_string(),
    };
    let mut sidebar = CollapsibleSidebar::new("Sidebar", width).mode(SidebarMode::Collapsed);
    let resize = UiAction::set_value(sidebar.state_id().clone(), "999");
    assert!(sidebar.apply_action(&resize).handled);
    assert_eq!(320, sidebar.width());
    assert_eq!(
        SidebarEvent::WidthChanged(320, "main-sidebar".to_string()),
        *sidebar.last_event()
    );

    let shell = AppShell::new("App", 1000).slot(AppShellSlot {
        kind: AppShellSlotKind::Leading,
        width: sidebar.width(),
        node: sidebar.into(),
    });
    assert_eq!(680, shell.main_available_width());
    assert_eq!(UiNodeKind::AppShell, UiTree::new(shell).root().kind());
}

#[test]
fn virtualized_list_and_tree_compute_stable_visible_ranges() {
    let config = VirtualizationConfig {
        total_count: 10_000,
        viewport_height: 100,
        scroll_y: 900,
        overscan: 2,
        row_height: RowHeightProvider::Fixed(20),
        keep_focused_in_window: true,
        focused_index: Some(80),
    };
    let mut list = VirtualizedList::new("Rows", config.clone());
    assert_eq!(43, list.visible_range().start);
    assert_eq!(81, list.visible_range().end);

    let scroll = UiAction::set_value(list.state_id().clone(), "1000");
    assert!(list.apply_action(&scroll).handled);
    assert!(matches!(list.last_event(), VirtualizedEvent::Scrolled(_)));

    let tree = VirtualizedTree::new("Tree", config).expanded_node("root");
    assert_eq!(10_000, tree.visible_range().aria_set_size);
    assert_eq!(UiNodeKind::VirtualizedTree, UiTree::new(tree).root().kind());
}

#[test]
fn skeleton_and_motion_make_passive_and_reduced_motion_contract_explicit() {
    let skeleton = Skeleton::new("Avatar", SkeletonShape::Circle)
        .animation(SkeletonAnimation::Shimmer)
        .reduced_motion(true);
    assert_eq!(SkeletonAnimation::None, skeleton.effective_animation());

    let cluster = SkeletonCluster::new("Loading").item(skeleton);
    let tree = UiTree::new(cluster);
    assert_eq!(UiNodeKind::SkeletonCluster, tree.root().kind());
    assert_eq!("loading", tree.root().props().accessibility_label);

    let spec = MotionSpec {
        primitive: MotionPrimitiveKind::Slide,
        duration_ms: 180,
        distance_px: 12,
        policy: ReducedMotionPolicy::Respect,
    };
    let mut motion = MotionPrimitive::new("Panel motion", spec);
    let reduce = UiAction::reduced_motion(motion.state_id().clone(), true);
    assert!(motion.apply_action(&reduce).handled);
    assert_eq!(0, motion.effective_duration_ms());
}

#[test]
fn title_bar_window_chrome_and_splash_screen_emit_window_events() {
    let mut title = TitleBar::new("Window").options(
        TitleBarStyle::Unified,
        WindowControlsPosition::Trailing,
        UiSize::Small,
    );
    let close = UiAction::set_selected_index(title.state_id().clone(), 0);
    assert!(title.apply_action(&close).handled);
    assert_eq!(
        TitleBarEvent::ControlPressed(WindowControlKind::Close),
        *title.last_event()
    );
    let title_tree = UiTree::new(title);
    assert_eq!(UiVariant::Filled, title_tree.root().props().variant);

    let spec = MotionSpec {
        primitive: MotionPrimitiveKind::Fade,
        duration_ms: 120,
        distance_px: 0,
        policy: ReducedMotionPolicy::ForceReduced,
    };
    let mut splash = SplashScreen::new("Boot", spec)
        .status(SplashStatus::Error)
        .progress(Some(40))
        .size(SplashSize::Window);
    assert_eq!("alert", splash.accessibility_role());
    let retry = UiAction::button_press(splash.state_id().clone());
    assert!(splash.apply_action(&retry).handled);
    assert_eq!(SplashEvent::Retried, *splash.last_event());

    let splash_tree = UiTree::new(splash);
    assert_eq!(UiNodeKind::SplashScreen, splash_tree.root().kind());
    assert_eq!(40, splash_tree.root().props().progress_percent);
}

#[test]
fn widget_molecules_public_api_exports_app_primitives() {
    let tree = UiTree::new(AppShell::new("App", 800).slot(AppShellSlot {
        kind: AppShellSlotKind::Main,
        width: 800,
        node: Text::new("Main").into(),
    }));
    assert_eq!(1, tree.root().children().len());
}
