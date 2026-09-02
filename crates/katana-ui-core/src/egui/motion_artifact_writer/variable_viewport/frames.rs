use crate::egui::OpaqueRootArtifactReceipt;
use image::{GenericImageView, ImageDecoder, Rgba, RgbaImage};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::io::Read;
use std::path::Path;

use super::super::error::MotionArtifactError;
use super::super::types::VariableViewportSourceViewport;
use super::super::validation::{
    expected_stage_name, hash_sha256, io_error, validate_provenance_bytes,
};

const RGBA_CHANNEL_COUNT: u64 = 4;
const NORMALIZED_FRAME_COPY_COUNT: u64 = 2;
const MAX_VARIABLE_VIEWPORT_WORKING_SET_BYTES: u64 = 1024 * 1024 * 1024;
pub(super) const MAX_VARIABLE_VIEWPORT_ENCODED_PNG_BYTES: u64 =
    MAX_VARIABLE_VIEWPORT_WORKING_SET_BYTES / 16;
pub(super) const MAX_VARIABLE_VIEWPORT_PROVENANCE_BYTES: u64 = 1024 * 1024;
const BOUNDED_READ_CHUNK_BYTES: usize = 64 * 1024;

pub(super) struct LoadedReceipts {
    pub(super) images: Vec<RgbaImage>,
    pub(super) source_viewports: Vec<VariableViewportSourceViewport>,
    pub(super) source_png_sha256: Vec<String>,
    pub(super) root_record_hashes: Vec<String>,
    pub(super) frame_sequence_sha256: String,
}

pub(super) fn load_receipts(
    receipts: &[OpaqueRootArtifactReceipt],
) -> Result<LoadedReceipts, MotionArtifactError> {
    validate_normalized_working_set(receipts)?;
    let mut stages = BTreeSet::new();
    let mut images = Vec::with_capacity(receipts.len());
    let mut source_viewports = Vec::with_capacity(receipts.len());
    let mut source_png_sha256 = Vec::with_capacity(receipts.len());
    let mut root_record_hashes = Vec::with_capacity(receipts.len());
    let mut frame_sequence = Sha256::new();
    frame_sequence.update((receipts.len() as u64).to_le_bytes());

    for (index, opaque) in receipts.iter().enumerate() {
        let receipt = opaque.artifact();
        if !stages.insert(receipt.stage_id()) {
            return Err(MotionArtifactError::DuplicateStage(
                receipt.stage_id().to_owned(),
            ));
        }
        let expected_stage = expected_stage_name(index);
        if receipt.stage_id() != expected_stage {
            return Err(MotionArtifactError::StaleStage {
                expected: expected_stage,
                actual: receipt.stage_id().to_owned(),
            });
        }
        if !receipt.png_path().is_file() {
            return Err(MotionArtifactError::MissingPng(
                receipt.png_path().to_path_buf(),
            ));
        }
        if !receipt.manifest_path().is_file() {
            return Err(MotionArtifactError::MissingProvenance(
                receipt.manifest_path().to_path_buf(),
            ));
        }
        let bytes = read_bounded_file(receipt.png_path(), MAX_VARIABLE_VIEWPORT_ENCODED_PNG_BYTES)?;
        if hash_sha256(&bytes) != receipt.png_sha256() {
            return Err(MotionArtifactError::BadPngSha {
                path: receipt.png_path().to_path_buf(),
            });
        }
        let provenance = read_bounded_file(
            receipt.manifest_path(),
            MAX_VARIABLE_VIEWPORT_PROVENANCE_BYTES,
        )?;
        validate_provenance_bytes(receipt, &provenance)?;
        let decoder =
            image::codecs::png::PngDecoder::new(std::io::Cursor::new(&bytes)).map_err(|error| {
                MotionArtifactError::InvalidPng {
                    path: receipt.png_path().to_path_buf(),
                    reason: error.to_string(),
                }
            })?;
        if decoder.color_type() != image::ColorType::Rgba8 {
            return Err(MotionArtifactError::InvalidPng {
                path: receipt.png_path().to_path_buf(),
                reason: "PNG color type is not RGBA8".into(),
            });
        }
        let image = image::load_from_memory_with_format(&bytes, image::ImageFormat::Png).map_err(
            |error| MotionArtifactError::InvalidPng {
                path: receipt.png_path().to_path_buf(),
                reason: error.to_string(),
            },
        )?;
        if image.dimensions() != (receipt.width(), receipt.height()) {
            return Err(MotionArtifactError::WrongDimensions {
                path: receipt.png_path().to_path_buf(),
                expected: (receipt.width(), receipt.height()),
                actual: image.dimensions(),
            });
        }
        let rgba = image.to_rgba8();
        if rgba.pixels().all(|pixel| pixel.0 == [0, 0, 0, 0]) {
            return Err(MotionArtifactError::EmptyPixels(
                receipt.png_path().to_path_buf(),
            ));
        }

        frame_sequence.update((bytes.len() as u64).to_le_bytes());
        frame_sequence.update(&bytes);
        source_viewports.push(VariableViewportSourceViewport {
            width: receipt.width(),
            height: receipt.height(),
        });
        source_png_sha256.push(receipt.png_sha256().to_owned());
        root_record_hashes.push(receipt.root_record_hash().to_owned());
        images.push(rgba);
    }

    Ok(LoadedReceipts {
        images,
        source_viewports,
        source_png_sha256,
        root_record_hashes,
        frame_sequence_sha256: hex::encode(frame_sequence.finalize()),
    })
}

