//! 保持中のtab-strip rootからhostへ型付き操作を渡す終端。
//!
//! show中の操作通知とroot-frameのevent batchは別の経路とする。
//! proposalごとのnonceで一回限りの配送を管理し、後続の正当な操作を抑止しない。

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
