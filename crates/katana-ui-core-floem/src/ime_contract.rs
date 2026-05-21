use katana_ui_core::adapter_contract::{AdapterExtension, ImeRequest, ImeRequestPhase};
use katana_ui_core::render_model::UiNodeId;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FloemTextAreaImeStub;

impl FloemTextAreaImeStub {
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
    use super::FloemTextAreaImeStub;
    use katana_ui_core::adapter_contract::{AdapterExtension, ImeInputKind, ImeRequestPhase};
    use katana_ui_core::render_model::UiNodeId;

    #[test]
    fn exposes_multiline_ime_compile_gate_stub() -> Result<(), String> {
        let extension =
            FloemTextAreaImeStub.composition_update(UiNodeId::new("composer"), "かな", 6);

        let AdapterExtension::Ime(request) = extension else {
            return Err("expected IME request".to_string());
        };
        assert_eq!(ImeInputKind::Multiline, request.input_kind);
        assert_eq!(ImeRequestPhase::Update, request.phase);
        assert_eq!("かな", request.preedit);
        assert_eq!(6, request.caret);
        Ok(())
    }
}
