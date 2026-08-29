use katana_ui_core_egui_adapter::command_chrome::{
    EguiCommandChromeFloatingOutput, EguiCommandChromeOutput, EguiCommandChromeSearchOutput,
};

#[derive(Debug)]
pub(super) struct CommandChromeScriptError(String);

impl std::fmt::Display for CommandChromeScriptError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.0.as_str())
    }
}

impl std::error::Error for CommandChromeScriptError {}

impl CommandChromeScriptError {
    pub(super) fn message(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

#[derive(Debug)]
pub(super) struct CommandChromeScriptFrame {
    pub(super) toolbar: EguiCommandChromeOutput,
    pub(super) floating: EguiCommandChromeFloatingOutput,
    pub(super) search: EguiCommandChromeSearchOutput,
    pub(super) accesskit_labels: Vec<String>,
}

#[derive(Debug)]
pub(super) struct CommandChromeScriptResult {
    pub(super) frames: Vec<CommandChromeScriptFrame>,
}

#[path = "command_chrome_script_frame.rs"]
mod command_chrome_script_frame;
#[path = "command_chrome_script_sequence.rs"]
mod command_chrome_script_sequence;
#[path = "command_chrome_script_sequence_search.rs"]
mod command_chrome_script_sequence_search;

pub(super) fn run_scripted_sequence() -> Result<CommandChromeScriptResult, CommandChromeScriptError>
{
    command_chrome_script_sequence::run_scripted_sequence()
}
