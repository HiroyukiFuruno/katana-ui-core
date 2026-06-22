//! Public atom widgets.

pub use crate::atom::{
    Badge, Button, Checkbox, Chip, ChipAction, ChipEvent, ChipKeyboardInput, ChipSize, ChipTone,
    ChipVariant, ColorSwatch, Divider, DragHandle, DropIndicator, Icon, IconTextButton,
    ImageSurface, Input, KeyCap, KeyCombo, KeyKind, KeyModifiers, LoadingDots, NamedKey,
    ProgressBar, Radio, RuntimePlatform, ShortcutCombo, ShortcutPlatform, ShortcutPlatformProvider,
    ShortcutSeparator, Skeleton, SkeletonAnimation, SkeletonShape, SkeletonSize, SlideControl,
    Spacer, Spinner, SvgButton, Text, TextArea, TextAreaAction, TextAreaActionOutcome,
    TextAreaCaretMove, TextAreaCompositionPhase, TextAreaCompositionState, TextAreaEvent,
    TextAreaKey, TextAreaKeyChord, TextAreaNewlineKey, TextAreaOptions, TextAreaResizeDelta,
    TextAreaResizeEvent, TextAreaSelection, TextAreaState, TextAreaSubmitKey, TextAreaTabBehavior,
    TextAreaValidationError, TextAreaWrapPolicy, TextButton, Toggle,
};

pub type TextInput = Input;
