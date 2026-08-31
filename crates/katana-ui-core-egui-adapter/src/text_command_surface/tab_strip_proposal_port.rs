//! Crate-private, proposal-specific transport for the retained tab-strip root.
//!
//! This is deliberately separate from the public root-frame event batch. A
//! proposal is consumed at most once by its own nonce; the handle never has a
//! root-global consumed flag that could suppress a later valid interaction.

mod model {
    include!("tab_strip_proposal_port/model.rs");
}

mod transport {
    include!("tab_strip_proposal_port/transport.rs");
}

#[cfg(test)]
pub(crate) use model::TabStripProposalOperationClass;
pub use model::{
    TabStripGroupPlacement, TabStripProposal, TabStripProposalOperation, TabStripTabPlacement,
};
pub use transport::{TabStripProposalPort, TabStripProposalPortError, TabStripProposalPortHandle};

#[cfg(test)]
mod tests {
    include!("tab_strip_proposal_port/tests.rs");
}
