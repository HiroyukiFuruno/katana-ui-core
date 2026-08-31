use super::super::targets::{accesskit_class, evidence_for};
use super::super::types::{KucInteractionActionClass, LocatorTarget, TextSelectionGeometry};
use super::super::{
    AccessKitEvidence, HashSet, KucInteractionLocator, KucInteractionSelector, RefCell,
};

pub(crate) const ZERO_I32: i32 = 0;
pub(crate) const ZERO_F32: f32 = 0.0;

pub(crate) const ONE_U64: u64 = 1;

pub(crate) const FRAME_STEP_ONE: u64 = ONE_U64;
pub(crate) const FRAME_STEP_TWO: u64 = FRAME_STEP_ONE + FRAME_STEP_ONE;
pub(crate) const FRAME_STEP_THREE: u64 = FRAME_STEP_TWO + FRAME_STEP_ONE;
pub(crate) const FRAME_STEP_FOUR: u64 = FRAME_STEP_TWO + FRAME_STEP_TWO;
pub(crate) const FRAME_STEP_FIVE: u64 = FRAME_STEP_THREE + FRAME_STEP_TWO;
pub(crate) const FRAME_STEP_SIX: u64 = FRAME_STEP_THREE + FRAME_STEP_THREE;
pub(crate) const FRAME_STEP_SEVEN: u64 = FRAME_STEP_FIVE + FRAME_STEP_TWO;
pub(crate) const FRAME_STEP_EIGHT: u64 = FRAME_STEP_FOUR + FRAME_STEP_FOUR;
pub(crate) const FRAME_STEP_NINE: u64 = FRAME_STEP_FIVE + FRAME_STEP_FOUR;
pub(crate) const FRAME_STEP_TEN: u64 = FRAME_STEP_FIVE + FRAME_STEP_FIVE;
pub(crate) const FRAME_STEP_ELEVEN: u64 = FRAME_STEP_TEN + FRAME_STEP_ONE;

pub(crate) const REQUEST_EVENT_COUNT_THREE: usize = FRAME_STEP_THREE as usize;
pub(crate) const CLICK_EVENT_COUNT_THREE: usize = REQUEST_EVENT_COUNT_THREE;
pub(crate) const KUC_FRAME_STEP_ONE: u64 = FRAME_STEP_ONE;

pub(crate) const KUC_TEXT_SELECTION_FRAME_START: u64 = FRAME_STEP_FIVE;
pub(crate) const KUC_TEXT_SELECTION_FRAME_SECOND: u64 = FRAME_STEP_SIX;
pub(crate) const KUC_TEXT_SELECTION_FRAME_THIRD: u64 = FRAME_STEP_SEVEN;
pub(crate) const KUC_TEXT_SELECTION_FRAME_FOURTH: u64 = FRAME_STEP_EIGHT;
pub(crate) const KUC_TEXT_SELECTION_FRAME_FIFTH: u64 = FRAME_STEP_NINE;

pub(crate) const KUC_SEARCH_TRACE_FRAME_QUERY: u64 = FRAME_STEP_ONE;
pub(crate) const KUC_SEARCH_TRACE_FRAME_FOCUS: u64 = FRAME_STEP_TWO;
pub(crate) const KUC_SEARCH_TRACE_FRAME_PREEDIT: u64 = FRAME_STEP_THREE;
pub(crate) const KUC_SEARCH_TRACE_FRAME_COMMIT: u64 = FRAME_STEP_FOUR;
pub(crate) const KUC_SEARCH_TRACE_FRAME_NEXT: u64 = FRAME_STEP_FIVE;
pub(crate) const KUC_SEARCH_TRACE_FRAME_PREVIOUS: u64 = FRAME_STEP_SIX;
pub(crate) const KUC_SEARCH_TRACE_FRAME_CLOSE: u64 = FRAME_STEP_SEVEN;
pub(crate) const KUC_SEARCH_TRACE_FRAME_VERIFY: u64 = FRAME_STEP_EIGHT;

pub(crate) const ONE_F32: f32 = 1.0;
pub(crate) const TWO_F32: f32 = ONE_F32 + ONE_F32;
pub(crate) const THREE_F32: f32 = TWO_F32 + ONE_F32;
pub(crate) const FIVE_F32: f32 = TWO_F32 + THREE_F32;

pub(crate) const TEXT_SELECTION_START_X: f32 = ONE_F32;
pub(crate) const TEXT_SELECTION_MID_X: f32 = THREE_F32;
pub(crate) const TEXT_SELECTION_END_X: f32 = FIVE_F32;
pub(crate) const TEXT_SELECTION_Y: f32 = ZERO_F32;

