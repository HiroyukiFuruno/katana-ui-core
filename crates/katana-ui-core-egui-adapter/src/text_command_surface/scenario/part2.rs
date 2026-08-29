/// A scenario lease plus its deterministic input stages.
pub struct FullTextCommandSurfaceScenario {
    id: FullTextCommandSurfaceScenarioId,
    lease: Option<EguiTextCommandSurfaceHostProjectionLease>,
    stages: Vec<FullTextCommandSurfaceRawInputStage>,
}

impl FullTextCommandSurfaceScenario {
    /// Returns the stable scenario ID.
    #[must_use]
    pub const fn id(&self) -> FullTextCommandSurfaceScenarioId {
        self.id
    }

    /// Consumes the scenario and returns its opaque host lease.
    pub fn into_lease(
        mut self,
    ) -> Result<EguiTextCommandSurfaceHostProjectionLease, FullTextCommandSurfaceScenarioError>
    {
        self.lease
            .take()
            .ok_or(FullTextCommandSurfaceScenarioError::LeaseConsumed)
    }

    /// Returns deterministic stages in their KUC-defined order.
    #[must_use]
    pub fn stages(&self) -> &[FullTextCommandSurfaceRawInputStage] {
        &self.stages
    }
}

/// Errors while issuing a generic scenario.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FullTextCommandSurfaceScenarioError {
    LeaseConsumed,
    InvalidProjection,
    RevisionExhausted,
}

impl std::fmt::Display for FullTextCommandSurfaceScenarioError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LeaseConsumed => formatter.write_str("scenario lease was already consumed"),
            Self::InvalidProjection => formatter.write_str("scenario projection is invalid"),
            Self::RevisionExhausted => {
                formatter.write_str("scenario session revision is exhausted")
            }
        }
    }
}

impl std::error::Error for FullTextCommandSurfaceScenarioError {}

impl std::error::Error for KucOpaqueMotionContinuationError {}

impl std::fmt::Debug for KucOpaqueMotionContinuation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("KucOpaqueMotionContinuation(..)")
    }
}

impl KucOpaqueMotionContinuation {
    fn selection(value: KucOpaqueTextSelectionContinuation) -> Self {
        Self {
            state: KucOpaqueMotionContinuationState::Selection(value),
        }
    }

    fn search(value: KucOpaqueSearchTraceContinuation) -> Self {
        Self {
            state: KucOpaqueMotionContinuationState::Search(value),
        }
    }

    fn click(value: KucOpaqueClickContinuation) -> Self {
        Self {
            state: KucOpaqueMotionContinuationState::Click(value),
        }
    }

    fn apply_to_raw_input_once(
        &mut self,
        input: &mut egui::RawInput,
    ) -> Result<(), KucOpaqueMotionContinuationError> {
        match &mut self.state {
            KucOpaqueMotionContinuationState::Selection(value) => value
                .apply_to_raw_input_once(input)
                .map_err(KucOpaqueMotionContinuationError::Selection),
            KucOpaqueMotionContinuationState::Search(value) => value
                .apply_to_raw_input_once(input)
                .map_err(KucOpaqueMotionContinuationError::Search),
            KucOpaqueMotionContinuationState::Click(value) => value
                .apply_to_raw_input_once(input)
                .map_err(KucOpaqueMotionContinuationError::Click),
        }
    }

    fn advance(
        self,
        locator: &KucInteractionLocator,
    ) -> Result<Option<Self>, KucOpaqueMotionContinuationError> {
        match self.state {
            KucOpaqueMotionContinuationState::Selection(value) => value
                .advance(locator)
                .map(|next| next.map(Self::selection))
                .map_err(KucOpaqueMotionContinuationError::Selection),
            KucOpaqueMotionContinuationState::Search(value) => value
                .advance(locator)
                .map(|next| next.map(Self::search))
                .map_err(KucOpaqueMotionContinuationError::Search),
            KucOpaqueMotionContinuationState::Click(value) => value
                .advance(locator)
                .map(|next| next.map(Self::click))
                .map_err(KucOpaqueMotionContinuationError::Click),
        }
    }
}
