use super::{TabStripOverlayState, TabStripRetainedError, TabStripRetainedState, TabStripText};

impl TabStripRetainedState {
    pub(super) fn flush_rename_start(&mut self) -> Result<(), TabStripRetainedError> {
        let Some(mut pending) = self.pending_rename_start.take() else {
            return Ok(());
        };
        if pending.commit.is_discarded() {
            self.inline_rename_consumed
                .retain(|target| target != &pending.target);
            self.overlay = TabStripOverlayState::Closed;
            return Ok(());
        }
        if !pending.commit.is_committed() {
            self.pending_rename_start = Some(pending);
            return Ok(());
        }
        let valid =
            valid_rename_path(&self.projection, &pending.target, &pending.correlation).is_some();
        if !valid {
            self.inline_rename_consumed
                .retain(|target| target != &pending.target);
            self.overlay = TabStripOverlayState::Closed;
            return Ok(());
        }
        {
            pending
                .port
                .forward_inline_rename_started(
                    super::tab_strip_proposal_port::TabStripInlineRenameStarted::new(
                        pending.target,
                        pending.correlation,
                    ),
                )
                .map_err(TabStripRetainedError::Port)?;
        }
        Ok(())
    }

    pub(super) fn flush_rename_changes(&mut self) -> Result<(), TabStripRetainedError> {
        let Some(mut pending) = self.pending_rename_changes.take() else {
            return Ok(());
        };
        if pending.commit.is_discarded() {
            let path = valid_rename_path(&self.projection, &pending.target, &pending.correlation);
            self.overlay = match (pending.before, path) {
                (
                    Some(TabStripOverlayState::GroupPopup {
                        anchor,
                        submenu_path,
                        rename,
                        ..
                    }),
                    Some(path),
                ) => TabStripOverlayState::GroupPopup {
                    path,
                    anchor,
                    submenu_path,
                    rename,
                },
                _ => TabStripOverlayState::Closed,
            };
            return Ok(());
        }
        if !pending.commit.is_committed() {
            self.pending_rename_changes = Some(pending);
            return Ok(());
        }
        let valid =
            valid_rename_path(&self.projection, &pending.target, &pending.correlation).is_some();
        if valid {
            for name in pending.names {
                self.next_nonce = self.next_nonce.saturating_add(1);
                let operation = super::TabStripProposalOperation::RenameGroup {
                    group: pending.target.clone(),
                    name,
                };
                let proposal = super::TabStripProposal::new(
                    self.next_nonce,
                    pending.correlation.clone(),
                    operation,
                )
                .with_origin(super::tab_strip_proposal_port::TabStripProposalOrigin::GroupPopup);
                pending
                    .port
                    .forward_once(proposal)
                    .map_err(TabStripRetainedError::Port)?;
            }
        }
        Ok(())
    }
    pub(super) fn forward_rename_route(
        &mut self,
        path: &str,
        name: TabStripText,
    ) -> Result<(), TabStripRetainedError> {
        let (correlation, operation) = self
            .routes
            .rename_proposal_for(path, name)
            .ok_or(TabStripRetainedError::MissingRoute)?;
        self.forward_proposal_with_origin(
            correlation,
            operation,
            super::tab_strip_proposal_port::TabStripProposalOrigin::GroupPopup,
        )
    }
}

fn valid_rename_path(
    projection: &super::TabStripProjection,
    target: &super::TabStripGroupTarget,
    correlation: &super::tab_strip_projection_lease::TabStripCorrelation,
) -> Option<String> {
    if &projection.correlation != correlation {
        return None;
    }
    let path = super::support::group_path_for_target(projection, target)?;
    let group = super::support::group_for_path(projection, &path)?;
    if group.capabilities.renamable
        && group
            .popup
            .as_ref()
            .is_some_and(|popup| popup.rename_placeholder.is_some())
    {
        Some(path)
    } else {
        None
    }
}
