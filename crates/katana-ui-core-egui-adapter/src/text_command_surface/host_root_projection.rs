use super::super::root::KucRootEffectRouter;
use super::EguiTextCommandSurfacePresentationToken;
use katana_ui_core::molecule::command_chrome::CommandChromeFamilyId;
use serde::{Deserialize, Serialize};

pub struct EguiTextCommandSurfaceHostProjectionLease {
    pub(super) token: EguiTextCommandSurfacePresentationToken,
    pub(super) router: Box<dyn KucRootEffectRouter>,
}

impl EguiTextCommandSurfaceHostProjectionLease {
    #[must_use]
    pub fn new<R>(token: EguiTextCommandSurfacePresentationToken, router: R) -> Self
    where
        R: KucRootEffectRouter + 'static,
    {
        Self {
            token,
            router: Box::new(router),
        }
    }

    #[must_use]
    pub fn from_router(
        token: EguiTextCommandSurfacePresentationToken,
        router: Box<dyn KucRootEffectRouter>,
    ) -> Self {
        Self { token, router }
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        EguiTextCommandSurfacePresentationToken,
        Box<dyn KucRootEffectRouter>,
    ) {
        (self.token, self.router)
    }
}

impl std::fmt::Debug for EguiTextCommandSurfaceHostProjectionLease {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("EguiTextCommandSurfaceHostProjectionLease(..)")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EguiTextCommandSurfaceCommandFamilyProjection {
    pub primary: Option<CommandChromeFamilyId>,
    pub floating: Option<CommandChromeFamilyId>,
}

impl EguiTextCommandSurfaceCommandFamilyProjection {
    pub(crate) fn legacy_compatibility() -> Self {
        Self {
            primary: Some(CommandChromeFamilyId::new("kuc-family-0")),
            floating: Some(CommandChromeFamilyId::new("kuc-family-1")),
        }
    }

    #[must_use]
    pub const fn new(
        primary: Option<CommandChromeFamilyId>,
        floating: Option<CommandChromeFamilyId>,
    ) -> Self {
        Self { primary, floating }
    }
}

impl Default for EguiTextCommandSurfaceCommandFamilyProjection {
    fn default() -> Self {
        Self::new(None, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text_command_surface::root::{
        EguiTextCommandSurfaceRootEventChildClass, EguiTextCommandSurfaceRootEventClassDispatch,
    };
    use crate::text_command_surface::{
        EguiTextCommandSurfaceHostTargetToken, EguiTextCommandSurfacePresentationToken,
        KucOpaqueHostEffectBatch, KucOpaqueHostEffectError, KucRootEventBatchContext,
    };

    fn no_effect(
        _context: KucRootEventBatchContext,
    ) -> Result<Option<KucOpaqueHostEffectBatch>, KucOpaqueHostEffectError> {
        Ok(None)
    }

    fn empty_context() -> KucRootEventBatchContext {
        let dispatch = |child_class| EguiTextCommandSurfaceRootEventClassDispatch {
            child_class,
            event_count: 0,
        };
        KucRootEventBatchContext {
            root_identity: String::new(),
            state_revision: 0,
            correlation_fingerprint: String::new(),
            class_dispatches: [
                dispatch(EguiTextCommandSurfaceRootEventChildClass::Text),
                dispatch(EguiTextCommandSurfaceRootEventChildClass::Toolbar),
                dispatch(EguiTextCommandSurfaceRootEventChildClass::Floating),
                dispatch(EguiTextCommandSurfaceRootEventChildClass::Search),
                dispatch(EguiTextCommandSurfaceRootEventChildClass::ContextMenu),
            ],
            text_events: Vec::new(),
            toolbar_events: Vec::new(),
            floating_events: Vec::new(),
            search_events: Vec::new(),
            context_menu_events: Vec::new(),
        }
    }

    fn token() -> EguiTextCommandSurfacePresentationToken {
        EguiTextCommandSurfacePresentationToken::from_opaque_bytes(
            7,
            EguiTextCommandSurfaceHostTargetToken::from_opaque_bytes(b"target"),
            b"presentation",
        )
    }

    #[test]
    fn lease_constructors_preserve_opaque_parts_and_debug_contract() {
        let lease = EguiTextCommandSurfaceHostProjectionLease::new(token(), no_effect);
        assert_eq!(
            format!("{lease:?}"),
            "EguiTextCommandSurfaceHostProjectionLease(..)"
        );
        let (token, mut router) = lease.into_parts();
        assert_eq!(token.revision, 7);
        assert!(router.route(empty_context()).expect("router").is_none());

        let lease = EguiTextCommandSurfaceHostProjectionLease::from_router(token, router);
        let (token, _) = lease.into_parts();
        assert_eq!(token.revision, 7);
    }

    #[test]
    fn command_family_projection_defaults_and_legacy_values_are_distinct() {
        assert_eq!(
            EguiTextCommandSurfaceCommandFamilyProjection::default(),
            EguiTextCommandSurfaceCommandFamilyProjection::new(None, None)
        );
        let legacy = EguiTextCommandSurfaceCommandFamilyProjection::legacy_compatibility();
        assert_eq!(legacy.primary.unwrap().as_str(), "kuc-family-0");
        assert_eq!(legacy.floating.unwrap().as_str(), "kuc-family-1");
    }
}
