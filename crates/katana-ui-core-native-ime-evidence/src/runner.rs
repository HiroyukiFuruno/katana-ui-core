use crate::{NativeTranscript, RunnerError, RunnerOptions, application::NativeApplication};
use katana_ui_core::egui::text_command_surface::{
    KucNativeUnicodeEvidenceSession, KucUnicodeColorGlyphEvidenceOptions,
};
use winit::event_loop::EventLoop;

/// Runs one native window and records only OS-delivered IME events plus the
/// number of `RawInput` frames extracted by `egui-winit`.
pub struct NativeRunner;

impl NativeRunner {
    pub fn run(options: RunnerOptions) -> Result<NativeTranscript, RunnerError> {
        if [
            &options.run_id,
            &options.revision,
            &options.challenge,
            &options.producer_id,
            &options.runner_id,
        ]
        .iter()
        .any(|value| value.trim().is_empty())
            || options.output.as_os_str().is_empty()
            || options.timeout.is_zero()
        {
            return Err(RunnerError::Core(
                "runner options contain an empty binding or zero timeout".into(),
            ));
        }
        if options.output.exists() {
            return Err(RunnerError::Core(
                "output already exists; each native run requires a fresh artifact path".into(),
            ));
        }
        if std::time::Instant::now()
            .checked_add(options.timeout)
            .is_none()
        {
            return Err(RunnerError::Core(
                "runner timeout overflows the monotonic clock".into(),
            ));
        }
        let started = std::time::Instant::now();
        let session =
            KucNativeUnicodeEvidenceSession::new(KucUnicodeColorGlyphEvidenceOptions::default())
                .map_err(|e| RunnerError::Core(e.to_string()))?;
        let event_loop = EventLoop::<egui_winit::accesskit_winit::Event>::with_user_event()
            .build()
            .map_err(RunnerError::EventLoop)?;
        let mut app = NativeApplication::new(options, session, started, event_loop.create_proxy());
        event_loop
            .run_app(&mut app)
            .map_err(RunnerError::EventLoop)?;
        let transcript = app.transcript;
        if let Some(error) = app.window_error {
            return Err(RunnerError::Window(error));
        }
        if let Some(error) = app.session_error {
            return Err(RunnerError::Core(error));
        }
        if app.timed_out {
            return Err(RunnerError::Timeout);
        }
        if !app.artifact_written
            || !transcript.native_event_observed
            || transcript.commit.is_empty()
        {
            return Err(RunnerError::MissingNativeEvent);
        }
        Ok(transcript)
    }
}
