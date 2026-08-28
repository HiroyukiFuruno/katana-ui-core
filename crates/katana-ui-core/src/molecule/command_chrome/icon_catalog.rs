use crate::render_model::{UiIconProps, UiSvgPaintPolicy};
mod icon_catalog_data;

const COMMAND_CHROME_ICON_COUNT: usize = 16;
const COMMAND_CHROME_ICON_VIEW_BOX_SIZE: usize = 16;

#[derive(Debug, Clone, Copy)]
pub(crate) struct CommandChromeIconData {
    svg_source: &'static str,
    role: &'static str,
    path_summary: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandChromeIcon {
    EmphasisStrong,
    EmphasisItalic,
    Strike,
    InlineCode,
    HeadingOne,
    HeadingTwo,
    HeadingThree,
    ListUnordered,
    ListOrdered,
    Quote,
    Rule,
    CodeBlock,
    TaskList,
    Link,
    Table,
    Image,
}

impl CommandChromeIcon {
    #[must_use]
    pub const fn all() -> [Self; COMMAND_CHROME_ICON_COUNT] {
        [
            Self::EmphasisStrong,
            Self::EmphasisItalic,
            Self::Strike,
            Self::InlineCode,
            Self::HeadingOne,
            Self::HeadingTwo,
            Self::HeadingThree,
            Self::ListUnordered,
            Self::ListOrdered,
            Self::Quote,
            Self::Rule,
            Self::CodeBlock,
            Self::TaskList,
            Self::Link,
            Self::Table,
            Self::Image,
        ]
    }

    #[must_use]
    pub fn icon_props(self) -> UiIconProps {
        let CommandChromeIconData {
            svg_source,
            role,
            path_summary,
        } = CommandChromeIconData::entry(self);

        let view_box =
            format!("0 0 {COMMAND_CHROME_ICON_VIEW_BOX_SIZE} {COMMAND_CHROME_ICON_VIEW_BOX_SIZE}");
        UiIconProps {
            svg_source: svg_source.to_owned(),
            view_box,
            path_summary: path_summary.to_owned(),
            paint_policy: UiSvgPaintPolicy::CurrentColor,
            role: role.to_owned(),
            ..UiIconProps::default()
        }
    }
}
