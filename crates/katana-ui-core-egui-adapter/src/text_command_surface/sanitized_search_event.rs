use super::sanitized_projection_hash::SanitizedProjectionHash;
const SHA256_BYTES: usize = SanitizedProjectionHash::SHA256_BYTES;

include!("sanitized_search_event.rs_body.inc");
