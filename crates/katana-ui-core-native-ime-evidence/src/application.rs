mod artifact;
mod deadline;
mod input_method_binding;
use crate::{NativeTranscript, RunnerOptions, renderer::RootRenderer};
use egui_winit::State;
use katana_ui_core::egui::text_command_surface::{
    KucNativeUnicodeEvidenceFrame, KucNativeUnicodeEvidenceSession,
};
use std::{
    io::Write,
    path::PathBuf,
    time::{Duration, Instant},
};
use winit::{
    application::ApplicationHandler,
    event::{Ime, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoopProxy},
    window::{Window, WindowAttributes, WindowId},
};
const WINDOW_WIDTH: f64 = 640.0;
const WINDOW_HEIGHT: f64 = 240.0;

pub(super) struct NativeApplication {
    pub transcript: NativeTranscript,
    pub output: PathBuf,
    window: Option<Window>,
    egui_state: Option<State>,
    session: KucNativeUnicodeEvidenceSession,
    last_frame: Option<KucNativeUnicodeEvidenceFrame>,
    pub(super) artifact_written: bool,
    pub(super) session_error: Option<String>,
    started: Instant,
    timeout: Duration,
    pub(super) timed_out: bool,
    pub(super) window_error: Option<winit::error::OsError>,
    window_id: Option<WindowId>,
    proxy: EventLoopProxy<egui_winit::accesskit_winit::Event>,
    input_method_binding: input_method_binding::InputMethodBinding,
}

impl NativeApplication {
    pub(super) fn new(
        options: RunnerOptions,
        session: KucNativeUnicodeEvidenceSession,
        started: Instant,
        proxy: EventLoopProxy<egui_winit::accesskit_winit::Event>,
    ) -> Self {
        let platform = std::env::consts::OS.to_owned();
        Self {
            output: options.output.clone(),
            transcript: NativeTranscript {
                schema_version: "kuc.native-ime-evidence.v1".into(),
                revision: options.revision.clone(),
                challenge: options.challenge.clone(),
                run_id: options.run_id.clone(),
                producer_id: options.producer_id.clone(),
                runner_id: options.runner_id.clone(),
                platform,
                ..Default::default()
            },
            window: None,
            egui_state: None,
            session,
            last_frame: None,
            artifact_written: false,
            session_error: None,
            started,
            timeout: options.timeout,
            timed_out: false,
            window_error: None,
            window_id: None,
            proxy,
            input_method_binding: input_method_binding::InputMethodBinding::default(),
        }
    }

    fn capture_frame(&mut self) -> bool {
        if let (Some(window), Some(state)) = (&self.window, &mut self.egui_state) {
            let raw_input = state.take_egui_input(window);
            let frame = match self.session.run_frame(raw_input) {
                Ok(frame) => frame,
                Err(error) => {
                    self.session_error = Some(error.to_string());
                    return false;
                }
            };
            self.transcript.egui_frames += 1;
            state.handle_platform_output(window, frame.platform_output.clone());
            if let Err(error) = RootRenderer::present(window, &frame) {
                self.session_error = Some(error);
                return false;
            }
            if let Some((observations, rgba_crop, hashes)) = &frame.observations {
                if observations.committed_text != self.transcript.commit
                    || frame.preedit.as_deref() != Some(self.transcript.preedit.as_str())
                {
                    self.session_error =
                        Some("native transcript does not match root observations".into());
                    return false;
                }
                if let Err(error) = self.write_artifact(observations, rgba_crop, hashes) {
                    self.session_error = Some(error);
                    return false;
                }
            }
            self.last_frame = Some(frame);
        }
        true
    }

    fn prepare_native_ime_event(&mut self, event: &WindowEvent) -> bool {
        if let WindowEvent::Ime(Ime::Preedit(text, _)) = event
            && !text.is_empty()
        {
            let remaining = self.timeout.saturating_sub(self.started.elapsed());
            if remaining.is_zero() {
                self.timed_out = true;
                return false;
            }
            let current =
                match crate::platform::PlatformInputMethod::current_with_timeout(remaining) {
                    Ok(current) => current,
                    Err(error) => {
                        self.session_error = Some(error.to_string());
                        return false;
                    }
                };
            if let Err(error) = self.input_method_binding.observe_preedit(current.clone()) {
                self.session_error = Some(error.into());
                return false;
            }
            self.transcript.input_method = current;
        }
        if let WindowEvent::Ime(Ime::Commit(text)) = event
            && !text.is_empty()
        {
            if !self.input_method_binding.is_pinned() {
                self.session_error = Some("native commit arrived before input method pin".into());
                return false;
            }
            let remaining = self.timeout.saturating_sub(self.started.elapsed());
            if remaining.is_zero() {
                self.timed_out = true;
                return false;
            }
            let current =
                match crate::platform::PlatformInputMethod::current_with_timeout(remaining) {
                    Ok(current) => current,
                    Err(error) => {
                        self.session_error = Some(error.to_string());
                        return false;
                    }
                };
            if let Err(error) = self.input_method_binding.verify_commit(&current) {
                self.session_error = Some(error.into());
                return false;
            }
        }
        true
    }
}

