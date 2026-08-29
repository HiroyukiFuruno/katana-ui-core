use crate::catalog_types::{
    PlatformColorEmojiFaceRecord, PlatformColorEmojiFaceResolver, PlatformEmojiFontCandidate,
    PlatformEmojiFontLoadError, PlatformEmojiFontLoader, PlatformEmojiFontObservation,
    PlatformFontCatalogError, PlatformFontCatalogPolicy,
};
use cosmic_text::FontSystem;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::SystemTime;

mod types;

pub use types::{PlatformFontCatalog, PlatformFontCatalogStats};

#[derive(Debug, Clone, PartialEq, Eq)]
struct FileStamp {
    identity: FileIdentity,
    length: u64,
    modified: Option<SystemTime>,
    created: Option<SystemTime>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FileIdentity {
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
struct CachedFileHash {
    stamp: FileStamp,
    hash: crate::PlatformFontSha256,
}

static FILE_HASH_CACHE: OnceLock<Mutex<HashMap<PathBuf, CachedFileHash>>> = OnceLock::new();

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TestHashComputationKey {
    thread: std::thread::ThreadId,
    path: PathBuf,
}

#[cfg(test)]
static TEST_HASH_COMPUTATIONS: OnceLock<Mutex<HashMap<TestHashComputationKey, usize>>> =
    OnceLock::new();

impl PlatformFontCatalog {
    #[must_use]
    pub fn new(policy: PlatformFontCatalogPolicy) -> Self {
        let mut font_system = FontSystem::new();
        let (emoji_face, emoji_load_attempts) = {
            let mut loader = SystemEmojiFontLoader {
                font_system: &mut font_system,
                load_attempts: 0,
            };
            let emoji_face = PlatformColorEmojiFaceResolver::resolve(&policy, &mut loader);
            (emoji_face, loader.load_attempts)
        };
        let regular_load_attempts = load_regular_candidates(&mut font_system, &policy);
        Self {
            policy,
            font_system: Mutex::new(font_system),
            emoji_face,
            stats: PlatformFontCatalogStats {
                font_database_discoveries: 1,
                candidate_load_attempts: emoji_load_attempts + regular_load_attempts,
            },
        }
    }

    #[must_use]
    pub fn policy(&self) -> &PlatformFontCatalogPolicy {
        &self.policy
    }

    #[must_use]
    pub fn emoji_face(&self) -> &PlatformColorEmojiFaceRecord {
        &self.emoji_face
    }

    #[must_use]
    pub const fn stats(&self) -> PlatformFontCatalogStats {
        self.stats
    }

    #[must_use]
    pub fn fingerprint(&self) -> crate::PlatformFontCatalogFingerprint {
        self.emoji_face.catalog_fingerprint
    }

    pub(crate) fn with_font_system<T>(
        &self,
        operation: impl FnOnce(&mut FontSystem) -> T,
    ) -> Result<T, PlatformFontCatalogError> {
        let mut font_system = self
            .font_system
            .lock()
            .map_err(|_| PlatformFontCatalogError::FontSystemLockPoisoned)?;
        Ok(operation(&mut font_system))
    }
}

struct SystemEmojiFontLoader<'a> {
    font_system: &'a mut FontSystem,
    load_attempts: usize,
}

impl PlatformEmojiFontLoader for SystemEmojiFontLoader<'_> {
    fn load(
        &mut self,
        candidate: &PlatformEmojiFontCandidate,
    ) -> Result<PlatformEmojiFontObservation, PlatformEmojiFontLoadError> {
        self.load_attempts += 1;
        let raw_file_sha256 =
            read_cached_file_hash(&candidate.source_file_path).map_err(|error| {
                if error.kind() == std::io::ErrorKind::NotFound {
                    PlatformEmojiFontLoadError::Missing {
                        source_file_path: candidate.source_file_path.clone(),
                    }
                } else {
                    PlatformEmojiFontLoadError::Io {
                        source_file_path: candidate.source_file_path.clone(),
                        message: error.to_string(),
                    }
                }
            })?;
        self.font_system
            .db_mut()
            .load_font_file(&candidate.source_file_path)
            .map_err(|error| PlatformEmojiFontLoadError::Io {
                source_file_path: candidate.source_file_path.clone(),
                message: error.to_string(),
            })?;
        let actual_family = family_from_loaded_file(
            self.font_system,
            &candidate.source_file_path,
            &candidate.expected_family,
        )
        .ok_or_else(|| PlatformEmojiFontLoadError::FaceNotFound {
            source_file_path: candidate.source_file_path.clone(),
        })?;
        Ok(PlatformEmojiFontObservation {
            actual_family,
            source_file_path: candidate.source_file_path.clone(),
            raw_file_sha256,
        })
    }
}

fn read_cached_file_hash(path: &Path) -> std::io::Result<crate::PlatformFontSha256> {
    let canonical_path = fs::canonicalize(path)?;
    let before = file_stamp(&canonical_path)?;
    let cache = FILE_HASH_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    if let Ok(cache) = cache.lock()
        && let Some(entry) = cache.get(&canonical_path)
        && entry.stamp == before
    {
        return Ok(entry.hash);
    }

    let bytes = fs::read(&canonical_path)?;
    let after = file_stamp(&canonical_path)?;
    let hash = digest_file(&canonical_path, &bytes);
    if before == after
        && let Ok(mut cache) = cache.lock()
    {
        cache.insert(canonical_path, CachedFileHash { stamp: after, hash });
    }
    Ok(hash)
}

