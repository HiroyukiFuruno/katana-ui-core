use super::targets::{
    TEXT_SURFACE_CONTEXT_TARGET_ID, append_context_menu_targets, append_generic_targets,
    append_search_targets, append_search_text_input_target, append_text_surface_context_target,
    append_toolbar_targets, center, overlapping_enabled_bounds,
};
use super::types::{
    KucInteractionActionClass, KucInteractionLocator, KucInteractionLocatorError,
    KucInteractionRequestError, KucInteractionSelector, KucOpaqueClickContinuation,
    KucOpaqueInteractionRequest, KucOpaqueSearchTraceContinuation,
    KucOpaqueTextSelectionContinuation, KucSearchTraceContinuationError,
    KucTextSelectionContinuationError, OpaqueClickPhase, SearchTracePhase, TextSelectionGeometry,
    TextSelectionPhase,
};
use super::{
    BoundAccessKitEvidence, EguiTextCommandSurfaceOutput, HashSet, KucRootEventBatchContext,
    RefCell,
};

impl KucInteractionLocator {
    #[must_use]
    pub const fn state_revision(&self) -> u64 {
        self.state_revision
    }

    pub(crate) fn from_output(
        root_identity: &str,
        context: &KucRootEventBatchContext,
        frame_serial: u64,
        output: &EguiTextCommandSurfaceOutput,
        bound_evidence: &BoundAccessKitEvidence,
    ) -> Self {
        let mut targets = Vec::new();
        let mut hidden = HashSet::new();
        let evidence = bound_evidence.matching_entries(context, root_identity);
        if let Some(toolbar) = output.toolbar.as_ref() {
            append_toolbar_targets(
                &mut targets,
                &toolbar.record,
                KucInteractionActionClass::Toolbar,
                evidence,
            );
            for item_id in &toolbar.record.hidden_item_ids {
                hidden.insert((item_id.clone(), KucInteractionActionClass::DropdownItem));
            }
        }
        if let Some(floating) = output
            .floating
            .as_ref()
            .and_then(|value| value.record.as_ref())
        {
            append_toolbar_targets(
                &mut targets,
                &floating.toolbar,
                KucInteractionActionClass::FloatingToolbar,
                evidence,
            );
        }
        let search_visible = output.search.is_some();
        let search_query_focused = output.search.as_ref().is_some_and(|search| {
            search.record.focused_target.as_deref() == Some(search.record.query.hit_target.as_str())
        });
        if let Some(search) = output.search.as_ref() {
            append_search_targets(&mut targets, &search.record, evidence);
            append_search_text_input_target(&mut targets, &search.record, evidence);
        }
        if output.context_menu.is_some()
            && output
                .context_menu
                .as_ref()
                .is_none_or(|value| value.record.is_none())
        {
            append_text_surface_context_target(&mut targets, evidence);
        }
        if let Some(menu) = output
            .context_menu
            .as_ref()
            .and_then(|value| value.record.as_ref())
        {
            append_context_menu_targets(&mut targets, menu, evidence);
        }
        append_generic_targets(&mut targets, evidence);
        /* WHY: A current bound AccessKit target is authoritative over stale hidden metadata. */
        for target in &targets {
            hidden.remove(&(target.action_identity.clone(), target.action_class));
        }
        Self {
            root_identity: root_identity.to_owned(),
            state_revision: context.state_revision(),
            frame_serial,
            correlation_fingerprint: context.correlation_fingerprint().to_owned(),
            ambiguous_bounds: overlapping_enabled_bounds(&targets),
            targets,
            hidden,
            requested: RefCell::new(HashSet::new()),
            selection_geometry: TextSelectionGeometry::from_output(output),
            selection_established: !output.text.record.frame.selection.range.is_collapsed(),
            floating_visible: output
                .floating
                .as_ref()
                .and_then(|floating| floating.record.as_ref())
                .is_some(),
            search_visible,
            search_query_focused,
        }
    }

    /// Starts a KUC-owned multi-frame pointer selection from this exact root frame.
    pub fn begin_text_selection(
        &self,
    ) -> Result<KucOpaqueTextSelectionContinuation, KucTextSelectionContinuationError> {
        let geometry = self
            .selection_geometry
            .ok_or(KucTextSelectionContinuationError::Unavailable)?;
        Ok(KucOpaqueTextSelectionContinuation {
            root_identity: self.root_identity.clone(),
            frame_serial: self.frame_serial,
            geometry,
            phase: TextSelectionPhase::Aim,
            applied: false,
        })
    }

