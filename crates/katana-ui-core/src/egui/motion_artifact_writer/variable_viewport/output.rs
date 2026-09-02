use cap_fs_ext::{DirExt, FollowSymlinks, OpenOptionsFollowExt};
use cap_std::fs::{Dir, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use super::super::constants::{
    VARIABLE_VIEWPORT_GIF_FILENAME, VARIABLE_VIEWPORT_MANIFEST_FILENAME,
    VARIABLE_VIEWPORT_MP4_FILENAME, VARIABLE_VIEWPORT_STAGING_DIRECTORY,
};
use super::super::error::MotionArtifactError;
use super::super::validation::{expected_stage_name, io_error};
use super::error::VariableViewportMotionArtifactError;

pub(super) fn open_output_directory(
    output_dir: &Path,
) -> Result<Dir, VariableViewportMotionArtifactError> {
    std::fs::create_dir_all(output_dir).map_err(io_error)?;
    Dir::open_ambient_dir(output_dir, cap_std::ambient_authority())
        .map_err(io_error)
        .map_err(Into::into)
}

pub(super) fn claim_public_staging_directory(
    output: &Dir,
    output_dir: &Path,
) -> Result<Dir, VariableViewportMotionArtifactError> {
    let staging_path = output_dir.join(VARIABLE_VIEWPORT_STAGING_DIRECTORY);
    match output.create_dir(VARIABLE_VIEWPORT_STAGING_DIRECTORY) {
        Ok(()) => output
            .open_dir_nofollow(VARIABLE_VIEWPORT_STAGING_DIRECTORY)
            .map_err(io_error)
            .map_err(Into::into),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            Err(VariableViewportMotionArtifactError::OccupiedOutputTarget { path: staging_path })
        }
        Err(error) => Err(io_error(error).into()),
    }
}

pub(super) fn reject_occupied_output_targets(
    output: &Dir,
    output_dir: &Path,
) -> Result<(), VariableViewportMotionArtifactError> {
    for filename in [
        VARIABLE_VIEWPORT_STAGING_DIRECTORY,
        VARIABLE_VIEWPORT_GIF_FILENAME,
        VARIABLE_VIEWPORT_MP4_FILENAME,
        VARIABLE_VIEWPORT_MANIFEST_FILENAME,
    ] {
        match output.symlink_metadata(filename) {
            Ok(_) => {
                return Err(VariableViewportMotionArtifactError::OccupiedOutputTarget {
                    path: output_dir.join(filename),
                });
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(io_error(error).into()),
        }
    }
    Ok(())
}

pub(super) fn verify_public_output_directories(
    output: &Dir,
    public_staging: &Dir,
    output_dir: &Path,
) -> Result<(), VariableViewportMotionArtifactError> {
    verify_directory_identity(output, output_dir)?;
    verify_directory_identity(
        public_staging,
        &output_dir.join(VARIABLE_VIEWPORT_STAGING_DIRECTORY),
    )
}

pub(super) fn verify_public_artifact_file(
    artifact: &same_file::Handle,
    artifact_path: &Path,
) -> Result<(), VariableViewportMotionArtifactError> {
    verify_identity(artifact, artifact_path, "file")
}

pub(super) fn verify_public_frame_files(
    frames: &[same_file::Handle],
    output_dir: &Path,
) -> Result<(), VariableViewportMotionArtifactError> {
    for (index, frame) in frames.iter().enumerate() {
        let filename = PathBuf::from(expected_stage_name(index)).with_extension("png");
        verify_public_artifact_file(
            frame,
            &output_dir
                .join(VARIABLE_VIEWPORT_STAGING_DIRECTORY)
                .join(filename),
        )?;
    }
    Ok(())
}

fn verify_directory_identity(
    directory: &Dir,
    directory_path: &Path,
) -> Result<(), VariableViewportMotionArtifactError> {
    let pinned =
        same_file::Handle::from_file(directory.try_clone().map_err(io_error)?.into_std_file())
            .map_err(io_error)?;
    verify_identity(&pinned, directory_path, "directory")
}

fn verify_identity(
    pinned: &same_file::Handle,
    current_path: &Path,
    kind: &str,
) -> Result<(), VariableViewportMotionArtifactError> {
    let current = same_file::Handle::from_path(current_path).map_err(io_error)?;
    if pinned != &current {
        return Err(io_error(std::io::Error::other(format!(
            "public artifact {kind} changed during export: {}",
            current_path.display()
        )))
        .into());
    }
    Ok(())
}

pub(super) fn private_scratch_directory(
    temporary_parent: &Path,
) -> Result<tempfile::TempDir, VariableViewportMotionArtifactError> {
    temporary_parent
        .to_str()
        .ok_or(MotionArtifactError::InvalidSettings)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        const OWNER_ONLY_DIRECTORY_MODE: u32 = 0o700;
        tempfile::Builder::new()
            .permissions(std::fs::Permissions::from_mode(OWNER_ONLY_DIRECTORY_MODE))
            .tempdir_in(temporary_parent)
            .map_err(io_error)
            .map_err(Into::into)
            .and_then(reject_non_utf8_scratch_directory)
    }
    #[cfg(not(unix))]
    {
        tempfile::Builder::new()
            .tempdir_in(temporary_parent)
            .map_err(io_error)
            .map_err(Into::into)
            .and_then(reject_non_utf8_scratch_directory)
    }
}

