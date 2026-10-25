use super::{EguiTextCommandSurfaceChild, EguiTextCommandSurfaceOutput, UiRect};

impl EguiTextCommandSurfaceOutput {
    pub(crate) fn with_selection_dialog(
        mut self,
        dialog: Option<super::super::selection_dialog::SelectionDialogOutput>,
    ) -> Self {
        if let Some(dialog) = dialog {
            self.root_bounds =
                union_root_bounds(self.root_bounds, dialog.paint_plan.surface_bounds);
            self.selection_dialog = Some(dialog);
            self.artifact_order
                .push(EguiTextCommandSurfaceChild::SelectionDialog);
        }
        self
    }
}

fn union_root_bounds(a: UiRect, b: UiRect) -> UiRect {
    let x = a.x.min(b.x);
    let y = a.y.min(b.y);
    let right =
        a.x.saturating_add_unsigned(a.width)
            .max(b.x.saturating_add_unsigned(b.width));
    let bottom =
        a.y.saturating_add_unsigned(a.height)
            .max(b.y.saturating_add_unsigned(b.height));
    UiRect::new(
        x,
        y,
        right.saturating_sub(x) as u32,
        bottom.saturating_sub(y) as u32,
    )
}