fn digest_file(_path: &Path, bytes: &[u8]) -> crate::PlatformFontSha256 {
    #[cfg(test)]
    record_hash_computation(_path);
    crate::PlatformFontSha256::digest(bytes)
}

fn file_stamp(path: &Path) -> std::io::Result<FileStamp> {
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

#[cfg(test)]
fn record_hash_computation(path: &Path) {
    let key = TestHashComputationKey {
        thread: std::thread::current().id(),
        path: path.to_path_buf(),
    };
    let computations = TEST_HASH_COMPUTATIONS.get_or_init(|| Mutex::new(HashMap::new()));
    let Ok(mut computations) = computations.lock() else {
        return;
    };
    *computations.entry(key).or_default() += 1;
}

fn family_from_loaded_file(
    font_system: &FontSystem,
    source_file_path: &Path,
    expected_family: &str,
) -> Option<String> {
    let families = font_system
        .db()
        .faces()
        .filter(|face| match &face.source {
            fontdb::Source::File(path) => path == source_file_path,
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

fn load_regular_candidates(
    font_system: &mut FontSystem,
    policy: &PlatformFontCatalogPolicy,
) -> usize {
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

#[cfg(test)]
mod tests {
    use super::{FILE_HASH_CACHE, PlatformFontCatalog, file_stamp, read_cached_file_hash};
    use crate::{
        PlatformEmojiFontCandidate, PlatformFontCatalogPolicy, PlatformFontProfile,
        PlatformFontSha256,
    };
    use cosmic_text::FontSystem;
    use std::fs;
    use std::io;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_TEST_FILE: AtomicU64 = AtomicU64::new(0);

    fn test_file() -> PathBuf {
        let serial = NEXT_TEST_FILE.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "kuc-text-raster-catalog-cache-{}-{serial}.font",
            std::process::id()
        ))
    }

    fn reset_hash_cache_observation(path: &std::path::Path) -> io::Result<()> {
        let path = fs::canonicalize(path)?;
        let cache =
            FILE_HASH_CACHE.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
        cache
            .lock()
            .map_err(|_| io::Error::other("file hash cache lock poisoned"))?
            .remove(&path);
        let key = super::TestHashComputationKey {
            thread: std::thread::current().id(),
            path,
        };
        let computations = super::TEST_HASH_COMPUTATIONS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
        computations
            .lock()
            .map_err(|_| io::Error::other("hash computation counter lock poisoned"))?
            .remove(&key);
        Ok(())
    }

    fn hash_computation_count(path: &std::path::Path) -> io::Result<usize> {
        let key = super::TestHashComputationKey {
            thread: std::thread::current().id(),
            path: fs::canonicalize(path)?,
        };
        let computations = super::TEST_HASH_COMPUTATIONS
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
        Ok(computations
            .lock()
            .map_err(|_| io::Error::other("hash computation counter lock poisoned"))?
            .get(&key)
            .copied()
            .unwrap_or_default())
    }

    fn installed_font_candidate() -> io::Result<PlatformEmojiFontCandidate> {
        FontSystem::new()
            .db()
            .faces()
            .find_map(|face| match &face.source {
                fontdb::Source::File(path) => face
                    .families
                    .first()
                    .map(|(family, _)| PlatformEmojiFontCandidate::new(path.clone(), family)),
                _ => None,
            })
            .ok_or_else(|| io::Error::other("a file-backed system font is required"))
    }

    fn policy_for_installed_font(
        mut candidate: PlatformEmojiFontCandidate,
    ) -> io::Result<PlatformFontCatalogPolicy> {
        candidate.expected_raw_file_sha256 = Some(PlatformFontSha256::digest(&fs::read(
            &candidate.source_file_path,
        )?));
        Ok(PlatformFontCatalogPolicy::new(
            PlatformFontProfile::current(),
            Vec::new(),
            Vec::new(),
            vec![candidate],
        ))
    }

    #[test]
    fn second_catalog_construction_reuses_the_first_file_hash_computation() -> io::Result<()> {
        let candidate = installed_font_candidate()?;
        let path = candidate.source_file_path.clone();
        reset_hash_cache_observation(&path)?;
        let policy = policy_for_installed_font(candidate)?;

        let first = PlatformFontCatalog::new(policy.clone());
        let second = PlatformFontCatalog::new(policy);

        assert!(first.emoji_face().is_available());
        assert!(second.emoji_face().is_available());
        assert_eq!(hash_computation_count(&path)?, 1);
        Ok(())
    }

    #[test]
    fn same_length_replacement_rehashes_a_different_file_identity() -> io::Result<()> {
        let path = test_file();
        let replacement = path.with_extension("replacement");
        let previous = path.with_extension("previous");
        let first_bytes = b"first font bytes";
        let second_bytes = b"other font bytes";
        assert_eq!(first_bytes.len(), second_bytes.len());
        fs::write(&path, first_bytes)?;
        reset_hash_cache_observation(&path)?;
        let first = read_cached_file_hash(&path)?;
        let before = file_stamp(&path)?;

        fs::write(&replacement, second_bytes)?;
        fs::rename(&path, &previous)?;
        fs::rename(&replacement, &path)?;
        let after = file_stamp(&path)?;
        let second = read_cached_file_hash(&path)?;

        assert_ne!(before.identity, after.identity);
        assert_ne!(first, second);
        assert_eq!(second, PlatformFontSha256::digest(second_bytes));
        assert_eq!(hash_computation_count(&path)?, 2);
        fs::remove_file(&path)?;
        fs::remove_file(&previous)?;
        Ok(())
    }
}
