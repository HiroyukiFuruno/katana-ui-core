use crate::text_raster::PlatformFontSha256;
use cosmic_text::FontSystem;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct FileStamp {
    pub(super) identity: FileIdentity,
    length: u64,
    modified: Option<SystemTime>,
    created: Option<SystemTime>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct FileIdentity {
    #[cfg(unix)]
    device: u64,
    #[cfg(unix)]
    inode: u64,
    #[cfg(unix)]
    changed_seconds: i64,
    #[cfg(unix)]
    changed_nanoseconds: i64,
    #[cfg(windows)]
    volume_serial_number: u32,
    #[cfg(windows)]
    file_index: u64,
    #[cfg(not(any(unix, windows)))]
    created: Option<SystemTime>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CachedFileHash {
    stamp: FileStamp,
    hash: PlatformFontSha256,
}

pub(super) static FILE_HASH_CACHE: OnceLock<Mutex<HashMap<PathBuf, CachedFileHash>>> =
    OnceLock::new();

pub(super) fn read_cached_file_hash(
    path: &Path,
) -> std::io::Result<crate::text_raster::PlatformFontSha256> {
    let canonical_path = fs::canonicalize(path)?;
    let before = file_stamp(&canonical_path)?;
    let cache = FILE_HASH_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let cached = lock_or_recover(cache)
        .get(&canonical_path)
        .and_then(|entry| (entry.stamp == before).then_some(entry.hash));
    if let Some(entry) = cached {
        return Ok(entry);
    }

    let bytes = fs::read(&canonical_path)?;
    let after = file_stamp(&canonical_path)?;
    let hash = crate::text_raster::PlatformFontSha256::digest(&bytes);
    let stable_entry =
        (before == after).then_some((canonical_path, CachedFileHash { stamp: after, hash }));
    lock_or_recover(cache).extend(stable_entry);
    Ok(hash)
}

pub(super) fn load_regular_candidates(
    font_system: &mut FontSystem,
    policy: &crate::text_raster::catalog_types::PlatformFontCatalogPolicy,
) -> usize {
    use std::collections::HashSet;

    let mut loaded_paths = HashSet::new();
    policy
        .proportional_candidates
        .iter()
        .chain(&policy.monospace_candidates)
        .filter(|path| loaded_paths.insert((*path).clone()))
        .map(|path| {
            let _ = font_system.db_mut().load_font_file(path);
            1
        })
        .sum()
}

pub(super) fn family_from_loaded_file(
    font_system: &FontSystem,
    source_file_path: &Path,
    expected_family: &str,
) -> Option<String> {
    let families = font_system
        .db()
        .faces()
        .filter(|face| match &face.source {
            cosmic_text::fontdb::Source::File(path) => path == source_file_path,
            _ => false,
        })
        .flat_map(|face| face.families.iter().map(|(family, _)| family.clone()))
        .collect::<Vec<_>>();
    families
        .iter()
        .find(|family| family.as_str() == expected_family)
        .cloned()
        .or_else(|| families.into_iter().next())
}

pub(super) fn file_stamp(path: &Path) -> std::io::Result<FileStamp> {
    let metadata = fs::metadata(path)?;
    Ok(FileStamp {
        identity: file_identity(path, &metadata)?,
        length: metadata.len(),
        modified: metadata.modified().ok(),
        created: metadata.created().ok(),
    })
}

fn file_identity(_path: &Path, _metadata: &fs::Metadata) -> std::io::Result<FileIdentity> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;

        Ok(FileIdentity {
            device: _metadata.dev(),
            inode: _metadata.ino(),
            changed_seconds: _metadata.ctime(),
            changed_nanoseconds: _metadata.ctime_nsec(),
        })
    }

    #[cfg(windows)]
    {
        windows_file_identity(_path)
    }

    #[cfg(not(any(unix, windows)))]
    {
        Ok(FileIdentity {
            created: _metadata.created().ok(),
        })
    }
}

#[cfg(windows)]
fn windows_file_identity(path: &Path) -> std::io::Result<FileIdentity> {
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::Storage::FileSystem::{
        BY_HANDLE_FILE_INFORMATION, GetFileInformationByHandle,
    };

    const FILE_INDEX_HIGH_SHIFT: u32 = 32;

    let file = fs::File::open(path)?;
    let mut information = BY_HANDLE_FILE_INFORMATION::default();
    /* SAFETY: `file` owns a valid handle for this call, `information` is writable for its complete
    C layout, and the handle remains alive until after the call returns. */
    let succeeded = unsafe {
        GetFileInformationByHandle(file.as_raw_handle(), std::ptr::from_mut(&mut information))
    };
    if succeeded == 0 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(FileIdentity {
        volume_serial_number: information.dwVolumeSerialNumber,
        file_index: (u64::from(information.nFileIndexHigh) << FILE_INDEX_HIGH_SHIFT)
            | u64::from(information.nFileIndexLow),
    })
}

fn lock_or_recover<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

#[cfg(test)]
#[path = "catalog_cache_tests.rs"]
mod tests;
