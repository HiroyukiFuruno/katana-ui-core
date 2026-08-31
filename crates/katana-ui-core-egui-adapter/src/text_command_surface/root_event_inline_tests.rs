use super::dispatcher::{RootEventEnvelope, RootEventFingerprint};
use super::*;
use crate::text_command_surface::source_address_projection_lease::SourceAddressSubmissionPort;
use std::rc::Rc;

const ROOT_EVENT_FRAME_WIDTH: f32 = 800.0;
const ROOT_EVENT_FRAME_HEIGHT: f32 = 600.0;
const ROOT_EVENT_INPUT_X: f32 = 100.0;
const ROOT_EVENT_INPUT_Y: f32 = 580.0;

mod router_context {
    use super::*;
    include!("root_event_inline_tests/router_context.rs");
}

mod detach {
    use super::dispatch_setup::source_submission;
    use super::router_context::CountingForwarder;
    use super::*;
    include!("root_event_inline_tests/detach.rs");
}

mod dispatch_setup {
    use super::*;
    include!("root_event_inline_tests/dispatch_setup.rs");
}

mod source_address {
    use super::dispatch_setup::{
        OrderRecorder, RecordingSourcePort, full_payload, source_submission,
    };
    use super::*;
    include!("root_event_inline_tests/source_address.rs");
}

mod transport_dispatch {
    use super::dispatch_setup::{OrderRecorder, full_payload};
    use super::*;
    include!("root_event_inline_tests/transport_dispatch.rs");
}

mod effect_dispatch {
    use super::dispatch_setup::{OrderRecorder, full_payload};
    use super::transport_dispatch::{DispatcherError, StageFailingDispatcher};
    use super::*;
    include!("root_event_inline_tests/effect_dispatch.rs");
}

mod retained_lifecycle {
    use super::*;
    include!("root_event_inline_tests/retained_lifecycle.rs");
}

mod detach_lifecycle {
    use super::dispatch_setup::full_payload;
    use super::*;
    include!("root_event_inline_tests/detach_lifecycle.rs");
}
