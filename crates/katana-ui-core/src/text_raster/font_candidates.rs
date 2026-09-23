use crate::text_raster::catalog_types::{
    PlatformEmojiFontCandidate, PlatformFontProfile, PlatformFontSha256,
};
use std::path::PathBuf;

const LINUX_GITHUB_RUNNER_EMOJI_SHA256: PlatformFontSha256 = PlatformFontSha256::from_bytes([
    0x93, 0xcd, 0xc4, 0xee, 0x9a, 0xa4, 0x0e, 0x2a, 0xfc, 0xee, 0xcc, 0x63, 0xda, 0x0c, 0xa0, 0x5e,
    0xc7, 0xaa, 0xb4, 0xbe, 0xc9, 0x91, 0xec, 0xe5, 0x1a, 0x6b, 0x52, 0x38, 0x9f, 0x48, 0xa4, 0x77,
]);
const LINUX_COVERAGE_RUNTIME_EMOJI_SHA256: PlatformFontSha256 = PlatformFontSha256::from_bytes([
    0xe5, 0x89, 0x9e, 0xd3, 0x8b, 0x8e, 0xd8, 0x3e, 0x08, 0xbd, 0x3a, 0xc5, 0xde, 0x09, 0x79, 0x1e,
    0x9d, 0x19, 0xd2, 0x88, 0x33, 0x3a, 0x79, 0x6d, 0xe1, 0xd3, 0x5a, 0xd1, 0x73, 0x96, 0xf1, 0xec,
]);

pub(crate) struct PlatformFontCatalogCandidates;

impl PlatformFontCatalogCandidates {
    pub(crate) fn proportional_for(profile: PlatformFontProfile) -> Vec<PathBuf> {
        match profile {
            PlatformFontProfile::MacOs => paths(&[
                "/System/Library/Fonts/ヒラギノ角ゴシック W3.ttc",
                "/System/Library/Fonts/SFNS.ttf",
            ]),
            PlatformFontProfile::Windows => paths(&[
                "C:/Windows/Fonts/segoeui.ttf",
                "C:/Windows/Fonts/meiryo.ttc",
            ]),
            PlatformFontProfile::Linux => paths(&[
                "/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf",
                "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            ]),
            PlatformFontProfile::Unsupported => Vec::new(),
        }
    }

    pub(crate) fn monospace_for(profile: PlatformFontProfile) -> Vec<PathBuf> {
        match profile {
            PlatformFontProfile::MacOs => paths(&[
                "/System/Library/Fonts/Menlo.ttc",
                "/System/Library/Fonts/SFNSMono.ttf",
            ]),
            PlatformFontProfile::Windows => paths(&["C:/Windows/Fonts/consola.ttf"]),
            PlatformFontProfile::Linux => paths(&[
                "/usr/share/fonts/truetype/noto/NotoSansMono-Regular.ttf",
                "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf",
            ]),
            PlatformFontProfile::Unsupported => Vec::new(),
        }
    }

    pub(crate) fn emoji_for(profile: PlatformFontProfile) -> Vec<PlatformEmojiFontCandidate> {
        let (paths, family) = match profile {
            PlatformFontProfile::MacOs => (
                paths(&["/System/Library/Fonts/Apple Color Emoji.ttc"]),
                "Apple Color Emoji",
            ),
            PlatformFontProfile::Windows => {
                (paths(&["C:/Windows/Fonts/seguiemj.ttf"]), "Segoe UI Emoji")
            }
            PlatformFontProfile::Linux => {
                let path = PathBuf::from("/usr/share/fonts/truetype/noto/NotoColorEmoji.ttf");
                return [
                    LINUX_GITHUB_RUNNER_EMOJI_SHA256,
                    LINUX_COVERAGE_RUNTIME_EMOJI_SHA256,
                ]
                .into_iter()
                .map(|hash| {
                    PlatformEmojiFontCandidate::new(path.clone(), "Noto Color Emoji")
                        .with_expected_raw_file_sha256(hash)
                })
                .collect();
            }
            PlatformFontProfile::Unsupported => return Vec::new(),
        };
        paths
            .into_iter()
            .map(|path| PlatformEmojiFontCandidate::new(path, family))
            .collect()
    }
}

fn paths(paths: &[&str]) -> Vec<PathBuf> {
    paths.iter().map(PathBuf::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supported_profiles_expose_their_platform_font_candidates() {
        for (profile, proportional_count, monospace_count, emoji_count) in [
            (PlatformFontProfile::MacOs, 2, 2, 1),
            (PlatformFontProfile::Windows, 2, 1, 1),
            (PlatformFontProfile::Linux, 2, 2, 2),
        ] {
            assert_eq!(
                PlatformFontCatalogCandidates::proportional_for(profile).len(),
                proportional_count
            );
            assert_eq!(
                PlatformFontCatalogCandidates::monospace_for(profile).len(),
                monospace_count
            );
            let emoji = PlatformFontCatalogCandidates::emoji_for(profile);
            assert_eq!(emoji.len(), emoji_count);
            assert_eq!(
                emoji[0].expected_family,
                profile
                    .expected_emoji_family()
                    .expect("supported profiles have an emoji family")
            );
        }
    }

    #[test]
    fn linux_release_profiles_are_pinned_in_the_public_catalog() {
        let hashes = PlatformFontCatalogCandidates::emoji_for(PlatformFontProfile::Linux)
            .into_iter()
            .map(|candidate| {
                candidate
                    .expected_raw_file_sha256
                    .expect("Linux release candidates must be pinned")
                    .to_hex()
            })
            .collect::<Vec<_>>();

        assert_eq!(
            hashes,
            vec![
                "93cdc4ee9aa40e2afceecc63da0ca05ec7aab4bec991ece51a6b52389f48a477",
                "e5899ed38b8ed83e08bd3ac5de09791e9d19d288333a796de1d35ad17396f1ec",
            ]
        );
    }

    #[test]
    fn unsupported_profile_has_no_platform_font_candidates() {
        assert!(
            PlatformFontCatalogCandidates::proportional_for(PlatformFontProfile::Unsupported)
                .is_empty()
        );
        assert!(
            PlatformFontCatalogCandidates::monospace_for(PlatformFontProfile::Unsupported)
                .is_empty()
        );
        assert!(
            PlatformFontCatalogCandidates::emoji_for(PlatformFontProfile::Unsupported).is_empty()
        );
    }
}
