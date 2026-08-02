use std::fs;
use std::io::{self, ErrorKind};
use std::path::Path;
use std::time::UNIX_EPOCH;

pub(crate) struct SnapshotOutput;

impl SnapshotOutput {
    pub(crate) fn prepare(path: &Path) -> io::Result<()> {
        if let Some(parent) = path.parent().filter(|it| !it.as_os_str().is_empty()) {
            fs::create_dir_all(parent)?;
        }
        match fs::remove_file(path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error),
        }
    }

    pub(crate) fn evidence(path: &Path) -> io::Result<String> {
        let metadata = fs::metadata(path)?;
        let modified = metadata.modified()?;
        let seconds = modified
            .duration_since(UNIX_EPOCH)
            .map_or(0, |duration| duration.as_secs());
        Ok(format!(
            "bytes={} modified_unix={}",
            metadata.len(),
            seconds
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::SnapshotOutput;
    use std::error::Error;
    use std::{env, fs, process};

    #[test]
    fn snapshot_prepare_removes_stale_fixed_output() -> Result<(), Box<dyn Error>> {
        let directory = env::temp_dir().join(format!("kuc-storybook-snapshot-{}", process::id()));
        let path = directory.join("panel.png");

        fs::create_dir_all(&directory)?;
        fs::write(&path, b"stale")?;
        SnapshotOutput::prepare(&path)?;

        assert!(!path.exists());
        fs::remove_dir_all(&directory)?;
        Ok(())
    }

    #[test]
    fn snapshot_evidence_reports_written_file() -> Result<(), Box<dyn Error>> {
        let directory =
            env::temp_dir().join(format!("kuc-storybook-snapshot-evidence-{}", process::id()));
        let path = directory.join("panel.png");

        fs::create_dir_all(&directory)?;
        fs::write(&path, b"fresh")?;
        let evidence = SnapshotOutput::evidence(&path)?;

        assert!(evidence.contains("bytes=5"));
        assert!(evidence.contains("modified_unix="));
        fs::remove_dir_all(&directory)?;
        Ok(())
    }

    #[test]
    fn snapshot_prepare_accepts_a_bare_relative_output_name() -> Result<(), Box<dyn Error>> {
        SnapshotOutput::prepare(std::path::Path::new(
            "kuc-storybook-intentionally-missing-output.png",
        ))?;
        Ok(())
    }
}