    /// Starts the KUC-owned generic search trace from the current query input target.
    pub fn begin_search_trace(
        &self,
    ) -> Result<KucOpaqueSearchTraceContinuation, KucSearchTraceContinuationError> {
        let query = self
            .targets
            .iter()
            .filter(|target| target.action_class == KucInteractionActionClass::TextInput)
            .map(|target| target.action_identity.clone())
            .collect::<Vec<_>>();
        let [query] = query.as_slice() else {
            return Err(KucSearchTraceContinuationError::Unavailable);
        };
        let request = self
            .request(KucInteractionSelector::new(
                query.clone(),
                KucInteractionActionClass::TextInput,
            ))
            .map_err(KucSearchTraceContinuationError::Interaction)?;
        Ok(KucOpaqueSearchTraceContinuation {
            root_identity: self.root_identity.clone(),
            frame_serial: self.frame_serial,
            phase: SearchTracePhase::Focus(request),
            applied: false,
        })
    }

    pub fn request(
        &self,
        selector: KucInteractionSelector,
    ) -> Result<KucOpaqueInteractionRequest, KucInteractionLocatorError> {
        let key = (selector.action_identity.clone(), selector.action_class);
        if !self.requested.borrow_mut().insert(key.clone()) {
            return Err(KucInteractionLocatorError::Duplicate);
        }
        if self.hidden.contains(&key) {
            return Err(KucInteractionLocatorError::Hidden);
        }
        let mut selected = None;
        let mut duplicate = false;
        for target in self.targets.iter().filter(|target| {
            target.action_identity == selector.action_identity
                && target.action_class == selector.action_class
        }) {
            if selected.is_some() {
                duplicate = true;
                break;
            }
            selected = Some(target);
        }
        let target = selected.ok_or(KucInteractionLocatorError::Missing)?;
        if duplicate
            || (!target.disabled && self.ambiguous_bounds.contains(&target.evidence.bounds))
        {
            return Err(KucInteractionLocatorError::Ambiguous);
        }
        if target.disabled {
            return Err(KucInteractionLocatorError::Disabled);
        }
        let point = center(target.evidence.bounds);
        let modifiers = egui::Modifiers::default();
        Ok(KucOpaqueInteractionRequest {
            root_identity: self.root_identity.clone(),
            state_revision: self.state_revision,
            correlation_fingerprint: self.correlation_fingerprint.clone(),
            events: vec![
                egui::Event::PointerMoved(point),
                egui::Event::PointerButton {
                    pos: point,
                    button: if selector.action_class
                        == KucInteractionActionClass::TextSurfaceContextTarget
                    {
                        egui::PointerButton::Secondary
                    } else {
                        egui::PointerButton::Primary
                    },
                    pressed: true,
                    modifiers,
                },
                egui::Event::PointerButton {
                    pos: point,
                    button: if selector.action_class
                        == KucInteractionActionClass::TextSurfaceContextTarget
                    {
                        egui::PointerButton::Secondary
                    } else {
                        egui::PointerButton::Primary
                    },
                    pressed: false,
                    modifiers,
                },
            ],
            queued: false,
        })
    }

    pub fn begin_click(
        &self,
        selector: KucInteractionSelector,
    ) -> Result<KucOpaqueClickContinuation, KucInteractionLocatorError> {
        let request = self.request(selector.clone())?;
        let event = self.click_event(&selector, OpaqueClickPhase::Aim)?;
        Ok(KucOpaqueClickContinuation {
            root_identity: request.root_identity,
            frame_serial: self.frame_serial,
            selector,
            event,
            phase: OpaqueClickPhase::Aim,
            applied: false,
        })
    }

    /// Requests the generic physical context-menu opener for this frame's text surface.
    ///
    /// The target identity and geometry are resolved and retained inside KUC. The caller only
    /// receives an opaque request that can be applied to the next root input frame.
    pub fn request_context_open(
        &self,
    ) -> Result<KucOpaqueInteractionRequest, KucInteractionLocatorError> {
        self.request(KucInteractionSelector::new(
            TEXT_SURFACE_CONTEXT_TARGET_ID,
            KucInteractionActionClass::TextSurfaceContextTarget,
        ))
    }

    pub fn queue_request(
        &self,
        mut request: KucOpaqueInteractionRequest,
        input: &mut egui::RawInput,
    ) -> Result<(), KucInteractionRequestError> {
        if request.root_identity != self.root_identity {
            return Err(KucInteractionRequestError::RootMismatch);
        }
        if request.state_revision != self.state_revision
            || request.correlation_fingerprint != self.correlation_fingerprint
        {
            return Err(KucInteractionRequestError::Stale);
        }
        request.apply_to_raw_input_once(input)
    }
}
