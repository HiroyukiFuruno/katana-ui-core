/// An opaque KUC-owned frame in the full-surface motion catalogue.
#[derive(Clone)]
pub struct FullTextCommandSurfaceMotionFrame {
    scenario_id: FullTextCommandSurfaceScenarioId,
    stage: FullTextCommandSurfaceRawInputStage,
    provenance_id: String,
    selection_transition: SelectionMotionTransition,
    find_transition: FindMotionTransition,
    dropdown_transition: DropdownMotionTransition,
}

impl FullTextCommandSurfaceMotionFrame {
    /// Returns the KUC-owned scenario identity required to retain the matching root lease.
    #[must_use]
    pub const fn scenario_id(&self) -> FullTextCommandSurfaceScenarioId {
        self.scenario_id
    }

    /// Applies the KUC-owned stage and any pending opaque continuation to the next frame.
    pub fn apply_to(
        &self,
        input: &mut egui::RawInput,
        continuation: &mut Option<KucOpaqueMotionContinuation>,
    ) -> Result<(), FullTextCommandSurfaceMotionPlanError> {
        self.stage.apply_to(input);
        if matches!(
            self.selection_transition,
            SelectionMotionTransition::Advance
        ) || matches!(self.find_transition, FindMotionTransition::Advance)
            || matches!(self.dropdown_transition, DropdownMotionTransition::Advance)
        {
            continuation
                .as_mut()
                .ok_or(FullTextCommandSurfaceMotionPlanError::MissingContinuation)?
                .apply_to_raw_input_once(input)
                .map_err(FullTextCommandSurfaceMotionPlanError::Continuation)?;
        }
        Ok(())
    }

    /// Captures the KUC-owned continuation required after this current root frame.
    pub fn capture_continuation(
        &self,
        locator: &KucInteractionLocator,
        continuation: &mut Option<KucOpaqueMotionContinuation>,
    ) -> Result<(), FullTextCommandSurfaceMotionPlanError> {
        match self.dropdown_transition {
            DropdownMotionTransition::None => {}
            DropdownMotionTransition::BeginTrigger | DropdownMotionTransition::BeginItem => {
                if continuation.is_some() {
                    return Err(FullTextCommandSurfaceMotionPlanError::UnexpectedContinuation);
                }
                let selector = match self.dropdown_transition {
                    DropdownMotionTransition::BeginTrigger => KucInteractionSelector::new(
                        "kuc.rich.block-code",
                        KucInteractionActionClass::DropdownTrigger,
                    ),
                    DropdownMotionTransition::BeginItem => KucInteractionSelector::new(
                        "kuc.generic-language-00",
                        KucInteractionActionClass::DropdownItem,
                    ),
                    DropdownMotionTransition::None | DropdownMotionTransition::Advance => {
                        return Err(FullTextCommandSurfaceMotionPlanError::InvalidTransition);
                    }
                };
                *continuation = Some(KucOpaqueMotionContinuation::click(
                    locator
                        .begin_click(selector)
                        .map_err(FullTextCommandSurfaceMotionPlanError::Dropdown)?,
                ));
                return Ok(());
            }
            DropdownMotionTransition::Advance => {
                let current = continuation
                    .take()
                    .ok_or(FullTextCommandSurfaceMotionPlanError::MissingContinuation)?;
                *continuation = current
                    .advance(locator)
                    .map_err(FullTextCommandSurfaceMotionPlanError::Continuation)?;
                return Ok(());
            }
        }
        match (self.selection_transition, self.find_transition) {
            (SelectionMotionTransition::None, FindMotionTransition::None) => {
                if continuation.is_some() {
                    return Err(FullTextCommandSurfaceMotionPlanError::UnexpectedContinuation);
                }
            }
            (SelectionMotionTransition::Begin, FindMotionTransition::None) => {
                if continuation.is_some() {
                    return Err(FullTextCommandSurfaceMotionPlanError::UnexpectedContinuation);
                }
                *continuation = Some(KucOpaqueMotionContinuation::selection(
                    locator
                        .begin_text_selection()
                        .map_err(FullTextCommandSurfaceMotionPlanError::Selection)?,
                ));
            }
            (SelectionMotionTransition::Advance, FindMotionTransition::None)
            | (SelectionMotionTransition::None, FindMotionTransition::Advance) => {
                let current = continuation
                    .take()
                    .ok_or(FullTextCommandSurfaceMotionPlanError::MissingContinuation)?;
                *continuation = current
                    .advance(locator)
                    .map_err(FullTextCommandSurfaceMotionPlanError::Continuation)?;
            }
            (SelectionMotionTransition::None, FindMotionTransition::Begin) => {
                if continuation.is_some() {
                    return Err(FullTextCommandSurfaceMotionPlanError::UnexpectedContinuation);
                }
                *continuation = Some(KucOpaqueMotionContinuation::search(
                    locator
                        .begin_search_trace()
                        .map_err(FullTextCommandSurfaceMotionPlanError::Search)?,
                ));
            }
            _ => return Err(FullTextCommandSurfaceMotionPlanError::InvalidTransition),
        }
        Ok(())
    }