fn reject_non_utf8_scratch_directory(
    scratch: tempfile::TempDir,
) -> Result<tempfile::TempDir, VariableViewportMotionArtifactError> {
    scratch
        .path()
        .to_str()
        .ok_or(MotionArtifactError::InvalidSettings)?;
    Ok(scratch)
}

pub(super) fn reject_scratch_output_overlap(
    scratch_dir: &Path,
    output_dir: &Path,
) -> Result<(), VariableViewportMotionArtifactError> {
    let scratch_dir = std::fs::canonicalize(scratch_dir).map_err(io_error)?;
    let output_dir = std::fs::canonicalize(output_dir).map_err(io_error)?;
    if scratch_dir.starts_with(output_dir) {
        return Err(MotionArtifactError::InvalidSettings.into());
    }
    Ok(())
}

pub(super) fn publish_public_frames(
    scratch_dir: &Path,
    public_staging: &Dir,
    output_dir: &Path,
    frame_count: usize,
) -> Result<Vec<same_file::Handle>, VariableViewportMotionArtifactError> {
    let mut frames = Vec::with_capacity(frame_count);
    for index in 0..frame_count {
        let filename = PathBuf::from(expected_stage_name(index)).with_extension("png");
        frames.push(publish_scratch_file(
            &scratch_dir.join(&filename),
            public_staging,
            &filename,
            &output_dir
                .join(VARIABLE_VIEWPORT_STAGING_DIRECTORY)
                .join(&filename),
        )?);
    }
    Ok(frames)
}

pub(super) fn publish_scratch_file(
    scratch_path: &Path,
    output: &Dir,
    filename: impl AsRef<Path>,
    output_path: &Path,
) -> Result<same_file::Handle, VariableViewportMotionArtifactError> {
    let mut source = std::fs::File::open(scratch_path).map_err(io_error)?;
    let mut destination = open_new_output(output, filename, output_path)?;
    std::io::copy(&mut source, &mut destination).map_err(io_error)?;
    same_file::Handle::from_file(destination.into_std())
        .map_err(io_error)
        .map_err(Into::into)
}

pub(super) fn write_new_output(
    output: &Dir,
    filename: impl AsRef<Path>,
    output_path: &Path,
    bytes: &[u8],
) -> Result<same_file::Handle, VariableViewportMotionArtifactError> {
    let mut destination = open_new_output(output, filename, output_path)?;
    destination.write_all(bytes).map_err(io_error)?;
    same_file::Handle::from_file(destination.into_std())
        .map_err(io_error)
        .map_err(Into::into)
}

fn open_new_output(
    output: &Dir,
    filename: impl AsRef<Path>,
    output_path: &Path,
) -> Result<cap_std::fs::File, VariableViewportMotionArtifactError> {
    let mut options = OpenOptions::new();
    options
        .write(true)
        .create_new(true)
        .follow(FollowSymlinks::No);
    output.open_with(filename, &options).map_err(|error| {
        if error.kind() == std::io::ErrorKind::AlreadyExists {
            VariableViewportMotionArtifactError::OccupiedOutputTarget {
                path: output_path.to_path_buf(),
            }
        } else {
            io_error(error).into()
        }
    })
}
