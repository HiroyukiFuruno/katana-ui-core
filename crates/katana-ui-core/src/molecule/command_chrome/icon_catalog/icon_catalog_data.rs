use super::CommandChromeIcon;
use super::CommandChromeIconData;

const fn icon_catalog_entry(
    svg_source: &'static str,
    role: &'static str,
    path_summary: &'static str,
) -> CommandChromeIconData {
    CommandChromeIconData {
        svg_source,
        role,
        path_summary,
    }
}

impl CommandChromeIconData {
    pub(crate) fn entry(icon: CommandChromeIcon) -> CommandChromeIconData {
        match icon {
            CommandChromeIcon::EmphasisStrong => icon_catalog_entry(
                r#"<svg><path fill="currentColor" d="M3 2h5.2c2.1 0 3.5 1.1 3.5 2.8 0 1-.5 1.8-1.4 2.3 1.3.5 2.1 1.4 2.1 2.7 0 2-1.6 3.2-4 3.2H3V2Zm2.4 2v2.4h2.5c.9 0 1.4-.4 1.4-1.2S8.8 4 7.9 4H5.4Zm0 4.4V11h3c1 0 1.6-.4 1.6-1.3S9.4 8.4 8.4 8.4h-3Z"/></svg>"#,
                "command.icon.emphasis-strong",
                "bold-letter-b",
            ),
            CommandChromeIcon::EmphasisItalic => icon_catalog_entry(
                r#"<svg><path fill="currentColor" d="M6.1 2h7.2l-.6 2h-2l-2.4 8h2l-.6 2H2.5l.6-2h2l2.4-8h-2l.6-2Zm1.8 2-2.4 8h2.4l2.4-8H7.9Z"/></svg>"#,
                "command.icon.emphasis-italic",
                "italic-letter-i",
            ),
            CommandChromeIcon::Strike => icon_catalog_entry(
                r#"<svg><path fill="currentColor" d="M5.1 3.2C5.8 2.4 6.8 2 8 2c2.1 0 3.5 1 3.8 2.7l-2 .5c-.2-.8-.8-1.2-1.8-1.2-.8 0-1.4.3-1.4.9 0 .5.5.8 1.8 1.2 2.4.7 3.4 1.7 3.4 3.1 0 1.8-1.5 2.8-3.7 2.8-2.2 0-3.7-1-4-2.8l2-.5c.2.9.9 1.3 2 1.3.9 0 1.6-.3 1.6-.9 0-.5-.5-.8-1.9-1.2C5.4 7.2 4.2 6.2 4.2 4.8c0-.6.3-1.2.9-1.6ZM2 7h12v1.7H2V7Z"/></svg>"#,
                "command.icon.strike",
                "struck-letter-s",
            ),
            CommandChromeIcon::InlineCode => icon_catalog_entry(
                r#"<svg><path fill="currentColor" d="m6.2 3.2 1.3 1.3L5 7l2.5 2.5-1.3 1.3L2.4 7l3.8-3.8Zm3.6 0L13.6 7l-3.8 3.8-1.3-1.3L11 7 8.5 4.5l1.3-1.3Z"/></svg>"#,
                "command.icon.inline-code",
                "inline-angle-brackets",
            ),
            CommandChromeIcon::HeadingOne => icon_catalog_entry(
                r#"<svg><path fill="currentColor" d="M2 2h2v5h3V2h2v12H7V9H4v5H2V2Zm9 3 2-1h1v10h-2V6.4l-1 .5V5Z"/></svg>"#,
                "command.icon.heading-one",
                "heading-h1",
            ),
            CommandChromeIcon::HeadingTwo => icon_catalog_entry(
                r#"<svg><path fill="currentColor" d="M2 2h2v5h3V2h2v12H7V9H4v5H2V2Zm9 10c0-1.2.4-2 1.7-3.1.9-.8 1.3-1.2 1.3-1.9 0-.6-.4-1-1-1-.7 0-1.1.4-1.2 1.2l-1.8-.4C11.2 5.6 12.2 5 13.7 5 15.2 5 16 5.8 16 7c0 1.2-.7 2-1.9 3l-.9.8H16V12h-5Z"/></svg>"#,
                "command.icon.heading-two",
                "heading-h2",
            ),
            CommandChromeIcon::HeadingThree => icon_catalog_entry(
                r#"<svg><path fill="currentColor" d="M2 2h2v5h3V2h2v12H7V9H4v5H2V2Zm9.1 3.3c.5-.2 1.1-.3 1.8-.3 1.7 0 2.7.8 2.7 2 0 .8-.5 1.4-1.2 1.7.9.2 1.5.9 1.5 1.8 0 1.4-1.1 2.3-2.9 2.3-.8 0-1.5-.2-2.1-.5l.5-1.5c.5.3 1 .4 1.5.4.7 0 1.1-.3 1.1-.8s-.4-.8-1.2-.8h-.7V8.2h.6c.7 0 1.1-.3 1.1-.8s-.4-.7-1-.7c-.4 0-.8.1-1.2.3l-.5-1.7Z"/></svg>"#,
                "command.icon.heading-three",
                "heading-h3",
            ),
            CommandChromeIcon::ListUnordered => icon_catalog_entry(
                r#"<svg><path fill="currentColor" d="M2 3h2v2H2V3Zm4 0h8v2H6V3ZM2 7h2v2H2V7Zm4 0h8v2H6V7Zm-4 4h2v2H2v-2Zm4 0h8v2H6v-2Z"/></svg>"#,
                "command.icon.list-unordered",
                "unordered-list",
            ),
            CommandChromeIcon::ListOrdered => icon_catalog_entry(
                r#"<svg><path fill="currentColor" d="M2 3h1.2L4 2.5h.8V6H3.5V4.1L2.8 4.5 2 3Zm4 0h8v2H6V3ZM2 7.2c.5-.2 1-.3 1.5-.3 1.1 0 1.7.5 1.7 1.3 0 .6-.4 1-1.1 1.4l-.6.4h1.8v1.5H2v-1.2l1.5-1.1c.4-.3.6-.5.6-.7 0-.2-.2-.3-.5-.3-.4 0-.8.1-1.2.3L2 7.2ZM6 8h8v2H6V8Zm-4 4h2v2H2v-2Zm4 0h8v2H6v-2Z"/></svg>"#,
                "command.icon.list-ordered",
                "ordered-list",
            ),
            CommandChromeIcon::Quote => icon_catalog_entry(
                r#"<svg><path fill="currentColor" d="M2 3h2v10H2V3Zm3 2c0-1.2.8-2 2-2h2v2H7c-.2 0-.4.2-.4.4V6h2.1v3H5V5Zm5 0c0-1.2.8-2 2-2h2v2h-2c-.2 0-.4.2-.4.4V6h2.1v3H10V5Z"/></svg>"#,
                "command.icon.quote",
                "quotation-mark",
            ),
            CommandChromeIcon::Rule => icon_catalog_entry(
                r#"<svg><path fill="currentColor" d="M2 7h12v2H2V7Z"/></svg>"#,
                "command.icon.rule",
                "horizontal-rule",
            ),
            CommandChromeIcon::CodeBlock => icon_catalog_entry(
                r#"<svg><path fill="currentColor" d="m6.4 4.1 1.2 1.4L5.5 7l2.1 1.5-1.2 1.4L2.5 7l3.9-2.9Zm3.2 0L13.5 7l-3.9 2.9-1.2-1.4L10.5 7 8.4 5.5l1.2-1.4ZM8.1 3h1.8L7.9 11H6.1l2-8Z"/></svg>"#,
                "command.icon.code-block",
                "code-angle-brackets",
            ),
            CommandChromeIcon::TaskList => icon_catalog_entry(
                r#"<svg><path fill="currentColor" d="M2 3h2v2H2V3Zm1 1 1-1 .8.8L3 5.6 2.2 4.8 3 4Zm3-1h8v2H6V3ZM2 7h2v2H2V7Zm1 1 1-1 .8.8L3 9.6l-.8-.8L3 8Zm3-1h8v2H6V7Zm-4 4h2v2H2v-2Zm4 0h8v2H6v-2Z"/></svg>"#,
                "command.icon.task-list",
                "task-list",
            ),
            CommandChromeIcon::Link => icon_catalog_entry(
                r#"<svg><path fill="currentColor" d="M6.5 10.8H5a3 3 0 0 1 0-6h2.5v1.7H5a1.3 1.3 0 0 0 0 2.6h1.5v1.7Zm3-5.6H11a3 3 0 0 1 0 6H8.5V9.5H11a1.3 1.3 0 0 0 0-2.6H9.5V5.2ZM6 7h4v2H6V7Z"/></svg>"#,
                "command.icon.link",
                "linked-chain",
            ),
            CommandChromeIcon::Table => icon_catalog_entry(
                r#"<svg><path fill="currentColor" d="M2 2h12v12H2V2Zm2 2v2h3V4H4Zm5 0v2h3V4H9ZM4 8v2h3V8H4Zm5 0v2h3V8H9ZM4 12v0h3v0H4Zm5 0v0h3v0H9ZM3 3v10h10V3H3Z"/></svg>"#,
                "command.icon.table",
                "table-grid",
            ),
            CommandChromeIcon::Image => icon_catalog_entry(
                r#"<svg><path fill="currentColor" d="M2 3h12v10H2z M3 4v8h10V4z M4 10l2.5-3 2 2 1.5-1.5L13 11v1H3z M10.5 6.5a1.5 1.5 0 1 0 0-3 1.5 1.5 0 0 0 0 3z"/></svg>"#,
                "command.icon.image",
                "image-frame-mountain-and-point",
            ),
        }
    }
}