    /// Returns the stable KUC-issued provenance for artifact manifests.
    #[must_use]
    pub fn provenance_id(&self) -> &str {
        &self.provenance_id
    }

    /// Returns the stage event count without disclosing the event payload.
    #[must_use]
    pub fn event_count(&self) -> usize {
        self.stage.event_count()
    }
}

impl std::fmt::Debug for FullTextCommandSurfaceMotionFrame {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("FullTextCommandSurfaceMotionFrame")
            .field("scenario_id", &self.scenario_id)
            .field("provenance_id", &self.provenance_id)
            .field("event_count", &self.event_count())
            .finish()
    }
}

/// A complete KUC-owned motion catalogue for the generic full-surface root.
#[derive(Debug)]
pub struct FullTextCommandSurfaceMotionPlan {
    frames: Vec<FullTextCommandSurfaceMotionFrame>,
}

impl FullTextCommandSurfaceMotionPlan {
    /// Issues exactly `requested_frames` KUC-defined frames or rejects an incomplete request.
    pub fn issue(requested_frames: usize) -> Result<Self, FullTextCommandSurfaceMotionPlanError> {
        let catalogue = motion_catalogue();
        let minimum = catalogue.len();
        if requested_frames < minimum || !requested_frames.is_multiple_of(minimum) {
            return Err(FullTextCommandSurfaceMotionPlanError::IncompleteCatalogue {
                requested: requested_frames,
                minimum,
            });
        }

        let frames = catalogue
            .iter()
            .cycle()
            .take(requested_frames)
            .enumerate()
            .map(
                |(
                    index,
                    (
                        scenario_id,
                        stage,
                        selection_transition,
                        find_transition,
                        dropdown_transition,
                    ),
                )| {
                    FullTextCommandSurfaceMotionFrame {
                        scenario_id: *scenario_id,
                        stage: stage.clone(),
                        provenance_id: format!("kuc-motion-{index:04}"),
                        selection_transition: *selection_transition,
                        find_transition: *find_transition,
                        dropdown_transition: *dropdown_transition,
                    }
                },
            )
            .collect();
        Ok(Self { frames })
    }

    /// Returns the catalogue's required minimum frame count.
    #[must_use]
    pub fn minimum_frame_count() -> usize {
        motion_catalogue().len()
    }

    /// Returns the complete ordered KUC-owned frame sequence.
    #[must_use]
    pub fn frames(&self) -> &[FullTextCommandSurfaceMotionFrame] {
        &self.frames
    }
}

/// Errors while issuing a KUC-owned motion plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FullTextCommandSurfaceMotionPlanError {
    IncompleteCatalogue { requested: usize, minimum: usize },
    MissingContinuation,
    UnexpectedContinuation,
    InvalidTransition,
    Selection(KucTextSelectionContinuationError),
    Search(KucSearchTraceContinuationError),
    Dropdown(KucInteractionLocatorError),
    Continuation(KucOpaqueMotionContinuationError),
}

impl std::fmt::Display for FullTextCommandSurfaceMotionPlanError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IncompleteCatalogue { requested, minimum } => write!(
                formatter,
                "requested {requested} motion frames cannot cover the required {minimum}-frame KUC catalogue"
            ),
            Self::MissingContinuation => {
                formatter.write_str("KUC motion frame requires a missing selection continuation")
            }
            Self::UnexpectedContinuation => formatter
                .write_str("KUC motion frame received an unexpected selection continuation"),
            Self::InvalidTransition => {
                formatter.write_str("KUC motion frame combines incompatible continuation phases")
            }
            Self::Selection(error) => {
                write!(formatter, "KUC selection continuation failed: {error}")
            }
            Self::Search(error) => write!(formatter, "KUC search trace failed: {error}"),
            Self::Dropdown(error) => write!(formatter, "KUC dropdown trace failed: {error}"),
            Self::Continuation(error) => {
                write!(formatter, "KUC motion continuation failed: {error}")
            }
        }
    }
}

impl std::error::Error for FullTextCommandSurfaceMotionPlanError {}
