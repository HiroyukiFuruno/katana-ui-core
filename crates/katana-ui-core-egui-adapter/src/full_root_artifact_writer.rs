use crate::text_command_surface::EguiTextCommandSurfaceHostRootFrame;
use image::codecs::png::PngEncoder;
use image::{ColorType, ImageEncoder};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
#[cfg(test)]
use std::path::PathBuf;

const RGBA_CHANNEL_COUNT: usize = 4;

#[path = "full_root_artifact_writer/types.rs"]
mod types;
use types::FullRootArtifactManifest;
pub use types::{FullRootArtifact, FullRootArtifactError, FullRootArtifactWriter};

impl FullRootArtifact {
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

impl FullRootArtifactWriter {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Writes one root PNG and a metadata-only manifest.
    pub fn write(
        &self,
        frame: &EguiTextCommandSurfaceHostRootFrame,
        output_dir: &Path,
        stage_id: &str,
    ) -> Result<FullRootArtifact, FullRootArtifactError> {
        validate_output_dir(output_dir)?;
        validate_stage_id(stage_id)?;

        let (rgba, width, height, expected_pixel_hash) = frame.artifact_rgba();
        let expected_len = checked_rgba_len(width, height)?;
        if rgba.len() != expected_len {
            return Err(FullRootArtifactError::RgbaLength {
                expected: expected_len,
                actual: rgba.len(),
            });
        }
        if rgba.iter().all(|byte| *byte == 0) {
            return Err(FullRootArtifactError::EmptyPixels);
        }
        let pixel_hash = sha256_hex(rgba);
        if pixel_hash != expected_pixel_hash {
            return Err(FullRootArtifactError::FrameHashMismatch);
        }

        fs::create_dir_all(output_dir).map_err(FullRootArtifactError::CreateDirectory)?;
        let png_path = output_dir.join(format!("{stage_id}.png"));
        let manifest_path = output_dir.join(format!("{stage_id}.manifest.json"));
        let png_bytes = encode_png(rgba, width, height)?;
        if png_bytes.is_empty() {
            return Err(FullRootArtifactError::EmptyPng);
        }
        fs::write(&png_path, &png_bytes).map_err(|source| FullRootArtifactError::Write {
            path: png_path.clone(),
            source,
        })?;

        let png_sha256 = sha256_hex(&png_bytes);
        let manifest = FullRootArtifactManifest {
            width,
            height,
            root_record_hash: frame.record().record_hash(),
            pixel_hash: &pixel_hash,
            png_sha256: &png_sha256,
            png_path: &png_path,
        };
        let manifest_bytes =
            serde_json::to_vec_pretty(&manifest).map_err(FullRootArtifactError::ManifestEncode)?;
        fs::write(&manifest_path, manifest_bytes).map_err(|source| {
            FullRootArtifactError::Write {
                path: manifest_path.clone(),
                source,
            }
        })?;

        Ok(FullRootArtifact {
            stage_id: stage_id.to_owned(),
            png_path,
            manifest_path,
            width,
            height,
            root_record_hash: frame.record().record_hash().to_owned(),
            pixel_hash,
            png_sha256,
        })
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
            Self::Write { path, source } => write!(
                formatter,
                "artifact write failed for {}: {source}",
                path.display()
            ),
        }
    }
}

impl std::error::Error for FullRootArtifactError {}

fn validate_output_dir(path: &Path) -> Result<(), FullRootArtifactError> {
    if path.as_os_str().is_empty() {
        return Err(FullRootArtifactError::InvalidPath("path is empty"));
    }
    if path
        .file_name()
        .is_some_and(|name| name == "." || name == "..")
    {
        return Err(FullRootArtifactError::InvalidPath(
            "path names a directory marker",
        ));
    }
    Ok(())
}

fn validate_stage_id(stage_id: &str) -> Result<(), FullRootArtifactError> {
    if stage_id.is_empty()
        || stage_id == "."
        || stage_id == ".."
        || stage_id.contains('/')
        || stage_id.contains('\\')
        || stage_id.contains('\0')
    {
        return Err(FullRootArtifactError::InvalidStageId);
    }
    Ok(())
}

fn checked_rgba_len(width: u32, height: u32) -> Result<usize, FullRootArtifactError> {
    if width == 0 || height == 0 {
        return Err(FullRootArtifactError::ZeroDimensions);
    }
    (width as usize)
        .checked_mul(height as usize)
        .and_then(|pixels| pixels.checked_mul(RGBA_CHANNEL_COUNT))
        .ok_or(FullRootArtifactError::DimensionOverflow)
}

fn encode_png(rgba: &[u8], width: u32, height: u32) -> Result<Vec<u8>, FullRootArtifactError> {
    let mut bytes = Vec::new();
    PngEncoder::new(&mut bytes)
        .write_image(rgba, width, height, ColorType::Rgba8.into())
        .map_err(FullRootArtifactError::Encode)?;
    Ok(bytes)
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::GenericImageView;

    #[test]
    fn png_encoding_has_header_dimensions_and_non_empty_pixels() {
        let rgba = [12, 34, 56, 255, 0, 0, 0, 255];
        let png = encode_png(&rgba, 2, 1).expect("PNG encoding should succeed");
        assert_eq!(&png[..8], b"\x89PNG\r\n\x1a\n");
        let decoded = image::load_from_memory(&png).expect("PNG should decode");
        assert_eq!(decoded.dimensions(), (2, 1));
        assert!(png.iter().any(|byte| *byte != 0));
        assert_eq!(sha256_hex(&png), sha256_hex(&png));
    }

    #[test]
    fn invalid_path_and_stage_are_typed_failures() {
        assert!(matches!(
            validate_output_dir(Path::new("")),
            Err(FullRootArtifactError::InvalidPath("path is empty"))
        ));
        assert!(matches!(
            validate_stage_id("../escape"),
            Err(FullRootArtifactError::InvalidStageId)
        ));
    }

    #[test]
    fn manifest_contains_only_root_metadata() {
        let manifest = FullRootArtifactManifest {
            width: 2,
            height: 1,
            root_record_hash: "record",
            pixel_hash: "pixels",
            png_sha256: "png",
            png_path: Path::new("frame.png"),
        };
        let value: serde_json::Value =
            serde_json::from_slice(&serde_json::to_vec(&manifest).expect("manifest should encode"))
                .expect("manifest should be JSON");
        let object = value.as_object().expect("manifest should be an object");
        assert_eq!(object.len(), 6);
        for forbidden in ["rgba", "child", "paint", "palette", "geometry", "accesskit"] {
            assert!(!value.to_string().to_lowercase().contains(forbidden));
        }
    }
}
