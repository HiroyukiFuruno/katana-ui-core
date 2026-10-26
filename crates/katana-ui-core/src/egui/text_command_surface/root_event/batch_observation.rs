use super::EguiTextCommandSurfaceRootEventBatch;
use crate::molecule::command_chrome::{CommandChromeToolbarEvent, FloatingCommandToolbarEvent};
use crate::molecule::selection::ContextMenuEvent;

impl EguiTextCommandSurfaceRootEventBatch {
    pub(crate) fn contains_command_activation(
        &self,
        action_identity: &str,
        floating: bool,
    ) -> bool {
        let transport = self.transport.borrow();
        let Some(transport) = transport.as_ref() else {
            return false;
        };
        if floating {
            transport.payload.floating.as_ref().is_some_and(|events| {
                events.iter().any(|event| {
                    matches!(event, FloatingCommandToolbarEvent::Toolbar {
                        event: CommandChromeToolbarEvent::CommandActivated { action_id },
                    } if action_id.as_str() == action_identity)
                })
            })
        } else {
            transport.payload.toolbar.as_ref().is_some_and(|events| {
                events.iter().any(|event| {
                    matches!(event, CommandChromeToolbarEvent::CommandActivated { action_id }
                        if action_id.as_str() == action_identity)
                })
            })
        }
    }

    pub(crate) fn contains_context_menu_opened(&self) -> bool {
        let transport = self.transport.borrow();
        transport.as_ref().is_some_and(|transport| {
            transport
                .payload
                .context_menu
                .as_ref()
                .is_some_and(|events| {
                    events
                        .iter()
                        .any(|event| matches!(event, ContextMenuEvent::Opened { .. }))
                })
        })
    }
}