pub(crate) const CLICK_FRAME_SOURCE: u64 = FRAME_STEP_TEN;
pub(crate) const CLICK_FRAME_PRESS: u64 = FRAME_STEP_ELEVEN;
pub(crate) const CLICK_FRAME_RELEASE: u64 = FRAME_STEP_TEN + FRAME_STEP_TWO;
pub(crate) const KUC_LOCATOR_REQUEST_REVISION: u64 = FRAME_STEP_FOUR;
pub(crate) const KUC_LOCATOR_OWNER_FRAME: u64 = FRAME_STEP_NINE;
pub(crate) const KUC_LOCATOR_STALE_FRAME: u64 = FRAME_STEP_EIGHT;
pub(crate) const TEST_BOUNDS_SIZE_PX: u32 = 10;

pub(crate) fn locator(
    root: &str,
    revision: u64,
    targets: Vec<LocatorTarget>,
) -> KucInteractionLocator {
    KucInteractionLocator {
        root_identity: root.to_owned(),
        state_revision: revision,
        frame_serial: revision,
        correlation_fingerprint: format!("correlation-{revision}"),
        targets,
        ambiguous_bounds: Vec::new(),
        hidden: HashSet::new(),
        requested: RefCell::new(HashSet::new()),
        selection_geometry: None,
        selection_established: false,
        floating_visible: false,
        search_visible: false,
        search_query_focused: false,
    }
}

pub(crate) fn locator_for_continue(
    root: &str,
    revision: u64,
    targets: Vec<LocatorTarget>,
    search_visible: bool,
    search_query_focused: bool,
    selection_established: bool,
    floating_visible: bool,
) -> KucInteractionLocator {
    KucInteractionLocator {
        root_identity: root.to_owned(),
        state_revision: revision,
        frame_serial: revision,
        correlation_fingerprint: format!("correlation-{revision}"),
        targets,
        ambiguous_bounds: Vec::new(),
        hidden: HashSet::new(),
        requested: RefCell::new(HashSet::new()),
        selection_geometry: Some(TextSelectionGeometry {
            start: egui::pos2(TEXT_SELECTION_START_X, TEXT_SELECTION_Y),
            midpoint: egui::pos2(TEXT_SELECTION_MID_X, TEXT_SELECTION_Y),
            end: egui::pos2(TEXT_SELECTION_END_X, TEXT_SELECTION_Y),
        }),
        selection_established,
        floating_visible,
        search_visible,
        search_query_focused,
    }
}

pub(crate) fn click_geometry_locator(root: &str, revision: u64) -> KucInteractionLocator {
    locator_for_continue(root, revision, Vec::new(), false, false, true, false)
}

pub(crate) fn text_selection_locator_for_continue(
    root: &str,
    revision: u64,
) -> KucInteractionLocator {
    locator_for_continue(root, revision, Vec::new(), false, false, true, true)
}

pub(crate) fn search_locator_for_continue(
    root: &str,
    revision: u64,
    search_query_focused: bool,
    search_visible: bool,
) -> KucInteractionLocator {
    locator_for_continue(
        root,
        revision,
        vec![
            target("search-query", KucInteractionActionClass::TextInput, false),
            target(
                "search:next",
                KucInteractionActionClass::SearchControl,
                false,
            ),
            target(
                "search:previous",
                KucInteractionActionClass::SearchControl,
                false,
            ),
            target(
                "search:close",
                KucInteractionActionClass::SearchControl,
                false,
            ),
        ],
        search_visible,
        search_query_focused,
        true,
        true,
    )
}

pub(crate) fn target(id: &str, class: KucInteractionActionClass, disabled: bool) -> LocatorTarget {
    LocatorTarget {
        action_identity: id.to_owned(),
        action_class: class,
        disabled,
        evidence: AccessKitEvidence {
            response_id: egui::Id::new(("test", id)),
            bounds: katana_ui_core::render_model::UiRect::new(
                ZERO_I32,
                ZERO_I32,
                TEST_BOUNDS_SIZE_PX,
                TEST_BOUNDS_SIZE_PX,
            ),
            label: id.to_owned(),
            disabled,
            target_identity: id.to_owned(),
            target_class: accesskit_class(class),
        },
    }
}

pub(crate) fn click_selector(id: &str, class: KucInteractionActionClass) -> KucInteractionSelector {
    KucInteractionSelector::new(id.to_owned(), class)
}

pub(crate) fn evidence_for_toolbar_match(
    evidence: &[AccessKitEvidence],
    target: &str,
) -> AccessKitEvidence {
    evidence_for(evidence, target, KucInteractionActionClass::Toolbar, false)
        .expect("target exists for toolbar")
}

pub(crate) fn search_text_geometry() -> TextSelectionGeometry {
    TextSelectionGeometry {
        start: egui::pos2(TEXT_SELECTION_START_X, TEXT_SELECTION_Y),
        midpoint: egui::pos2(TEXT_SELECTION_MID_X, TEXT_SELECTION_Y),
        end: egui::pos2(TEXT_SELECTION_END_X, TEXT_SELECTION_Y),
    }
}

pub(crate) fn search_text_geometry_points() -> TextSelectionGeometry {
    search_text_geometry()
}
