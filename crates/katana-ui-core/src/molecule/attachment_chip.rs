#[path = "attachment_chip/model.rs"]
mod model;
#[path = "attachment_chip/types.rs"]
mod types;

pub use model::AttachmentChip;
pub use types::{
    AttachmentChipAction, AttachmentChipEvent, AttachmentKind, AttachmentMeta, AttachmentProgress,
    AttachmentStatus, AttachmentThumbnail,
};
