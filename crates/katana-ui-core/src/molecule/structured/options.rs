use super::model::{CommandPalette, DynamicArrayEditor, TreeView};
use super::types::TreeLineStyle;

macro_rules! structured_options {
    ($name:ident) => {
        impl $name {
            #[must_use]
            pub fn active(mut self, value: impl Into<String>) -> Self {
                self.model.active_id = value.into();
                self
            }

            #[must_use]
            pub fn line_display(mut self, value: bool) -> Self {
                self.model.line_display = value;
                self
            }

            #[must_use]
            pub fn line_style(mut self, value: TreeLineStyle) -> Self {
                self.model.line_style = value;
                self
            }

            #[must_use]
            pub fn line_width(mut self, value: u8) -> Self {
                self.model.line_width = value;
                self
            }

            #[must_use]
            pub fn icons_visible(mut self, value: bool) -> Self {
                self.model.icons_visible = value;
                self
            }

            #[must_use]
            pub fn directory_icon(mut self, value: impl Into<String>) -> Self {
                self.model.directory_icon = value.into();
                self
            }

            #[must_use]
            pub fn file_icon(mut self, value: impl Into<String>) -> Self {
                self.model.file_icon = value.into();
                self
            }

            #[must_use]
            pub fn tree_font_role(mut self, value: impl Into<String>) -> Self {
                self.model.font_role = value.into();
                self
            }
        }
    };
}

structured_options!(TreeView);
structured_options!(CommandPalette);
structured_options!(DynamicArrayEditor);
