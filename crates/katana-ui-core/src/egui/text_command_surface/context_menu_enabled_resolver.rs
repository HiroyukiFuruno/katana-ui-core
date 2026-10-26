#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KucContextMenuEnabledResolverError;

impl std::fmt::Display for KucContextMenuEnabledResolverError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("context-menu enabled resolver failed")
    }
}

impl std::error::Error for KucContextMenuEnabledResolverError {}

pub(crate) type ContextMenuEnabledResolver =
    Box<dyn FnMut(&str, bool) -> Result<bool, KucContextMenuEnabledResolverError>>;

#[cfg(test)]
#[path = "context_menu_enabled_resolver_tests.rs"]
mod tests;
