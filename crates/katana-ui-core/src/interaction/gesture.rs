use crate::render_model::{UiRect, UiStateId};
mod types;

use types::ActivePointer;
pub use types::*;

impl UiSurfaceGestureController {
    #[must_use]
    pub fn new(surfaces: impl IntoIterator<Item = UiGestureSurface>) -> Self {
        Self {
            surfaces: surfaces.into_iter().collect(),
            active_pointer: None,
        }
    }

    #[must_use]
    pub fn hit_target(&self, point: UiSurfacePoint) -> Option<&UiGestureSurface> {
        self.surfaces
            .iter()
            .rev()
            .find(|surface| surface.hit(point))
    }

    pub fn apply(&mut self, input: UiSurfaceGestureInput) -> UiSurfaceGestureOutcome {
        match input {
            UiSurfaceGestureInput::PointerDown {
                pointer_id,
                position,
            } => {
                let Some(surface) = self.hit_target(position) else {
                    return UiSurfaceGestureOutcome::unhandled(None);
                };
                let target = surface.target.clone();
                if !surface.capabilities.pointer_pan {
                    return UiSurfaceGestureOutcome::unhandled(Some(target));
                }
                self.active_pointer = Some(ActivePointer {
                    pointer_id,
                    target: target.clone(),
                    last: position,
                });
                handled(
                    target,
                    UiSurfaceGestureInput::PointerDown {
                        pointer_id,
                        position,
                    },
                    None,
                )
            }
            UiSurfaceGestureInput::PointerMove {
                pointer_id,
                position,
            } => {
                let Some(active) = self
                    .active_pointer
                    .as_mut()
                    .filter(|value| value.pointer_id == pointer_id)
                else {
                    return UiSurfaceGestureOutcome::unhandled(
                        self.hit_target(position)
                            .map(|surface| surface.target.clone()),
                    );
                };
                let delta_x = (position.x - active.last.x) as f32;
                let delta_y = (position.y - active.last.y) as f32;
                active.last = position;
                handled(
                    active.target.clone(),
                    UiSurfaceGestureInput::PointerMove {
                        pointer_id,
                        position,
                    },
                    Some(UiSurfaceGestureCommand::PanBy { delta_x, delta_y }),
                )
            }
            UiSurfaceGestureInput::PointerUp {
                pointer_id,
                position,
            } => {
                let Some(active) = self
                    .active_pointer
                    .take()
                    .filter(|value| value.pointer_id == pointer_id)
                else {
                    return UiSurfaceGestureOutcome::unhandled(
                        self.hit_target(position)
                            .map(|surface| surface.target.clone()),
                    );
                };
                handled(
                    active.target,
                    UiSurfaceGestureInput::PointerUp {
                        pointer_id,
                        position,
                    },
                    None,
                )
            }
            UiSurfaceGestureInput::SmoothScroll {
                position,
                delta_x,
                delta_y,
            } => self
                .resolve_capability(position, |capabilities| capabilities.smooth_scroll_pan)
                .map_or_else(
                    || {
                        UiSurfaceGestureOutcome::unhandled(
                            self.hit_target(position)
                                .map(|surface| surface.target.clone()),
                        )
                    },
                    |target| {
                        handled(
                            target,
                            UiSurfaceGestureInput::SmoothScroll {
                                position,
                                delta_x,
                                delta_y,
                            },
                            Some(UiSurfaceGestureCommand::PanBy { delta_x, delta_y }),
                        )
                    },
                ),
            UiSurfaceGestureInput::Zoom { multiplier, center } => {
                let valid = multiplier.is_finite() && multiplier > 0.0;
                self.resolve_capability(center, |capabilities| capabilities.zoom)
                    .filter(|_| valid)
                    .map_or_else(
                        || {
                            UiSurfaceGestureOutcome::unhandled(
                                self.hit_target(center)
                                    .map(|surface| surface.target.clone()),
                            )
                        },
                        |target| {
                            handled(
                                target,
                                UiSurfaceGestureInput::Zoom { multiplier, center },
                                Some(UiSurfaceGestureCommand::ZoomBy { multiplier, center }),
                            )
                        },
                    )
            }
        }
    }

