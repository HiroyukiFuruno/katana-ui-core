use katana_ui_core::adapter_contract::{AdapterExtension, ImeRequest, ImeRequestPhase};
use katana_ui_core::render_model::UiNodeId;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct GpuiTextAreaImeStub;

impl GpuiTextAreaImeStub {
    #[must_use]
    pub fn composition_update(
        self,
        target: UiNodeId,
        preedit: impl Into<String>,
        caret: usize,
    ) -> AdapterExtension {
        AdapterExtension::Ime(ImeRequest::multiline(
            target,
            ImeRequestPhase::Update,
            preedit,
            caret,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::GpuiTextAreaImeStub;
    use katana_ui_core::adapter_contract::{AdapterExtension, ImeInputKind, ImeRequestPhase};
    use katana_ui_core::render_model::UiNodeId;

    #[test]
    fn exposes_multiline_ime_compile_gate_stub() {
        let extension = GpuiTextAreaImeStub.composition_update(UiNodeId::new("composer"), "👩‍💻", 11);

        let AdapterExtension::Ime(request) = extension else {
            panic!("expected IME request");
        };
        assert_eq!(ImeInputKind::Multiline, request.input_kind);
        assert_eq!(ImeRequestPhase::Update, request.phase);
        assert_eq!("👩‍💻", request.preedit);
        assert_eq!(11, request.caret);
    }
}
