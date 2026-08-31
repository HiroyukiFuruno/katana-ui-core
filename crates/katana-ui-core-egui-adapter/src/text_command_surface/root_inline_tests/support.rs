use super::*;
use crate::context_menu::{ContextMenuPresentation, ContextMenuPresentationItem};
use katana_ui_core::atom::{TextArea, TextAreaAction, TextAreaCompositionPhase, TextAreaSelection};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeAction, CommandChromeFamilyId, CommandChromeToolbar,
    FloatingCommandToolbarVisibility,
};
use katana_ui_core::molecule::command_chrome::{
    CommandChromeSearchStrip, CommandChromeText, SearchControlStrings, SearchResultSummaryTemplate,
};
use katana_ui_core::molecule::structured::SearchControlStrip;
use katana_ui_core::molecule::structured::source_address_strip::{
    SourceAddressPresentation, SourceAddressStrip,
};
use katana_ui_core::text_surface::TextSurfaceAction;
use katana_ui_core::text_surface::{
    TextSurface, TextSurfaceAutomaticGutterPresentation, TextSurfaceProps, TextSurfaceViewport,
};
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

struct CountingTabStripPort(Rc<Cell<usize>>);

impl super::super::tab_strip_proposal_port::TabStripProposalPort for CountingTabStripPort {
    fn forward_proposal(
        &mut self,
        proposal: super::super::tab_strip_proposal_port::TabStripProposal,
    ) -> Result<(), super::super::tab_strip_proposal_port::TabStripProposalPortError> {
        proposal.consume_for_port();
        self.0.set(self.0.get().saturating_add(1));
        Ok(())
    }
}

struct NavigationTabStripPort(Rc<RefCell<Vec<bool>>>);

impl super::super::tab_strip_proposal_port::TabStripProposalPort for NavigationTabStripPort {
    fn forward_proposal(
        &mut self,
        proposal: super::super::tab_strip_proposal_port::TabStripProposal,
    ) -> Result<(), super::super::tab_strip_proposal_port::TabStripProposalPortError> {
        let direction = proposal
            .navigation_direction_for_test()
            .expect("navigation renderer must forward only a navigation proposal");
        proposal.consume_for_port();
        self.0.borrow_mut().push(direction);
        Ok(())
    }
}

struct GroupCollapseTabStripPort(Rc<RefCell<Vec<bool>>>);

impl super::super::tab_strip_proposal_port::TabStripProposalPort for GroupCollapseTabStripPort {
    fn forward_proposal(
        &mut self,
        proposal: super::super::tab_strip_proposal_port::TabStripProposal,
    ) -> Result<(), super::super::tab_strip_proposal_port::TabStripProposalPortError> {
        let collapsed = proposal
            .group_collapsed_for_test()
            .expect("group header must forward only a collapse proposal");
        proposal.consume_for_port();
        self.0.borrow_mut().push(collapsed);
        Ok(())
    }
}

struct TrailingTabStripPort(
    Rc<RefCell<Vec<super::super::tab_strip_proposal_port::TabStripProposalOperationClass>>>,
);

impl super::super::tab_strip_proposal_port::TabStripProposalPort for TrailingTabStripPort {
    fn forward_proposal(
        &mut self,
        proposal: super::super::tab_strip_proposal_port::TabStripProposal,
    ) -> Result<(), super::super::tab_strip_proposal_port::TabStripProposalPortError> {
        let class = proposal.operation_class_for_test();
        proposal.consume_for_port();
        self.0.borrow_mut().push(class);
        Ok(())
    }
}
