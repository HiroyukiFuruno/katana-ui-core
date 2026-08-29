use super::focus_request::TextSurfaceFocusRequestAcknowledgement;
use super::surface_model::TextSurface;

impl TextSurface {
    /// Consumes a new request after the adapter has allocated its native focus response.
    ///
    /// This intentionally does not alter KUC focus state. The adapter issues the native request,
    /// then a later synchronization reports the actual focus fact through `FocusChanged`.
    pub fn issue_controlled_focus_request(
        &mut self,
    ) -> Option<TextSurfaceFocusRequestAcknowledgement> {
        let request = self.props.focus_request.clone()?;
        if self.state.last_focus_request_token.as_ref() == Some(&request.token) {
            return None;
        }
        self.state.last_focus_request_token = Some(request.token.clone());
        Some(TextSurfaceFocusRequestAcknowledgement {
            token: request.token,
            focused: request.focused,
        })
    }
}
