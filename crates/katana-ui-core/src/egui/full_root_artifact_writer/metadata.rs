pub(super) use super::metadata_types::{
    FrameRootMetadata, FullRootArtifact, FullRootArtifactError, FullRootArtifactManifest,
};
use std::path::Path;
#[cfg(test)]
use std::path::PathBuf;

impl FullRootArtifact {
    pub(super) fn from_metadata(metadata: FrameRootMetadata<'_>, png_sha256: String) -> Self {
        Self {
            stage_id: metadata.stage_id.to_owned(),
            png_path: metadata.png_path,
            manifest_path: metadata.manifest_path,
            width: metadata.width,
            height: metadata.height,
            root_record_hash: metadata.root_record_hash.to_owned(),
            pixel_hash: metadata.pixel_hash,
            png_sha256,
        }
    }

    #[must_use]
    pub fn stage_id(&self) -> &str {
        &self.stage_id
    }

    #[must_use]
    pub fn png_path(&self) -> &Path {
        &self.png_path
    }

    #[must_use]
    pub fn manifest_path(&self) -> &Path {
        &self.manifest_path
    }

    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    #[must_use]
    pub fn root_record_hash(&self) -> &str {
        &self.root_record_hash
    }

    #[must_use]
    pub fn pixel_hash(&self) -> &str {
        &self.pixel_hash
    }

    #[must_use]
    pub fn png_sha256(&self) -> &str {
        &self.png_sha256
    }

    #[must_use]
    #[cfg(test)]
    pub(crate) fn from_test_parts(
        stage_id: String,
        png_path: PathBuf,
        manifest_path: PathBuf,
        width: u32,
        height: u32,
        root_record_hash: String,
        pixel_hash: String,
        png_sha256: String,
    ) -> Self {
        Self {
            stage_id,
            png_path,
            manifest_path,
            width,
            height,
            root_record_hash,
            pixel_hash,
            png_sha256,
        }
    }
}

impl<'a> FullRootArtifactManifest<'a> {
    #[cfg(test)]
    pub(super) const fn from_test_parts(
        width: u32,
        height: u32,
        root_record_hash: &'a str,
        pixel_hash: &'a str,
        png_sha256: &'a str,
        png_path: &'a Path,
    ) -> Self {
        Self {
            width,
            height,
            root_record_hash,
            pixel_hash,
            png_sha256,
            png_path,
        }
    }
}

impl FrameRootMetadata<'_> {
    #[must_use]
    pub(super) fn manifest<'a>(&'a self, png_sha256: &'a str) -> FullRootArtifactManifest<'a> {
        FullRootArtifactManifest {
            width: self.width,
            height: self.height,
            root_record_hash: self.root_record_hash,
            pixel_hash: &self.pixel_hash,
            png_sha256,
            png_path: &self.png_path,
        }
    }
}

impl std::fmt::Display for FullRootArtifactError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPath(reason) => {
                write!(formatter, "invalid artifact output path: {reason}")
            }
            Self::InvalidStageId => formatter.write_str("invalid artifact stage id"),
            Self::ZeroDimensions => formatter.write_str("root frame dimensions must be non-zero"),
            Self::DimensionOverflow => {
                formatter.write_str("root frame dimensions overflow RGBA size")
            }
            Self::RgbaLength { expected, actual } => {
                write!(
                    formatter,
                    "root frame has {actual} RGBA bytes; expected {expected}"
                )
            }
            Self::EmptyPixels => formatter.write_str("root frame pixels are empty"),
            Self::FrameHashMismatch => {
                formatter.write_str("root frame pixel hash does not match its record")
            }
            Self::EmptyPng => formatter.write_str("encoded root PNG is empty"),
            Self::CreateDirectory(error) => {
                write!(formatter, "artifact directory creation failed: {error}")
            }
            Self::Encode(error) => write!(formatter, "root PNG encoding failed: {error}"),
            Self::ManifestEncode(error) => {
                write!(formatter, "root manifest encoding failed: {error}")
            }
            Self::Write { path, source } => {
                write!(
                    formatter,
                    "artifact write failed for {}: {source}",
                    path.display()
                )
            }
        }
    }
}

impl std::error::Error for FullRootArtifactError {}

#[cfg(test)]
mod tests {
    use super::super::validation::sha256_hex;
    use super::*;
    use sha2::Digest;

