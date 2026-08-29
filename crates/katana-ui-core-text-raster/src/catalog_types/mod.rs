mod policy;
mod resolver;
mod resolver_types;
#[cfg(test)]
mod tests;
mod types;

pub use policy::PlatformFontCatalogPolicy;
pub use resolver::PlatformColorEmojiFaceResolver;
pub use resolver_types::{
    PlatformColorEmojiAvailability, PlatformColorEmojiError, PlatformColorEmojiFaceRecord,
    PlatformColorEmojiUnavailableReason, PlatformEmojiFontLoadError, PlatformEmojiFontLoader,
    PlatformEmojiFontObservation, PlatformFontCatalogError,
};
pub use types::{
    PlatformEmojiFontCandidate, PlatformFontCatalogFingerprint, PlatformFontProfile,
    PlatformFontSha256,
};
