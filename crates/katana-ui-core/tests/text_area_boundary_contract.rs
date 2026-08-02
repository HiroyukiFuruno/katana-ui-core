use katana_ui_core::adapter_contract::{
    AdapterExtension, ImeInputKind, ImeRequest, ImeRequestPhase,
};
use katana_ui_core::atom::{Input, InputValidationError, TextArea};
use katana_ui_core::render_model::{UiNode, UiNodeId, UiNodeKind};

#[test]
fn input_rejects_multiline_value_and_directs_consumers_to_text_area() {
    let input = Input::new("Single line").value("one\ntwo");
    let text_area = UiNode::from(TextArea::new("Multi line").value("one\ntwo"));

    assert_eq!(
        Err(InputValidationError::MultilineValueRequiresTextArea),
        input.validate()
    );
    assert_eq!(UiNodeKind::TextArea, text_area.kind());
    assert_eq!("one\ntwo", text_area.props().interaction.value);
}

#[test]
fn input_accepts_single_line_value() {
    assert_eq!(
        Ok(()),
        Input::new("Single line").value("one line").validate()
    );
}

#[test]
fn adapter_ime_request_carries_multiline_preedit_and_caret() {
    let target = UiNodeId::new("composer");
    let request = ImeRequest::multiline(
        target.clone(),
        ImeRequestPhase::Update,
        "日本\n語",
        "日本\n語".len(),
    );
    let extension = AdapterExtension::Ime(request.clone());

    assert_eq!(target, request.target);
    assert_eq!(ImeInputKind::Multiline, request.input_kind);
    assert_eq!(ImeRequestPhase::Update, request.phase);
    assert_eq!("日本\n語", request.preedit);
    assert_eq!("日本\n語".len(), request.caret);
    assert!(matches!(extension, AdapterExtension::Ime(it) if it == request));
}