pub(super) fn read_bounded_file(
    path: &Path,
    maximum_bytes: u64,
) -> Result<Vec<u8>, MotionArtifactError> {
    let file = std::fs::File::open(path).map_err(io_error)?;
    let initial_length = file.metadata().map_err(io_error)?.len();
    if initial_length > maximum_bytes {
        return Err(MotionArtifactError::InvalidSettings);
    }
    read_bounded(file, initial_length, maximum_bytes)
}

pub(super) fn read_bounded(
    reader: impl Read,
    initial_length: u64,
    maximum_bytes: u64,
) -> Result<Vec<u8>, MotionArtifactError> {
    let bounded_limit = maximum_bytes
        .checked_add(1)
        .ok_or(MotionArtifactError::InvalidSettings)?;
    let maximum_bytes =
        usize::try_from(maximum_bytes).map_err(|_| MotionArtifactError::InvalidSettings)?;
    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(
            usize::try_from(initial_length).map_err(|_| MotionArtifactError::InvalidSettings)?,
        )
        .map_err(|_| MotionArtifactError::InvalidSettings)?;
    let mut reader = reader.take(bounded_limit);
    let mut chunk = [0_u8; BOUNDED_READ_CHUNK_BYTES];
    loop {
        let count = reader.read(&mut chunk).map_err(io_error)?;
        if count == 0 {
            return Ok(bytes);
        }
        let length = bytes
            .len()
            .checked_add(count)
            .ok_or(MotionArtifactError::InvalidSettings)?;
        if length > maximum_bytes {
            return Err(MotionArtifactError::InvalidSettings);
        }
        bytes
            .try_reserve_exact(count)
            .map_err(|_| MotionArtifactError::InvalidSettings)?;
        bytes.extend_from_slice(&chunk[..count]);
    }
}

pub(super) fn validate_normalized_working_set(
    receipts: &[OpaqueRootArtifactReceipt],
) -> Result<(), MotionArtifactError> {
    let width = receipts
        .iter()
        .map(|receipt| receipt.artifact().width())
        .max()
        .ok_or(MotionArtifactError::EmptySequence)?;
    let height = receipts
        .iter()
        .map(|receipt| receipt.artifact().height())
        .max()
        .ok_or(MotionArtifactError::EmptySequence)?;
    let source_bytes = receipts.iter().try_fold(0_u64, |total, receipt| {
        let bytes = rgba_bytes(receipt.artifact().width(), receipt.artifact().height())?;
        total
            .checked_add(bytes)
            .ok_or(MotionArtifactError::InvalidSettings)
    })?;
    let frame_count =
        u64::try_from(receipts.len()).map_err(|_| MotionArtifactError::InvalidSettings)?;
    let normalized_bytes = rgba_bytes(width, height)?
        .checked_mul(frame_count)
        .and_then(|bytes| bytes.checked_mul(NORMALIZED_FRAME_COPY_COUNT))
        .ok_or(MotionArtifactError::InvalidSettings)?;
    let working_set = source_bytes
        .checked_add(normalized_bytes)
        .ok_or(MotionArtifactError::InvalidSettings)?;
    if working_set > MAX_VARIABLE_VIEWPORT_WORKING_SET_BYTES {
        return Err(MotionArtifactError::InvalidSettings);
    }
    usize::try_from(working_set)
        .map(|_| ())
        .map_err(|_| MotionArtifactError::InvalidSettings)
}

fn rgba_bytes(width: u32, height: u32) -> Result<u64, MotionArtifactError> {
    u64::from(width)
        .checked_mul(u64::from(height))
        .and_then(|pixels| pixels.checked_mul(RGBA_CHANNEL_COUNT))
        .filter(|bytes| *bytes > 0)
        .ok_or(MotionArtifactError::InvalidSettings)
}

pub(super) fn normalize_frames(images: &[RgbaImage], width: u32, height: u32) -> Vec<RgbaImage> {
    images
        .iter()
        .map(|source| {
            let mut canvas = RgbaImage::from_pixel(width, height, Rgba([0, 0, 0, u8::MAX]));
            for (x, y, pixel) in source.enumerate_pixels() {
                canvas.put_pixel(x, y, *pixel);
            }
            canvas
        })
        .collect()
}

pub(super) fn write_staging_frames(
    images: &[RgbaImage],
    staging_dir: &Path,
) -> Result<(), MotionArtifactError> {
    for (index, image) in images.iter().enumerate() {
        let path = staging_dir
            .join(expected_stage_name(index))
            .with_extension("png");
        let mut bytes = Vec::new();
        encode_staging_frame(image, &path, &mut bytes)?;
        std::fs::write(path, bytes).map_err(io_error)?;
    }
    Ok(())
}

pub(super) fn encode_staging_frame(
    image: &RgbaImage,
    path: &Path,
    writer: impl std::io::Write,
) -> Result<(), MotionArtifactError> {
    use image::ImageEncoder;

    image::codecs::png::PngEncoder::new(writer)
        .write_image(
            image.as_raw(),
            image.width(),
            image.height(),
            image::ColorType::Rgba8.into(),
        )
        .map_err(|error| MotionArtifactError::InvalidPng {
            path: path.to_path_buf(),
            reason: error.to_string(),
        })
}
