use crate::PlatformFontSha256;
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
    #[cfg(not(unix))]
    created: Option<SystemTime>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CachedFileHash {
    stamp: FileStamp,
    hash: PlatformFontSha256,
}

pub(super) static FILE_HASH_CACHE: OnceLock<Mutex<HashMap<PathBuf, CachedFileHash>>> =
    OnceLock::new();

pub(super) fn read_cached_file_hash(path: &Path) -> std::io::Result<crate::PlatformFontSha256> {
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
    let hash = crate::PlatformFontSha256::digest(&bytes);
    let stable_entry =
        (before == after).then_some((canonical_path, CachedFileHash { stamp: after, hash }));
    lock_or_recover(cache).extend(stable_entry);
    Ok(hash)
}

pub(super) fn load_regular_candidates(
    font_system: &mut FontSystem,
    policy: &crate::catalog_types::PlatformFontCatalogPolicy,
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
        identity: file_identity(&metadata),
        length: metadata.len(),
        modified: metadata.modified().ok(),
        created: metadata.created().ok(),
    })
}

fn file_identity(metadata: &fs::Metadata) -> FileIdentity {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;

        FileIdentity {
            device: metadata.dev(),
            inode: metadata.ino(),
            changed_seconds: metadata.ctime(),
            changed_nanoseconds: metadata.ctime_nsec(),
        }
    }

    #[cfg(not(unix))]
    {
        FileIdentity {
            created: metadata.created().ok(),
        }
    }
}

fn lock_or_recover<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

#[cfg(test)]
mod tests {
    use super::{CachedFileHash, FILE_HASH_CACHE, lock_or_recover, read_cached_file_hash};
    use crate::PlatformFontSha256;
    use std::fs;
    use std::process::Command;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::{Arc, Mutex};

    static NEXT_TEST_FILE: AtomicU64 = AtomicU64::new(0);

    fn test_file() -> std::path::PathBuf {
        let serial = NEXT_TEST_FILE.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "kuc-text-raster-catalog-cache-in-file-{}-{serial}.font",
            std::process::id()
        ))
    }

    fn cached_entry(path: &std::path::Path) -> CachedFileHash {
        let canonical_path = fs::canonicalize(path).expect("test file must be canonicalizable");
        lock_or_recover(FILE_HASH_CACHE.get_or_init(Mutex::default))
            .get(&canonical_path)
            .cloned()
            .expect("file hash must be retained in the cache")
    }

    fn poison_actual_cache_lock() {
        let cache = FILE_HASH_CACHE.get_or_init(Mutex::default);
        let poisoned = Arc::new(cache);
        let result = std::thread::spawn(move || {
            let _guard = poisoned.lock().expect("initial cache lock");
            panic!("poison actual cache lock");
        })
        .join();
        assert!(result.is_err());
    }

    #[test]
    fn poisoned_cache_lock_recovers_the_retained_value() {
        let value = Arc::new(Mutex::new(7));
        let poison = Arc::clone(&value);
        assert!(
            std::thread::spawn(move || {
                let _guard = poison.lock().expect("initial lock");
                panic!("poison lock fixture");
            })
            .join()
            .is_err()
        );
        assert_eq!(*lock_or_recover(&value), 7);
    }

    #[test]
    fn unchanged_file_reuses_the_actual_cached_hash() {
        let path = test_file();
        fs::write(&path, b"stable font bytes").expect("write test file");

        let first = read_cached_file_hash(&path).expect("first hash read");
        let cached_after_first = cached_entry(&path);
        let second = read_cached_file_hash(&path).expect("second hash read");
        let cached_after_second = cached_entry(&path);

        assert_eq!(first, second);
        assert_eq!(cached_after_first, cached_after_second);
        assert_eq!(cached_after_second.hash, second);
        fs::remove_file(path).expect("remove test file");
    }

    #[test]
    fn changed_file_recomputes_the_actual_cached_hash() {
        let path = test_file();
        fs::write(&path, b"old font bytes").expect("write initial test file");
        let first = read_cached_file_hash(&path).expect("first hash read");
        let cached_after_first = cached_entry(&path);

        fs::write(&path, b"new font bytes with a changed stamp").expect("rewrite test file");
        let second = read_cached_file_hash(&path).expect("second hash read");
        let cached_after_second = cached_entry(&path);

        assert_ne!(first, second);
        assert_eq!(
            second,
            PlatformFontSha256::digest(b"new font bytes with a changed stamp")
        );
        assert_ne!(cached_after_first.stamp, cached_after_second.stamp);
        assert_eq!(cached_after_second.hash, second);
        fs::remove_file(path).expect("remove test file");
    }

    #[test]
    fn actual_cache_lock_poison_recovery_is_safe() {
        const CHILD_ENV: &str = "KUC_TEXT_RASTER_POISON_ACTUAL_CACHE";
        if std::env::var_os(CHILD_ENV).is_some() {
            poison_actual_cache_lock();
            let path = test_file();
            fs::write(&path, b"poison recovery font bytes").expect("write test file");
            assert!(read_cached_file_hash(&path).is_ok());
            fs::remove_file(path).expect("remove test file");
            return;
        }

        let status = Command::new(std::env::current_exe().expect("test executable"))
            .args([
                "--exact",
                "catalog::catalog_cache::tests::actual_cache_lock_poison_recovery_is_safe",
            ])
            .env(CHILD_ENV, "1")
            .status()
            .expect("run isolated poison recovery test");
        assert!(status.success());
    }
}
