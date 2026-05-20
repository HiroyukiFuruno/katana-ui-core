use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiActionSource {
    Generic,
    Click,
    Button,
    Input,
    Checkbox,
    Radio,
    Toggle,
    Progress,
    ColorPicker,
    ColorPickerBlending,
    SlideControl,
    SplitPane,
    SplitPaneReset,
    SplitPaneKeyboard,
    ScrollArea,
    ScrollbarVisibility,
    SearchBox,
    InputSubmit,
    SegmentedToggle,
    SelectBox,
    Accordion,
    AccordionIcon,
    AccordionText,
    AccordionRow,
    Tooltip,
    Popover,
    ModalEscape,
    ModalBackdrop,
    ColorPickerOpen,
    CodeDiffMode,
    CodeDiffDirection,
    CodeDiffExpand,
    CodeDiffScrollSync,
}

impl UiActionSource {
    pub(crate) fn press_name(self) -> &'static str {
        match self {
            Self::Click => "click",
            Self::Button => "button_press",
            Self::SearchBox => "search_submitted",
            Self::InputSubmit => "input_submitted",
            Self::Accordion | Self::AccordionIcon | Self::AccordionText | Self::AccordionRow => {
                "accordion_toggle"
            }
            Self::Tooltip => "tooltip_toggle",
            Self::Popover => "popover_toggle",
            Self::ModalEscape => "modal_escape",
            Self::ModalBackdrop => "modal_backdrop_click",
            Self::ColorPickerOpen => "color_picker_toggle",
            Self::CodeDiffExpand => "code_diff_expand",
            Self::CodeDiffScrollSync => "code_diff_scroll_sync",
            Self::ScrollArea => "scroll_area_press",
            _ => "press",
        }
    }

    pub(crate) fn selection_name(self) -> &'static str {
        match self {
            Self::Checkbox => "checkbox_checked",
            Self::Radio => "radio_selected",
            Self::Toggle => "toggle_checked",
            Self::SegmentedToggle => "segmented_toggle_selected",
            Self::SelectBox => "select_box_selected",
            _ => "set_selected_index",
        }
    }
}