    #[test]
    fn full_root_artifact_getters_reflect_input_metadata() {
        let artifact = FullRootArtifact::from_test_parts(
            "frame-000".to_owned(),
            "/tmp/art.png".into(),
            "/tmp/artifact.json".into(),
            2,
            3,
            "record".to_owned(),
            "pixels".to_owned(),
            "sha".to_owned(),
        );
        assert_eq!(artifact.stage_id(), "frame-000");
        assert_eq!(artifact.png_path(), std::path::Path::new("/tmp/art.png"));
        assert_eq!(
            artifact.manifest_path(),
            std::path::Path::new("/tmp/artifact.json")
        );
        assert_eq!(artifact.width(), 2);
        assert_eq!(artifact.height(), 3);
        assert_eq!(artifact.root_record_hash(), "record");
        assert_eq!(artifact.pixel_hash(), "pixels");
        assert_eq!(artifact.png_sha256(), "sha");
    }

    #[test]
    fn full_root_artifact_error_messages_are_stable() {
        assert!(
            FullRootArtifactError::InvalidPath("path is empty")
                .to_string()
                .contains("invalid artifact output path")
        );
        assert_eq!(
            FullRootArtifactError::InvalidStageId.to_string(),
            "invalid artifact stage id"
        );
        assert_eq!(
            FullRootArtifactError::ZeroDimensions.to_string(),
            "root frame dimensions must be non-zero"
        );
        assert_eq!(
            FullRootArtifactError::DimensionOverflow.to_string(),
            "root frame dimensions overflow RGBA size"
        );
        assert_eq!(
            FullRootArtifactError::EmptyPixels.to_string(),
            "root frame pixels are empty"
        );
        assert_eq!(
            FullRootArtifactError::FrameHashMismatch.to_string(),
            "root frame pixel hash does not match its record"
        );
        assert_eq!(
            FullRootArtifactError::EmptyPng.to_string(),
            "encoded root PNG is empty"
        );
        assert_eq!(
            FullRootArtifactError::RgbaLength {
                expected: 8,
                actual: 4,
            }
            .to_string(),
            "root frame has 4 RGBA bytes; expected 8"
        );
        let io = || std::io::Error::other("opaque io failure");
        assert!(
            FullRootArtifactError::CreateDirectory(io())
                .to_string()
                .contains("artifact directory creation failed")
        );
        assert!(
            FullRootArtifactError::Encode(image::ImageError::IoError(io()))
                .to_string()
                .contains("root PNG encoding failed")
        );
        let json_error =
            serde_json::from_str::<serde_json::Value>("{").expect_err("invalid JSON must fail");
        assert!(
            FullRootArtifactError::ManifestEncode(json_error)
                .to_string()
                .contains("root manifest encoding failed")
        );
        assert!(
            FullRootArtifactError::Write {
                path: "frame.png".into(),
                source: io(),
            }
            .to_string()
            .contains("artifact write failed for frame.png")
        );
    }

    #[test]
    fn full_root_manifest_roundtrips_with_frame_metadata() {
        let manifest = FullRootArtifactManifest::from_test_parts(
            2,
            3,
            "record",
            "pixels",
            "png",
            std::path::Path::new("frame.png"),
        );
        let frame = FrameRootMetadata {
            stage_id: "frame-000",
            png_path: std::path::PathBuf::from("frame.png"),
            manifest_path: std::path::PathBuf::from("frame.json"),
            width: 2,
            height: 3,
            root_record_hash: "record",
            pixel_hash: "pixels".to_owned(),
        };
        let bytes = serde_json::to_vec(&manifest).expect("manifest should serialize");
        let value =
            serde_json::from_slice::<serde_json::Value>(&bytes).expect("manifest should parse");
        assert_eq!(value["width"], 2);
        assert_eq!(value["height"], 3);
        assert_eq!(value["root_record_hash"], "record");
        assert_eq!(value["pixel_hash"], "pixels");
        assert_eq!(value["png_sha256"], "png");
        let _ = frame.manifest("png");
    }

    #[test]
    fn hash_helper_matches_sha_output() {
        let hashed = sha256_hex(b"artifact");
        let expected = hex::encode(sha2::Sha256::digest(b"artifact"));
        assert_eq!(hashed, expected);
    }
}
