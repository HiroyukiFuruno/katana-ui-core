mod model {
    include!("tab_strip_projection_lease/types.rs");
    include!("tab_strip_projection_lease/logic_a.rs");
    include!("tab_strip_projection_lease/logic_b.rs");
    include!("tab_strip_projection_lease/logic_c.rs");
    include!("tab_strip_projection_lease/logic_d.rs");
}

pub use model::{
    TabStripContextMenuPresentation, TabStripControlPresentation, TabStripCorrelation,
    TabStripGroupCapabilities, TabStripGroupDescriptor, TabStripGroupPopupPresentation,
    TabStripGroupTarget, TabStripMenuEntry, TabStripMenuOperation, TabStripNavigationPresentation,
    TabStripProjection, TabStripProjectionLease, TabStripScrollPresentation,
    TabStripSurfaceCapabilities, TabStripSwatchDescriptor, TabStripSwatchTarget,
    TabStripTabCapabilities, TabStripTabDescriptor, TabStripTabTarget, TabStripText,
};

#[cfg(test)]
mod tests {
    include!("tab_strip_projection_lease/tests.rs");
}
