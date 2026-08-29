use super::{CommandChromeSearchPresentation, CommandChromeSearchStrip};

impl CommandChromeSearchStrip {
    /// Applies external state and presentation without producing a search command event.
    pub fn synchronize_presentation(&mut self, value: CommandChromeSearchPresentation) -> bool {
        let mut changed = self.strip.synchronize_presentation(
            value.query,
            value.options,
            value.result_count,
            value.active_index,
            value.replace_mode,
            value.replace_value,
        );
        if self.strings != value.strings {
            self.strings = value.strings;
            changed = true;
        }
        if self.capabilities != value.capabilities {
            self.capabilities = value.capabilities;
            changed = true;
        }
        if self.icons != value.icons {
            self.icons = value.icons;
            changed = true;
        }
        changed
    }
}
