use super::{CommandChromeFamilyId, CommandChromeToolbar};
use crate::interaction::placement::{Placement, PlacementResult, Rect, Size};
use crate::render_model::UiNodeId;
use serde::{Deserialize, Serialize};

const DEFAULT_CLAMP_MARGIN: i32 = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FloatingCommandToolbarLayout {
    pub anchor: Rect,
    pub panel_size: Size,
    pub viewport: Rect,
}

impl FloatingCommandToolbarLayout {
    #[must_use]
    pub const fn new(anchor: Rect, panel_size: Size, viewport: Rect) -> Self {
        Self {
            anchor,
            panel_size,
            viewport,
        }
    }

    /// Creates an adapter-measured layout. Consumers provide only surface-derived facts.
    #[must_use]
    pub const fn unmeasured(anchor: Rect, viewport: Rect) -> Self {
        Self::new(anchor, Size::new(0, 0), viewport)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FloatingCommandToolbarCloseReason {
    OutsideClick,
    ConsumerSurfaceClick,
    Escape,
    Explicit,
}

/// Consumer が指定する floating command toolbar の初期可視状態。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum FloatingCommandToolbarVisibility {
    #[default]
    Closed,
    Visible,
}

/// Consumer-supplied frame facts for an adapter-measured floating toolbar.
///
/// Panel dimensions are intentionally absent: they are measured by the KUC adapter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FloatingCommandToolbarPresentation {
    pub anchor: Rect,
    pub viewport: Rect,
    pub visibility: FloatingCommandToolbarVisibility,
}