    pub fn apply_with_override(
        &mut self,
        input: UiSurfaceGestureInput,
        mut callback: impl FnMut(&UiSurfaceGestureEvent) -> UiSurfaceGestureOverride,
    ) -> UiSurfaceGestureOutcome {
        let mut outcome = self.apply(input);
        let Some(event) = outcome.event.as_ref() else {
            return outcome;
        };
        match callback(event) {
            UiSurfaceGestureOverride::UseDefault => {}
            UiSurfaceGestureOverride::Command(command) => outcome.command = Some(command),
            UiSurfaceGestureOverride::Ignore => {
                outcome.command = None;
                outcome.captured = false;
            }
        }
        outcome
    }

    pub fn set_fullscreen(
        &mut self,
        target: &UiStateId,
        fullscreen: bool,
    ) -> Option<UiSurfaceHostEvent> {
        let surface = self
            .surfaces
            .iter_mut()
            .find(|surface| &surface.target == target && surface.capabilities.fullscreen)?;
        surface.fullscreen = fullscreen;
        Some(UiSurfaceHostEvent::FullscreenChanged {
            target: target.clone(),
            fullscreen,
        })
    }

    fn resolve_capability(
        &self,
        point: UiSurfacePoint,
        accepts: impl FnOnce(UiSurfaceGestureCapabilities) -> bool,
    ) -> Option<UiStateId> {
        self.hit_target(point)
            .filter(|surface| accepts(surface.capabilities))
            .map(|surface| surface.target.clone())
    }
}

fn handled(
    target: UiStateId,
    input: UiSurfaceGestureInput,
    command: Option<UiSurfaceGestureCommand>,
) -> UiSurfaceGestureOutcome {
    UiSurfaceGestureOutcome {
        target: Some(target.clone()),
        event: Some(UiSurfaceGestureEvent { target, input }),
        command,
        captured: true,
    }
}

fn contains(bounds: UiRect, point: UiSurfacePoint) -> bool {
    let max_x = bounds.x.saturating_add_unsigned(bounds.width);
    let max_y = bounds.y.saturating_add_unsigned(bounds.height);
    point.x >= bounds.x && point.x < max_x && point.y >= bounds.y && point.y < max_y
}

#[cfg(test)]
mod tests {
    use super::*;

    fn surface(capabilities: UiSurfaceGestureCapabilities) -> UiGestureSurface {
        UiGestureSurface::new("surface", UiRect::new(10, 20, 100, 80)).capabilities(capabilities)
    }

    #[test]
    fn pointer_hover_and_drag_resolve_the_same_topmost_hit_surface() {
        let capabilities = UiSurfaceGestureCapabilities::default().pointer_pan(true);
        let mut controller = UiSurfaceGestureController::new([
            surface(capabilities),
            UiGestureSurface::new("top", UiRect::new(40, 40, 20, 20)).capabilities(capabilities),
        ]);
        let point = UiSurfacePoint::new(45, 45);
        assert_eq!(
            controller.hit_target(point).map(|hit| hit.target.as_str()),
            Some("top")
        );
        let down = controller.apply(UiSurfaceGestureInput::PointerDown {
            pointer_id: 7,
            position: point,
        });
        assert_eq!(down.target, Some(UiStateId::new("top")));
        let moved = controller.apply(UiSurfaceGestureInput::PointerMove {
            pointer_id: 7,
            position: UiSurfacePoint::new(50, 48),
        });
        assert_eq!(
            moved.command,
            Some(UiSurfaceGestureCommand::PanBy {
                delta_x: 5.0,
                delta_y: 3.0
            })
        );
        assert!(
            controller
                .apply(UiSurfaceGestureInput::PointerUp {
                    pointer_id: 7,
                    position: point
                })
                .captured
        );
    }

    #[test]
    fn undeclared_capabilities_leave_scroll_and_zoom_uncaptured() {
        let mut controller = UiSurfaceGestureController::new([surface(Default::default())]);
        let point = UiSurfacePoint::new(20, 30);
        for input in [
            UiSurfaceGestureInput::SmoothScroll {
                position: point,
                delta_x: 1.0,
                delta_y: 2.0,
            },
            UiSurfaceGestureInput::Zoom {
                multiplier: 1.2,
                center: point,
            },
        ] {
            let outcome = controller.apply(input);
            assert_eq!(outcome.target, Some(UiStateId::new("surface")));
            assert!(!outcome.captured);
            assert!(outcome.event.is_none());
        }
    }

