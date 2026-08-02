use crate::render_model::UiNode;
use serde::{Deserialize, Serialize};

const STYLE_RGBA_CHANNELS: usize = 4;
pub type StyleRgba = [u8; STYLE_RGBA_CHANNELS];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StyleClass(String);

impl StyleClass {
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StyleProperty {
    Background,
    Foreground,
    Border,
    BorderWidth,
    Radius,
    Padding,
    Gap,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StyleValue {
    ColorToken(String),
    Rgba(StyleRgba),
    Px(f32),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StyleDeclaration {
    pub property: StyleProperty,
    pub value: StyleValue,
}

impl StyleDeclaration {
    #[must_use]
    pub fn new(property: StyleProperty, value: StyleValue) -> Self {
        Self { property, value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StyleSelector {
    Class(StyleClass),
}

impl StyleSelector {
    #[must_use]
    pub fn class(value: impl Into<String>) -> Self {
        Self::Class(StyleClass::new(value))
    }

    fn matches(&self, node: &UiNode) -> bool {
        match self {
            Self::Class(class) => node
                .props()
                .style_classes
                .iter()
                .any(|candidate| candidate == class.as_str()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StyleRule {
    selector: StyleSelector,
    declarations: Vec<StyleDeclaration>,
}

impl StyleRule {
    #[must_use]
    pub fn class(value: impl Into<String>, declarations: Vec<StyleDeclaration>) -> Self {
        Self {
            selector: StyleSelector::class(value),
            declarations,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct StyleSheet {
    rules: Vec<StyleRule>,
}

impl StyleSheet {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn rule(mut self, rule: StyleRule) -> Self {
        self.rules.push(rule);
        self
    }

    #[must_use]
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    #[must_use]
    pub fn resolve(&self, node: &UiNode) -> ResolvedStyle {
        let mut resolved = ResolvedStyle::default();
        for rule in &self.rules {
            if rule.selector.matches(node) {
                resolved.apply(&rule.declarations);
            }
        }
        resolved
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolvedStyle {
    declarations: Vec<StyleDeclaration>,
}

impl ResolvedStyle {
    #[must_use]
    pub fn declarations(&self) -> &[StyleDeclaration] {
        &self.declarations
    }

    #[must_use]
    pub fn value(&self, property: StyleProperty) -> Option<&StyleValue> {
        self.declarations
            .iter()
            .find(|it| it.property == property)
            .map(|it| &it.value)
    }

    fn apply(&mut self, declarations: &[StyleDeclaration]) {
        for declaration in declarations {
            if let Some(existing) = self
                .declarations
                .iter_mut()
                .find(|it| it.property == declaration.property)
            {
                *existing = declaration.clone();
            } else {
                self.declarations.push(declaration.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{StyleDeclaration, StyleProperty, StyleRule, StyleSheet, StyleValue};
    use crate::atom::Button;
    use crate::render_model::UiNode;

    #[test]
    fn style_sheet_can_be_replaced_after_component_creation() {
        let node = UiNode::from(Button::new("Save")).style_class("primary");
        let blue = StyleSheet::new().rule(StyleRule::class(
            "primary",
            vec![StyleDeclaration::new(
                StyleProperty::Background,
                StyleValue::ColorToken("accent".to_string()),
            )],
        ));
        let red = StyleSheet::new().rule(StyleRule::class(
            "primary",
            vec![StyleDeclaration::new(
                StyleProperty::Background,
                StyleValue::ColorToken("danger".to_string()),
            )],
        ));

        assert_eq!(
            Some(&StyleValue::ColorToken("accent".to_string())),
            blue.resolve(&node).value(StyleProperty::Background)
        );
        assert_eq!(
            Some(&StyleValue::ColorToken("danger".to_string())),
            red.resolve(&node).value(StyleProperty::Background)
        );
    }

    #[test]
    fn matching_rules_replace_prior_declarations_and_expose_ordered_values() {
        let node = UiNode::from(Button::new("Save")).style_class("primary");
        let sheet = StyleSheet::new()
            .rule(StyleRule::class(
                "primary",
                vec![StyleDeclaration::new(
                    StyleProperty::Background,
                    StyleValue::ColorToken("before".to_string()),
                )],
            ))
            .rule(StyleRule::class(
                "primary",
                vec![
                    StyleDeclaration::new(
                        StyleProperty::Background,
                        StyleValue::ColorToken("after".to_string()),
                    ),
                    StyleDeclaration::new(
                        StyleProperty::Foreground,
                        StyleValue::ColorToken("text".to_string()),
                    ),
                ],
            ));

        let resolved = sheet.resolve(&node);
        assert_eq!(2, resolved.declarations().len());
        assert_eq!(
            Some(&StyleValue::ColorToken("after".to_string())),
            resolved.value(StyleProperty::Background)
        );
    }
}