impl FloatingCommandToolbarPresentation {
    #[must_use]
    pub const fn new(
        anchor: Rect,
        viewport: Rect,
        visibility: FloatingCommandToolbarVisibility,
    ) -> Self {
        Self {
            anchor,
            viewport,
            visibility,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FloatingCommandToolbar {
    pub(super) toolbar: CommandChromeToolbar,
    pub(super) layout: FloatingCommandToolbarLayout,
    pub(super) placement_priority: Vec<Placement>,
    pub(super) clamp_margin: i32,
    pub(super) focus_return_target: Option<UiNodeId>,
    pub(super) open: bool,
    pub(super) placement: Option<PlacementResult>,
    pub(super) bounds: Option<Rect>,
}

impl FloatingCommandToolbar {
    #[must_use]
    pub fn new(toolbar: CommandChromeToolbar, layout: FloatingCommandToolbarLayout) -> Self {
        Self {
            toolbar,
            layout,
            placement_priority: vec![Placement::BottomStart, Placement::TopStart],
            clamp_margin: DEFAULT_CLAMP_MARGIN,
            focus_return_target: None,
            open: false,
            placement: None,
            bounds: None,
        }
    }

    /// Assigns an explicit family identity for this mounted slot.
    #[must_use]
    pub fn command_family(mut self, value: CommandChromeFamilyId) -> Self {
        self.toolbar = self.toolbar.command_family(value);
        self
    }

    #[must_use]
    pub const fn command_family_id(&self) -> &CommandChromeFamilyId {
        self.toolbar.command_family_id()
    }

    /// Creates a floating toolbar whose panel dimensions are measured by the KUC adapter.
    #[must_use]
    pub fn new_adapter_measured(
        toolbar: CommandChromeToolbar,
        anchor: Rect,
        viewport: Rect,
    ) -> Self {
        Self::new(
            toolbar,
            FloatingCommandToolbarLayout::unmeasured(anchor, viewport),
        )
    }

    #[must_use]
    pub fn placement_priority(mut self, value: impl IntoIterator<Item = Placement>) -> Self {
        self.placement_priority = value.into_iter().collect();
        self
    }

    #[must_use]
    pub const fn clamp_margin(mut self, value: i32) -> Self {
        self.clamp_margin = value;
        self
    }

    #[must_use]
    pub fn focus_return_target(mut self, value: UiNodeId) -> Self {
        self.focus_return_target = Some(value);
        self
    }

    /// Consumer の host state から初期可視状態を設定する。
    ///
    /// construction は interaction ではないため `Opened` event は発生させない。
    /// visible 初期状態では placement を直ちに解決する。
    #[must_use]
    pub fn initial_visibility(mut self, value: FloatingCommandToolbarVisibility) -> Self {
        if value == FloatingCommandToolbarVisibility::Visible {
            self.initialize_visible();
        }
        self
    }

    #[must_use]
    pub const fn toolbar_model(&self) -> &CommandChromeToolbar {
        &self.toolbar
    }

    #[must_use]
    pub fn toolbar_model_mut(&mut self) -> &mut CommandChromeToolbar {
        &mut self.toolbar
    }

    #[must_use]
    pub const fn layout_model(&self) -> FloatingCommandToolbarLayout {
        self.layout
    }

    #[must_use]
    pub const fn is_open(&self) -> bool {
        self.open
    }

    #[must_use]
    pub const fn visibility_model(&self) -> FloatingCommandToolbarVisibility {
        if self.open {
            FloatingCommandToolbarVisibility::Visible
        } else {
            FloatingCommandToolbarVisibility::Closed
        }
    }

    #[must_use]
    pub const fn placement_model(&self) -> Option<PlacementResult> {
        self.placement
    }

    #[must_use]
    pub const fn bounds_model(&self) -> Option<Rect> {
        self.bounds
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interaction::placement::{Placement, Rect, Size};
    use crate::molecule::command_chrome::{
        CommandChromeAction, CommandChromeFamilyId, CommandChromeIcon,
    };
    use crate::render_model::UiNodeId;

    fn toolbar() -> CommandChromeToolbar {
        CommandChromeToolbar::new().action(
            CommandChromeAction::new("format", "Format")
                .icon(CommandChromeIcon::EmphasisStrong.icon_props()),
        )
    }

    fn layout() -> FloatingCommandToolbarLayout {
        FloatingCommandToolbarLayout::new(
            Rect::new(20, 20, 8, 8),
            Size::new(40, 24),
            Rect::new(0, 0, 120, 120),
        )
    }

    #[test]
    fn new_toolbar_defaults_and_visibility_are_closed() {
        let toolbar = FloatingCommandToolbar::new(toolbar(), layout());

        assert_eq!(
            FloatingCommandToolbarVisibility::Closed,
            toolbar.visibility_model()
        );
        assert!(!toolbar.is_open());
        assert_eq!(2, toolbar.placement_priority.len());
        assert_eq!(Placement::BottomStart, toolbar.placement_priority[0]);
        assert!(toolbar.focus_return_target.is_none());
        assert!(toolbar.placement_model().is_none());
    }

    #[test]
    fn new_adapter_measured_keeps_panel_size_zero_until_measured() {
        let toolbar = FloatingCommandToolbar::new_adapter_measured(
            toolbar(),
            Rect::new(10, 10, 8, 8),
            Rect::new(0, 0, 120, 120),
        );

        assert_eq!(Size::new(0, 0), toolbar.layout_model().panel_size);
    }

    #[test]
    fn updates_family_and_mutators() {
        let toolbar = FloatingCommandToolbar::new(toolbar(), layout())
            .command_family(CommandChromeFamilyId::new("workspace"))
            .placement_priority(vec![Placement::TopStart])
            .clamp_margin(4)
            .focus_return_target(UiNodeId::new("surface"));

        assert_eq!("workspace", toolbar.command_family_id().as_str());
        assert_eq!(vec![Placement::TopStart], toolbar.placement_priority);
        assert_eq!(4, toolbar.clamp_margin);
        assert_eq!(Some(UiNodeId::new("surface")), toolbar.focus_return_target);
        assert_eq!(
            FloatingCommandToolbarVisibility::Closed,
            toolbar.visibility_model()
        );
    }

    #[test]
    fn initial_visibility_resolves_open_state() {
        let toolbar = FloatingCommandToolbar::new(toolbar(), layout())
            .initial_visibility(FloatingCommandToolbarVisibility::Visible);

        assert_eq!(
            FloatingCommandToolbarVisibility::Visible,
            toolbar.visibility_model()
        );
        assert!(toolbar.is_open());
        assert!(toolbar.placement_model().is_some());
        assert!(toolbar.bounds_model().is_some());
    }
}
