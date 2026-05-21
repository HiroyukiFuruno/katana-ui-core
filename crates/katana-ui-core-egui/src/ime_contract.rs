use katana_ui_core::adapter_contract::{AdapterExtension, ImeRequest, ImeRequestPhase};
use katana_ui_core::render_model::UiNodeId;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct EguiTextAreaImeStub;

impl EguiTextAreaImeStub {
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
    use super::EguiTextAreaImeStub;
    use katana_ui_core::adapter_contract::{AdapterExtension, ImeInputKind, ImeRequestPhase};
    use katana_ui_core::render_model::UiNodeId;

    #[test]
    fn exposes_multiline_ime_compile_gate_stub() -> Result<(), String> {
        let extension =
            EguiTextAreaImeStub.composition_update(UiNodeId::new("composer"), "日本\n語", 10);

        let AdapterExtension::Ime(request) = extension else {
            return Err("expected IME request".to_string());
        };
        assert_eq!(ImeInputKind::Multiline, request.input_kind);
        assert_eq!(ImeRequestPhase::Update, request.phase);
        assert_eq!("日本\n語", request.preedit);
        assert_eq!(10, request.caret);
        Ok(())
    }
}
