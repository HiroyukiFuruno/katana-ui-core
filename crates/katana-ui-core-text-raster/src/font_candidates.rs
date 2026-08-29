use crate::catalog_types::{PlatformEmojiFontCandidate, PlatformFontProfile};
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
        let Some(family) = profile.expected_emoji_family() else {
            return Vec::new();
        };
        let paths = match profile {
            PlatformFontProfile::MacOs => paths(&["/System/Library/Fonts/Apple Color Emoji.ttc"]),
            PlatformFontProfile::Windows => paths(&["C:/Windows/Fonts/seguiemj.ttf"]),
            PlatformFontProfile::Linux => {
                paths(&["/usr/share/fonts/truetype/noto/NotoColorEmoji.ttf"])
            }
            PlatformFontProfile::Unsupported => Vec::new(),
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