impl ApplicationHandler<egui_winit::accesskit_winit::Event> for NativeApplication {
    fn user_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        event: egui_winit::accesskit_winit::Event,
    ) {
        if self.window_id != Some(event.window_id) {
            return;
        }
        if let egui_winit::accesskit_winit::WindowEvent::ActionRequested(request) =
            event.window_event
            && let Some(state) = &mut self.egui_state
        {
            state.on_accesskit_action_request(request);
        }
        if !self.capture_frame() {
            event_loop.exit();
        }
    }
    fn new_events(&mut self, event_loop: &ActiveEventLoop, _cause: winit::event::StartCause) {
        if self.started.elapsed() >= self.timeout() {
            self.timed_out = true;
            event_loop.exit();
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::WaitUntil(self.started + self.timeout()));
        if self.window.is_some() {
            return;
        }
        let attrs = WindowAttributes::default()
            .with_visible(false)
            .with_title("KUC native IME evidence")
            .with_inner_size(winit::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT));
        let window = match event_loop.create_window(attrs) {
            Ok(window) => window,
            Err(error) => {
                self.window_error = Some(error);
                event_loop.exit();
                return;
            }
        };
        let ctx = self.session.context();
        let mut state = State::new(ctx, egui::ViewportId::ROOT, &window, None, None, None);
        state.init_accesskit(event_loop, &window, self.proxy.clone());
        state.egui_ctx().set_fonts(egui::FontDefinitions::default());
        /* WHY: OS の preedit 配信を開始する前に実ウィンドウの IME を有効化する。 */
        window.set_ime_allowed(true);
        window.set_visible(true);
        window.request_redraw();
        self.egui_state = Some(state);
        self.window = Some(window);
        self.window_id = self.window.as_ref().map(Window::id);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if self.window_id != Some(id) {
            return;
        }
        let is_redraw = matches!(&event, WindowEvent::RedrawRequested);
        if !self.prepare_native_ime_event(&event) {
            event_loop.exit();
            return;
        }
        let Some(window) = self.window.as_ref() else {
            return;
        };
        {
            if let Some(state) = &mut self.egui_state {
                state.set_allow_ime(true);
                let _ = state.on_window_event(window, &event);
            }
        }
        match event {
            WindowEvent::RedrawRequested => {
                if !self.capture_frame() || self.artifact_written {
                    event_loop.exit();
                }
                if self.transcript.egui_frames == 1 {
                    eprintln!(
                        "native-ready pid={} inputmethod=unbound width={} height={}",
                        std::process::id(),
                        WINDOW_WIDTH as u32,
                        WINDOW_HEIGHT as u32
                    );
                    let _ = std::io::stderr().flush();
                }
            }
            WindowEvent::Ime(Ime::Preedit(text, _)) => {
                if !text.is_empty() {
                    self.transcript.preedit = text;
                    self.transcript.commit_sequence = self
                        .transcript
                        .commit_sequence
                        .max(self.transcript.preedit_sequence)
                        + 1;
                    self.transcript.preedit_sequence = self.transcript.commit_sequence;
                    self.transcript.native_event_observed = true;
                    if !self.capture_frame() {
                        event_loop.exit();
                    }
                }
            }
            WindowEvent::Ime(Ime::Commit(text)) => {
                if !text.is_empty() {
                    self.transcript.commit.push_str(&text);
                    self.transcript.commit_sequence = self
                        .transcript
                        .preedit_sequence
                        .max(self.transcript.commit_sequence)
                        + 1;
                    self.transcript.native_event_observed = true;
                    if !self.capture_frame() {
                        event_loop.exit();
                    }
                }
                /* WHY: commit 済み root observation を保存した時点で一実行を終了する。 */
                if !self.transcript.preedit.is_empty() && !self.transcript.commit.is_empty() {
                    event_loop.exit();
                }
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {}
        }
        if !is_redraw && let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }
}

impl NativeApplication {
    fn timeout(&self) -> Duration {
        self.timeout
    }
}
