use super::{StoryCatalog, StoryExample};
use katana_ui_core::atom;
use katana_ui_core::component::ComponentAction;
use katana_ui_core::interaction::{RgbaActionValue, UiAction, UiCallbackLog};
use katana_ui_core::render_model::{UiAnimationState, UiDimension, UiTone, UiVisualRole};
use katana_ui_core::{atom::SkeletonAnimation, layout};

const PROGRESS_PERCENT: u8 = 64;
const SLIDE_VALUE: &str = "42";
const COLOR_SWATCH_RED: u8 = 64;
const COLOR_SWATCH_GREEN: u8 = 128;
const COLOR_SWATCH_BLUE: u8 = 255;
const COLOR_SWATCH_ALPHA: u8 = 255;
const COLOR_SWATCH_HUE: u16 = 210;
const LOADING_PHASE: u16 = 2;
const SPINNER_PHASE: u16 = 3;
const SKELETON_WIDTH_PX: u16 = 220;
const SKELETON_HEIGHT_PX: u16 = 44;
const REDUCED_MOTION_LINE_COUNT: usize = 2;
const REDUCED_MOTION_LAST_LINE_RATIO: f32 = 0.62;
const REDUCED_MOTION_RADIUS_PX: u16 = 6;
const TEXT_LINE_COUNT: usize = 3;
const TEXT_LAST_LINE_RATIO: f32 = 0.58;
const COMPACT_RADIUS_PX: u16 = 4;
const AVATAR_SIZE_PX: u16 = 44;
const AVATAR_RADIUS_PX: u16 = 22;
const RECT_RADIUS_PX: u16 = 8;
const LINE_THICKNESS_PX: f32 = 8.0;
const WARNING_RADIUS_PX: u16 = 14;

pub(super) fn skeleton() -> StoryExample {
    let mut reduced_motion = skeleton_preset(
        "reduced motion shape=Text size=220x44 animation=Shimmer tone=Neutral radius=6 reduced_motion=true accessibility_label=Reduced loading text",
        atom::SkeletonShape::Text {
            lines: REDUCED_MOTION_LINE_COUNT,
            last_line_ratio: REDUCED_MOTION_LAST_LINE_RATIO,
        },
        SKELETON_WIDTH_PX,
        SKELETON_HEIGHT_PX,
        SkeletonAnimation::Shimmer,
        UiTone::Neutral,
        REDUCED_MOTION_RADIUS_PX,
    );
    let reduced_motion_target = reduced_motion.state_id().clone();
    let reduced_motion_result = reduced_motion.apply_action(&UiAction::reduced_motion(
        reduced_motion_target.clone(),
        true,
    ));
    let shape_target = reduced_motion_target;
    let mut logs = reduced_motion_result.callback_log;
    logs.push(UiCallbackLog::new(
        shape_target,
        "skeleton_shape_changed",
        "shape=Rect size=160x80 animation=Pulse tone=Neutral radius=4 reduced_motion=false accessibility_label=Loading block",
        "shape=Text lines=2 last_line_ratio=0.62 size=220x44 animation=None tone=Neutral radius=6 reduced_motion=true accessibility_label=Reduced loading text",
    ));
    logs.push(UiCallbackLog::new(
        reduced_motion.state_id().clone(),
        "skeleton_animation_changed",
        "animation=Shimmer reduced_motion=false",
        "animation=None reduced_motion=true event=skeleton_animation_changed",
    ));
    StoryCatalog::interactive_story(
        "skeleton",
        layout::Column::new()
            .child(skeleton_preset(
                "text lines shape=Text size=220x44 animation=Shimmer tone=Neutral radius=4 reduced_motion=false accessibility_label=Loading text lines",
                atom::SkeletonShape::Text {
                    lines: TEXT_LINE_COUNT,
                    last_line_ratio: TEXT_LAST_LINE_RATIO,
                },
                SKELETON_WIDTH_PX,
                SKELETON_HEIGHT_PX,
                SkeletonAnimation::Shimmer,
                UiTone::Neutral,
                COMPACT_RADIUS_PX,
            ))
            .child(skeleton_preset(
                "avatar circle shape=Circle size=44x44 animation=Pulse tone=Accent radius=22 reduced_motion=false accessibility_label=Loading avatar",
                atom::SkeletonShape::Circle,
                AVATAR_SIZE_PX,
                AVATAR_SIZE_PX,
                SkeletonAnimation::Pulse,
                UiTone::Accent,
                AVATAR_RADIUS_PX,
            ))
            .child(skeleton_preset(
                "rect shimmer shape=Rect size=220x44 animation=Shimmer tone=Neutral radius=8 reduced_motion=false accessibility_label=Loading rectangle",
                atom::SkeletonShape::Rect,
                SKELETON_WIDTH_PX,
                SKELETON_HEIGHT_PX,
                SkeletonAnimation::Shimmer,
                UiTone::Neutral,
                RECT_RADIUS_PX,
            ))
            .child(skeleton_preset(
                "line wave shape=Line thickness=8 size=220x44 animation=Wave tone=Success radius=4 reduced_motion=false accessibility_label=Loading line",
                atom::SkeletonShape::Line {
                    thickness: LINE_THICKNESS_PX,
                },
                SKELETON_WIDTH_PX,
                SKELETON_HEIGHT_PX,
                SkeletonAnimation::Wave,
                UiTone::Success,
                COMPACT_RADIUS_PX,
            ))
            .child(reduced_motion)
            .child(skeleton_preset(
                "tone/radius shape=Rect size=220x44 animation=Pulse tone=Warning radius=14 reduced_motion=false accessibility_label=Loading warning block",
                atom::SkeletonShape::Rect,
                SKELETON_WIDTH_PX,
                SKELETON_HEIGHT_PX,
                SkeletonAnimation::Pulse,
                UiTone::Warning,
                WARNING_RADIUS_PX,
            )),
        logs,
    )
}

