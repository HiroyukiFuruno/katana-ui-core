use super::{PlatformFontCatalog, catalog_cache};
use crate::{
    PlatformEmojiFontCandidate, PlatformEmojiFontLoadError, PlatformEmojiFontLoader,
    PlatformFontCatalogPolicy, PlatformFontProfile, PlatformFontSha256,
};
use cosmic_text::FontSystem;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_TEST_FILE: AtomicU64 = AtomicU64::new(0);

fn test_file() -> PathBuf {
    let serial = NEXT_TEST_FILE.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "kuc-text-raster-catalog-cache-{}-{serial}.font",
        std::process::id()
    ))
}

fn reset_hash_cache(path: &std::path::Path) -> io::Result<()> {
    let path = fs::canonicalize(path)?;
    let cache = catalog_cache::FILE_HASH_CACHE
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
    cache
        .lock()
        .map_err(|_| io::Error::other("file hash cache lock poisoned"))?
        .remove(&path);
    Ok(())
}

fn installed_font_candidate() -> io::Result<PlatformEmojiFontCandidate> {
    FontSystem::new()
        .db()
        .faces()
        .find_map(|face| match &face.source {
            cosmic_text::fontdb::Source::File(path) => face
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
    reset_hash_cache(&path)?;
    let policy = policy_for_installed_font(candidate)?;

    let first = PlatformFontCatalog::new(policy.clone());
    let cached_after_first = catalog_cache::FILE_HASH_CACHE
        .get()
        .expect("catalog construction initializes the file hash cache")
        .lock()
        .map_err(|_| io::Error::other("file hash cache lock poisoned"))?
        .get(&fs::canonicalize(&path)?)
        .cloned()
        .expect("font hash must be retained in the cache");
    let second = PlatformFontCatalog::new(policy);
    let cached_after_second = catalog_cache::FILE_HASH_CACHE
        .get()
        .expect("catalog construction initializes the file hash cache")
        .lock()
        .map_err(|_| io::Error::other("file hash cache lock poisoned"))?
        .get(&fs::canonicalize(&path)?)
        .cloned()
        .expect("font hash must remain in the cache");

    assert!(first.emoji_face().is_available());
    assert!(second.emoji_face().is_available());
    assert_eq!(cached_after_first, cached_after_second);
    Ok(())
}

#[test]
fn loaded_family_lookup_ignores_non_file_font_sources() -> io::Result<()> {
    let candidate = installed_font_candidate()?;
    let bytes = fs::read(&candidate.source_file_path)?;
    let mut font_system = FontSystem::new();
    font_system.db_mut().load_font_data(bytes);

    assert!(
        catalog_cache::family_from_loaded_file(
            &font_system,
            &candidate.source_file_path,
            &candidate.expected_family,
        )
        .is_some()
    );
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
    reset_hash_cache(&path)?;
    let first = catalog_cache::read_cached_file_hash(&path)?;
    let canonical_path = fs::canonicalize(&path)?;
    let cached_after_first = catalog_cache::FILE_HASH_CACHE
        .get()
        .expect("first read initializes the file hash cache")
        .lock()
        .map_err(|_| io::Error::other("file hash cache lock poisoned"))?
        .get(&canonical_path)
        .cloned()
        .expect("first hash must be retained in the cache");
    let before = catalog_cache::file_stamp(&path)?;

    fs::write(&replacement, second_bytes)?;
    fs::rename(&path, &previous)?;
    fs::rename(&replacement, &path)?;
    let after = catalog_cache::file_stamp(&path)?;
    let second = catalog_cache::read_cached_file_hash(&path)?;
    let cached_after_second = catalog_cache::FILE_HASH_CACHE
        .get()
        .expect("second read retains the file hash cache")
        .lock()
        .map_err(|_| io::Error::other("file hash cache lock poisoned"))?
        .get(&canonical_path)
        .cloned()
        .expect("second hash must be retained in the cache");

    assert_ne!(before.identity, after.identity);
    assert_ne!(first, second);
    assert_eq!(second, PlatformFontSha256::digest(second_bytes));
    assert_ne!(cached_after_first, cached_after_second);
    fs::remove_file(&path)?;
    fs::remove_file(&previous)?;
    Ok(())
}

#[test]
fn system_loader_classifies_missing_and_invalid_font_files() -> io::Result<()> {
    let missing = test_file();
    let mut font_system = FontSystem::new();
    let mut loader = super::SystemEmojiFontLoader {
        font_system: &mut font_system,
        load_attempts: 0,
    };
    let error = loader
        .load(&PlatformEmojiFontCandidate::new(
            missing.clone(),
            "Missing family",
        ))
        .expect_err("missing font must fail");
    assert!(matches!(
        error,
        PlatformEmojiFontLoadError::Missing { source_file_path }
            if source_file_path == missing
    ));

    let unreadable = std::env::temp_dir();
    let error = loader
        .load(&PlatformEmojiFontCandidate::new(
            unreadable.clone(),
            "Directory is not a font",
        ))
        .expect_err("directory input must fail");
    assert!(matches!(
        error,
        PlatformEmojiFontLoadError::Io { source_file_path, .. }
            if source_file_path == unreadable
    ));

    let invalid = test_file();
    fs::write(&invalid, b"not a font")?;
    let error = loader
        .load(&PlatformEmojiFontCandidate::new(
            invalid.clone(),
            "Invalid family",
        ))
        .expect_err("invalid font must fail");
    assert!(matches!(
        error,
        PlatformEmojiFontLoadError::FaceNotFound { source_file_path }
            if source_file_path == invalid
    ));
    assert_eq!(loader.load_attempts, 3);
    fs::remove_file(invalid)?;

    let mapped = super::font_file_load_error(
        &PlatformEmojiFontCandidate::new("mapped.font".into(), "Mapped family"),
        io::Error::other("mapped error"),
    );
    assert!(matches!(
        mapped,
        PlatformEmojiFontLoadError::Io { source_file_path, message }
            if source_file_path == Path::new("mapped.font") && message == "mapped error"
    ));
    Ok(())
}
