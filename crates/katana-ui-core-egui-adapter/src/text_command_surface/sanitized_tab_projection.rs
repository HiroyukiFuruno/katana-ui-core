mod model {
    include!("sanitized_tab_projection/types.rs");
    include!("sanitized_tab_projection/logic_a.rs");
    include!("sanitized_tab_projection/logic_b.rs");
}

pub use model::{
    SanitizedTab, SanitizedTabCapabilities, SanitizedTabClosePresentation, SanitizedTabGroup,
    SanitizedTabGroupTarget, SanitizedTabProjection, SanitizedTabTarget,
};
#[cfg(test)]
pub type SanitizedTabGroupCapabilities = model::SanitizedTabGroupCapabilities;

#[cfg(test)]
mod tests {
    include!("sanitized_tab_projection/tests.rs");
}