fn skeleton_preset(
    label: &'static str,
    shape: atom::SkeletonShape,
    width_px: u16,
    height_px: u16,
    animation: SkeletonAnimation,
    tone: UiTone,
    radius_px: u16,
) -> atom::Skeleton {
    atom::Skeleton::new(label, shape)
        .size(atom::SkeletonSize::Fixed {
            width: UiDimension::Px(width_px),
            height: UiDimension::Px(height_px),
        })
        .animation(animation)
        .tone(tone)
        .radius_px(radius_px)
        .accessibility_label(label)
}

pub(super) fn loading_dots() -> StoryExample {
    let mut loading = atom::LoadingDots::new("Loading dots")
        .visual_role(UiVisualRole::Loading)
        .loading(true)
        .animation_state(UiAnimationState::Running);
    let target = loading.state_id().clone();
    let result = loading.apply_action(&UiAction::animation_tick(target, LOADING_PHASE));
    StoryCatalog::interactive_story("loading-dots", loading, result.callback_log)
}

pub(super) fn spinner() -> StoryExample {
    let mut spinner = atom::Spinner::new("Spinner")
        .accessibility_label("Loading")
        .visual_role(UiVisualRole::Loading)
        .loading(true);
    let target = spinner.state_id().clone();
    let result = spinner.apply_action(&UiAction::animation_tick(target, SPINNER_PHASE));
    StoryCatalog::interactive_story("spinner", spinner, result.callback_log)
}

pub(super) fn progress_bar() -> StoryExample {
    let mut progress = atom::ProgressBar::new("Progress bar").visual_role(UiVisualRole::Progress);
    let target = progress.state_id().clone();
    let result = progress.apply_action(&UiAction::progress_changed(target, true, PROGRESS_PERCENT));
    StoryCatalog::interactive_story("progress-bar", progress, result.callback_log)
}

pub(super) fn color_swatch() -> StoryExample {
    let mut color = atom::ColorSwatch::new("Color swatch").visual_role(UiVisualRole::Control);
    let target = color.state_id().clone();
    let result = color.apply_action(&UiAction::color_drag(
        target,
        RgbaActionValue::new(
            COLOR_SWATCH_RED,
            COLOR_SWATCH_GREEN,
            COLOR_SWATCH_BLUE,
            COLOR_SWATCH_ALPHA,
        ),
        COLOR_SWATCH_HUE,
        false,
    ));
    StoryCatalog::interactive_story("color-swatch", color, result.callback_log)
}

pub(super) fn toggle() -> StoryExample {
    let mut toggle = atom::Toggle::new("Toggle")
        .visual_role(UiVisualRole::Control)
        .selected(false);
    let target = toggle.state_id().clone();
    let result = toggle.apply_action(&UiAction::toggle_checked(target, true));
    StoryCatalog::interactive_story("toggle", toggle, result.callback_log)
}

pub(super) fn slide_control() -> StoryExample {
    let mut slide = atom::SlideControl::new("Slide control").visual_role(UiVisualRole::Control);
    let target = slide.state_id().clone();
    let result = slide.apply_action(&UiAction::slide_changed(target, SLIDE_VALUE));
    StoryCatalog::interactive_story("slide-control", slide, result.callback_log)
}
