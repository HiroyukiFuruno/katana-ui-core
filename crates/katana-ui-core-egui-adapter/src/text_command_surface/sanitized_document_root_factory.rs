mod sanitized_document_root_factory_error;
mod sanitized_document_root_factory_factory;
mod sanitized_document_root_factory_frame;

pub use sanitized_document_root_factory_error::SanitizedDocumentRootFactoryError;
pub use sanitized_document_root_factory_factory::{
    SanitizedDocumentRoot, SanitizedDocumentRootFactory,
};
pub use sanitized_document_root_factory_frame::SanitizedDocumentRootFrame;

#[cfg(test)]
mod tests {
    include!("sanitized_document_root_factory_tests.rs");
}
