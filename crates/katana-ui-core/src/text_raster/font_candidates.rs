use crate::text_raster::catalog_types::{PlatformEmojiFontCandidate, PlatformFontProfile};
use std::path::PathBuf;

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
            PlatformFontProfile::Linux => (
                paths(&["/usr/share/fonts/truetype/noto/NotoColorEmoji.ttf"]),
                "Noto Color Emoji",
            ),
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
        for (profile, proportional_count, monospace_count) in [
            (PlatformFontProfile::MacOs, 2, 2),
            (PlatformFontProfile::Windows, 2, 1),
            (PlatformFontProfile::Linux, 2, 2),
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
            assert_eq!(emoji.len(), 1);
            assert_eq!(
                emoji[0].expected_family,
                profile
                    .expected_emoji_family()
                    .expect("supported profiles have an emoji family")
            );
        }
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