    #[test]
    fn typed_override_and_fullscreen_host_event_are_closed_contracts() {
        let capabilities = UiSurfaceGestureCapabilities::default()
            .zoom(true)
            .fullscreen(true);
        let mut controller = UiSurfaceGestureController::new([surface(capabilities)]);
        let center = UiSurfacePoint::new(20, 30);
        let outcome = controller.apply_with_override(
            UiSurfaceGestureInput::Zoom {
                multiplier: 1.5,
                center,
            },
            |_| {
                UiSurfaceGestureOverride::Command(UiSurfaceGestureCommand::ZoomBy {
                    multiplier: 2.0,
                    center,
                })
            },
        );
        assert_eq!(
            outcome.command,
            Some(UiSurfaceGestureCommand::ZoomBy {
                multiplier: 2.0,
                center
            })
        );
        assert_eq!(
            controller.set_fullscreen(&UiStateId::new("surface"), true),
            Some(UiSurfaceHostEvent::FullscreenChanged {
                target: UiStateId::new("surface"),
                fullscreen: true
            })
        );
    }

    #[test]
    fn pointer_and_override_fail_closed_for_misses_mismatches_and_non_pan_surfaces() {
        let mut controller = UiSurfaceGestureController::new([
            surface(UiSurfaceGestureCapabilities::default()),
            UiGestureSurface::new("pan", UiRect::new(30, 30, 10, 10))
                .capabilities(UiSurfaceGestureCapabilities::default().pointer_pan(true)),
        ]);

        let miss = controller.apply(UiSurfaceGestureInput::PointerDown {
            pointer_id: 1,
            position: UiSurfacePoint::new(0, 0),
        });
        assert_eq!(miss.target, None);
        assert!(!miss.captured);

        let no_pan = controller.apply(UiSurfaceGestureInput::PointerDown {
            pointer_id: 2,
            position: UiSurfacePoint::new(20, 30),
        });
        assert_eq!(no_pan.target, Some(UiStateId::new("surface")));
        assert!(!no_pan.captured);

        let down = controller.apply(UiSurfaceGestureInput::PointerDown {
            pointer_id: 3,
            position: UiSurfacePoint::new(35, 35),
        });
        assert!(down.captured);
        let mismatch = controller.apply(UiSurfaceGestureInput::PointerMove {
            pointer_id: 4,
            position: UiSurfacePoint::new(36, 36),
        });
        assert!(!mismatch.captured);
        let ignored = controller.apply_with_override(
            UiSurfaceGestureInput::PointerMove {
                pointer_id: 3,
                position: UiSurfacePoint::new(38, 37),
            },
            |_| UiSurfaceGestureOverride::Ignore,
        );
        assert!(!ignored.captured);
        assert!(ignored.event.is_some());
        let stale_up = controller.apply(UiSurfaceGestureInput::PointerUp {
            pointer_id: 4,
            position: UiSurfacePoint::new(38, 37),
        });
        assert!(!stale_up.captured);
        let released = controller.apply(UiSurfaceGestureInput::PointerUp {
            pointer_id: 3,
            position: UiSurfacePoint::new(38, 37),
        });
        assert!(!released.captured);
    }

    #[test]
    fn zoom_rejects_non_positive_and_non_finite_multipliers_and_fullscreen_requires_capability() {
        let mut controller = UiSurfaceGestureController::new([surface(
            UiSurfaceGestureCapabilities::default().zoom(true),
        )]);
        let center = UiSurfacePoint::new(20, 30);
        for multiplier in [0.0, -1.0, f32::NAN, f32::INFINITY] {
            let outcome = controller.apply(UiSurfaceGestureInput::Zoom { multiplier, center });
            assert_eq!(outcome.target, Some(UiStateId::new("surface")));
            assert!(!outcome.captured);
        }
        assert_eq!(
            controller.set_fullscreen(&UiStateId::new("surface"), true),
            None
        );
    }
}

#[cfg(test)]
#[path = "gesture_coverage_tests.rs"]
mod coverage_tests;
