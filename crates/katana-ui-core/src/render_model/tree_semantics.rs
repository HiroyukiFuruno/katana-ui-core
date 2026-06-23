use super::{UiNode, UiNodeKind, UiTree};

pub struct UiTreeSemantics;

impl UiTreeSemantics {
    #[must_use]
    pub fn fingerprint(tree: &UiTree) -> String {
        let mut parts = Vec::new();
        Self::push_node(tree.root(), &mut parts);
        parts.join("|")
    }

    #[must_use]
    pub fn node_count(node: &UiNode) -> usize {
        1 + node.children().iter().map(Self::node_count).sum::<usize>()
    }

    #[must_use]
    pub fn kind_count(node: &UiNode, kind: UiNodeKind) -> usize {
        usize::from(node.kind() == kind)
            + node
                .children()
                .iter()
                .map(|child| Self::kind_count(child, kind))
                .sum::<usize>()
    }

    #[must_use]
    pub fn semantic_text_count(node: &UiNode) -> usize {
        usize::from(node.kind() == UiNodeKind::Text && !node.props().label.trim().is_empty())
            + node
                .children()
                .iter()
                .map(Self::semantic_text_count)
                .sum::<usize>()
    }

    #[must_use]
    pub fn emoji_span_count(node: &UiNode) -> usize {
        Self::text_span_count(node, |span| span.style.emoji)
    }

    fn push_node(node: &UiNode, parts: &mut Vec<String>) {
        parts.push(Self::node_part(node));
        for child in node.children() {
            Self::push_node(child, parts);
        }
    }

    fn node_part(node: &UiNode) -> String {
        let props = node.props();
        format!(
            "{:?}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            node.kind(),
            props.label,
            props.font_role,
            props.text.role,
            props.checked,
            props.interaction.summary(),
            props.common.selectable,
            props.style_classes.join(","),
            Self::context_menu_part(node),
            Self::image_surface_part(node),
            Self::icon_part(node),
            Self::span_part(node),
            node.children().len()
        )
    }

    fn context_menu_part(node: &UiNode) -> String {
        node.props()
            .context_menu
            .items
            .iter()
            .map(|item| format!("{}:{}:{}", item.id, item.label, item.checked))
            .collect::<Vec<_>>()
            .join(",")
    }

    fn image_surface_part(node: &UiNode) -> String {
        let props = &node.props().image_surface;
        if props.fingerprint.is_empty() {
            return String::new();
        }
        format!(
            "{}:{}:{}:{}:{:?}:{}:{}",
            props.fingerprint,
            props.width,
            props.height,
            props.content_scale,
            props.fit,
            props.accessibility_label,
            props
                .highlight_rects
                .iter()
                .map(|highlight| format!(
                    "{}:{}:{}:{}:{}:{}",
                    highlight.rect.x,
                    highlight.rect.y,
                    highlight.rect.width,
                    highlight.rect.height,
                    highlight.current,
                    highlight.label
                ))
                .collect::<Vec<_>>()
                .join(",")
        )
    }

    fn span_part(node: &UiNode) -> String {
        node.props()
            .text
            .spans
            .iter()
            .map(|span| {
                format!(
                    "{}:{}:{}:{}:{}:{}:{}:{}:{}:{:?}",
                    span.text,
                    span.link_target,
                    span.style.bold,
                    span.style.italic,
                    span.style.monospace,
                    span.style.underline,
                    span.style.strikethrough,
                    span.style.highlight,
                    span.style.emoji,
                    span.style.color_rgba
                )
            })
            .collect::<Vec<_>>()
            .join(";")
    }

    fn icon_part(node: &UiNode) -> String {
        let props = node.props();
        let mut parts = Vec::new();
        Self::push_icon_part("node", "", &props.icon, "", &mut parts);
        if let Some(slot) = &props.text_entry.leading_slot {
            Self::push_slot_icon_part("leading", slot, &mut parts);
        }
        for slot in &props.text_entry.trailing_icon_buttons {
            Self::push_slot_icon_part("trailing", slot, &mut parts);
        }
        parts.join(";")
    }

    fn push_slot_icon_part(prefix: &str, slot: &super::UiSlotSpec, parts: &mut Vec<String>) {
        let Some(icon) = &slot.icon else {
            return;
        };
        let callback = slot
            .action
            .as_ref()
            .map_or("", |action| action.callback.as_str());
        Self::push_icon_part(prefix, slot.label.as_str(), icon, callback, parts);
    }

    fn push_icon_part(
        prefix: &str,
        label: &str,
        icon: &super::UiIconProps,
        callback: &str,
        parts: &mut Vec<String>,
    ) {
        if icon.svg_source.trim().is_empty() {
            return;
        }
        parts.push(format!(
            "{}:{}:{}:{}:{:?}:{}:{}:{}",
            prefix,
            label,
            icon.svg_source,
            icon.view_box,
            icon.paint_policy,
            icon.role,
            icon.theme_token,
            callback
        ));
    }

    fn text_span_count(node: &UiNode, predicate: fn(&super::UiTextSpan) -> bool) -> usize {
        node.props()
            .text
            .spans
            .iter()
            .filter(|span| predicate(span))
            .count()
            + node
                .children()
                .iter()
                .map(|child| Self::text_span_count(child, predicate))
                .sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::UiTreeSemantics;
    use crate::atom::{Icon, Text};
    use crate::render_model::{UiTextSpan, UiTextSpanStyle, UiTree};

    #[test]
    fn fingerprint_changes_when_link_target_changes() {
        let left = UiTree::new(Text::new("link").text_spans(vec![link_span("a.md")]));
        let right = UiTree::new(Text::new("link").text_spans(vec![link_span("b.md")]));

        assert_ne!(
            UiTreeSemantics::fingerprint(&left),
            UiTreeSemantics::fingerprint(&right)
        );
    }

    #[test]
    fn counts_emoji_spans() {
        let tree = UiTree::new(Text::new("emoji").text_spans(vec![UiTextSpan::emoji("🙂")]));

        assert_eq!(1, UiTreeSemantics::emoji_span_count(tree.root()));
    }

    #[test]
    fn fingerprint_changes_when_svg_icon_source_changes() {
        let left = UiTree::new(Icon::new("search").svg_source("<svg data-icon=\"search\"/>"));
        let right = UiTree::new(Icon::new("search").svg_source("<svg data-icon=\"close\"/>"));

        assert_ne!(
            UiTreeSemantics::fingerprint(&left),
            UiTreeSemantics::fingerprint(&right)
        );
    }

    fn link_span(target: &str) -> UiTextSpan {
        UiTextSpan {
            text: "link".to_string(),
            style: UiTextSpanStyle {
                underline: true,
                ..UiTextSpanStyle::default()
            },
            link_target: target.to_string(),
        }
    }
}
