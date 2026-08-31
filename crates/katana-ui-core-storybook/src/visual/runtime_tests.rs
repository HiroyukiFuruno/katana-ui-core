use super::{
    ModalWindowPlacementError, StorybookDependencyRuntimeReport, StorybookKeyboardRuntimeReport,
    StorybookMouseTraceRuntimeReport, StorybookRuntimeReport, StorybookVisualError,
    StorybookWindowRun,
};

#[test]
fn runtime_and_window_reports_include_every_field() {
    let runtime = StorybookRuntimeReport {
        state_reflected: true,
        overlay_rendered: false,
        modal_plan_same_display: true,
        modal_plan_frontmost: false,
    };
    assert!(runtime.summary().contains("state_reflected=true"));
    let window = StorybookWindowRun {
        frames: 3,
        modal_window_opened: true,
        same_display: true,
        frontmost: false,
        state_reflected: true,
        overlay_rendered: false,
    };
    assert!(window.summary().contains("frames=3"));

    for report in [
        StorybookKeyboardRuntimeReport {
            checkbox_focused: false,
            checkbox_toggled: true,
            modal_closed: true,
            unavailable_clipboard_ignored: true,
        },
        StorybookKeyboardRuntimeReport {
            checkbox_focused: true,
            checkbox_toggled: false,
            modal_closed: true,
            unavailable_clipboard_ignored: true,
        },
        StorybookKeyboardRuntimeReport {
            checkbox_focused: true,
            checkbox_toggled: true,
            modal_closed: false,
            unavailable_clipboard_ignored: true,
        },
        StorybookKeyboardRuntimeReport {
            checkbox_focused: true,
            checkbox_toggled: true,
            modal_closed: true,
            unavailable_clipboard_ignored: false,
        },
    ] {
        assert!(!report.passed());
    }
    assert!(
        StorybookKeyboardRuntimeReport {
            checkbox_focused: true,
            checkbox_toggled: true,
            modal_closed: true,
            unavailable_clipboard_ignored: true,
        }
        .passed()
    );

    for report in [
        StorybookMouseTraceRuntimeReport {
            pointer_values_formatted: false,
            optional_index_formatted: true,
            progress_segment_formatted: true,
        },
        StorybookMouseTraceRuntimeReport {
            pointer_values_formatted: true,
            optional_index_formatted: false,
            progress_segment_formatted: true,
        },
        StorybookMouseTraceRuntimeReport {
            pointer_values_formatted: true,
            optional_index_formatted: true,
            progress_segment_formatted: false,
        },
    ] {
        assert!(!report.passed());
    }
    assert!(
        StorybookMouseTraceRuntimeReport {
            pointer_values_formatted: true,
            optional_index_formatted: true,
            progress_segment_formatted: true,
        }
        .passed()
    );

    for report in [
        StorybookDependencyRuntimeReport {
            missing_tab_group_close_ignored: false,
            same_tab_group_move_ignored: true,
            tab_group_removal_emitted: true,
        },
        StorybookDependencyRuntimeReport {
            missing_tab_group_close_ignored: true,
            same_tab_group_move_ignored: false,
            tab_group_removal_emitted: true,
        },
        StorybookDependencyRuntimeReport {
            missing_tab_group_close_ignored: true,
            same_tab_group_move_ignored: true,
            tab_group_removal_emitted: false,
        },
    ] {
        assert!(!report.passed());
    }
    assert!(
        StorybookDependencyRuntimeReport {
            missing_tab_group_close_ignored: true,
            same_tab_group_move_ignored: true,
            tab_group_removal_emitted: true,
        }
        .passed()
    );
}

#[test]
fn visual_error_formats_window_and_placement_sources() {
    let window = StorybookVisualError::from(minifb::Error::WindowCreate("test".to_string()));
    assert_eq!("Failed to create window", window.to_string());
    let placement = StorybookVisualError::from(ModalWindowPlacementError::ParentOutsideDisplay);
    assert_eq!("ParentOutsideDisplay", placement.to_string());
    let eframe = StorybookVisualError::from(eframe::Error::AppCreation(Box::new(
        std::io::Error::other("eframe failed"),
    )));
    assert!(eframe.to_string().contains("eframe failed"));
}
