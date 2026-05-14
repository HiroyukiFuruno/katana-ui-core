use super::ProgressBar;
use crate::composite::progress_bar::animation::schedule_next_frame;
use crate::composite::progress_bar::render::{render_determinate, render_indeterminate};
use crate::floem_view::FloemColor;
use crate::theme::Theme;
use floem::IntoView;
use floem::reactive::{SignalGet, SignalUpdate, create_rw_signal};
use floem::views::{Decorators, container, dyn_container, label, v_stack};
use std::rc::Rc;

const DEFAULT_FRAME: u64 = 0;
const DETERMINATE_FRAME: u64 = 0;
const WITH_LABEL_GAP: f32 = 4.0;

impl ProgressBar {
    #[must_use]
    pub fn view(self, theme: Theme) -> impl IntoView {
        let resolved = self.resolve(&theme);
        let frame = create_rw_signal(DEFAULT_FRAME);
        let mounted = create_rw_signal(true);
        let track_color = FloemColor::from_token(resolved.track_color);
        let fill_color = FloemColor::from_token(resolved.fill_color);
        let track_width = resolved.track_width;
        let size = resolved.size;
        let radius = resolved.radius;
        let label_text = Rc::new(resolved.label_text);
        let indeterminate = resolved.indeterminate;
        let progress = resolved.progress;
        let speed = resolved.animation_speed_ms;
        let show_label = resolved.show_label;

        if indeterminate && speed > 0 {
            schedule_next_frame(frame, mounted, speed);
        }

        let content = dyn_container(
            move || {
                if indeterminate {
                    frame.try_get().unwrap_or_default()
                } else {
                    DETERMINATE_FRAME
                }
            },
            move |frame| {
                let bar = if indeterminate {
                    render_indeterminate(track_width, size, radius, track_color, fill_color, frame)
                        .into_any()
                } else {
                    render_determinate(progress, track_width, size, radius, track_color, fill_color)
                        .into_any()
                };

                if show_label {
                    let label_text = Rc::clone(&label_text);
                    v_stack((bar, label(move || label_text.as_ref().clone())))
                        .style(|s| s.gap(WITH_LABEL_GAP))
                        .into_any()
                } else {
                    bar.into_any()
                }
            },
        )
        .style(move |style| {
            let style = style.width(track_width);
            if show_label {
                style
            } else {
                style.height(size)
            }
        })
        .on_cleanup(move || mounted.set(false));

        container(content)
            .style(move |style| style.width(track_width))
            .into_any()
    }
}
