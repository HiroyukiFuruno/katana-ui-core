use super::{
    ALIGN_CENTER_PAGE, COLUMN_PAGE, GRID_PAGE, LayoutStoryAction, LayoutStoryState,
    LayoutStoryUpdate, ROW_PAGE, SELECTED_CELL_INDEX, STACK_PAGE, STACK_TOP_INDEX,
    align_center_contract, is_live_layout_page, layout_alignment_for, layout_page,
};

impl LayoutStoryState {
    pub(in crate::visual) fn apply_action(
        &mut self,
        action: LayoutStoryAction,
    ) -> LayoutStoryUpdate {
        match action {
            LayoutStoryAction::RowAlign => self.apply_layout(ROW_PAGE, "row_align"),
            LayoutStoryAction::RowHover => {
                self.apply_row_operation("row_hover", "hover_start", "hover=row")
            }
            LayoutStoryAction::RowFocus => self.apply_row_focus(),
            LayoutStoryAction::RowKeyboard => self.apply_row_operation(
                "row_keyboard_align",
                "layout_changed",
                "keyboard=align-center",
            ),
            LayoutStoryAction::RowResize => self.apply_row_resize(),
            LayoutStoryAction::ColumnAlign => self.apply_layout(COLUMN_PAGE, "column_align"),
            LayoutStoryAction::ColumnHover => {
                self.apply_column_operation("column_hover", "hover_start", "hover=column")
            }
            LayoutStoryAction::ColumnFocus => self.apply_column_focus(),
            LayoutStoryAction::ColumnKeyboard => self.apply_column_operation(
                "column_keyboard_align",
                "layout_changed",
                "keyboard=align-center",
            ),
            LayoutStoryAction::ColumnResize => self.apply_column_resize(),
            LayoutStoryAction::StackReorder => self.apply_stack(),
            LayoutStoryAction::StackHover => {
                self.apply_stack_operation("stack_hover", "hover_start", "hover=stack")
            }
            LayoutStoryAction::StackFocus => self.apply_stack_focus(),
            LayoutStoryAction::StackKeyboard => self.apply_stack_operation(
                "stack_keyboard_reorder",
                "z_order_changed",
                "keyboard=z-order",
            ),
            LayoutStoryAction::StackResize => self.apply_stack_resize(),
            LayoutStoryAction::GridSelect => self.apply_grid(),
            LayoutStoryAction::GridHover => {
                self.apply_grid_operation("grid_hover", "hover_start", "hover=grid")
            }
            LayoutStoryAction::GridFocus => self.apply_grid_focus(),
            LayoutStoryAction::GridKeyboard => self.apply_grid_operation(
                "grid_keyboard_select",
                "grid_cell_selected",
                "keyboard=cell",
            ),
            LayoutStoryAction::GridResize => self.apply_grid_resize(),
            LayoutStoryAction::AlignCenterHover => self.apply_align_center_operation(
                "align_center_hover",
                "hover_start",
                "hover=center",
            ),
            LayoutStoryAction::AlignCenterFocus => self.apply_align_center_focus(),
            LayoutStoryAction::AlignCenterKeyboard => self.apply_align_center_operation(
                "align_center_keyboard_measure",
                "alignment_changed",
                "keyboard=center",
            ),
            LayoutStoryAction::AlignCenterResize => self.apply_align_center_resize(),
        }
    }

    pub(in crate::visual) fn apply_option(
        &mut self,
        page: &str,
        setting: &str,
    ) -> Option<LayoutStoryUpdate> {
        if !is_live_layout_page(page) {
            return None;
        }
        let state = match (page, setting) {
            (ROW_PAGE, "axis") => "row.axis=y",
            (ROW_PAGE, "gap") => "row.gap=large",
            (ROW_PAGE, "alignment") => "row.alignment=center",
            (ROW_PAGE, "overflow") => "row.overflow=scroll",
            (COLUMN_PAGE, "axis") => "column.axis=y",
            (COLUMN_PAGE, "gap") => "column.gap=large",
            (COLUMN_PAGE, "alignment") => "column.alignment=center",
            (COLUMN_PAGE, "overflow") => "column.overflow=scroll",
            (STACK_PAGE, "axis") => "stack.axis=y",
            (STACK_PAGE, "gap") => "stack.gap=large",
            (STACK_PAGE, "alignment") => "stack.alignment=center",
            (STACK_PAGE, "overflow") => "stack.overflow=scroll",
            (GRID_PAGE, "axis") => "grid.axis=y",
            (GRID_PAGE, "gap") => "grid.gap=large",
            (GRID_PAGE, "alignment") => "grid.alignment=center",
            (GRID_PAGE, "overflow") => "grid.overflow=scroll",
            (ALIGN_CENTER_PAGE, "axis") => "align_center.axis=y",
            (ALIGN_CENTER_PAGE, "gap") => "align_center.gap=large",
            (ALIGN_CENTER_PAGE, "alignment") => "align_center.alignment=center",
            (ALIGN_CENTER_PAGE, "overflow") => "align_center.overflow=scroll",
            _ => return None,
        };
        Some(self.apply_layout_option(page, state))
    }

    fn apply_layout(&mut self, page: &'static str, action: &'static str) -> LayoutStoryUpdate {
        let alignment = layout_alignment_for(page);
        self.page = page;
        self.alignment = alignment;
        self.selected_index = 0;
        self.callback = "callback=layout";
        LayoutStoryUpdate::new(action, "layout_changed", alignment)
    }

    fn apply_row_operation(
        &mut self,
        action: &'static str,
        event: &'static str,
        state: &'static str,
    ) -> LayoutStoryUpdate {
        self.page = ROW_PAGE;
        self.alignment = "alignment=center";
        self.callback = "callback=row";
        if action == "row_hover" {
            self.hovered = true;
        }
        LayoutStoryUpdate::new(action, event, state)
    }

    fn apply_row_focus(&mut self) -> LayoutStoryUpdate {
        self.page = ROW_PAGE;
        self.alignment = "alignment=center";
        self.callback = "callback=row-focus";
        self.focused = true;
        LayoutStoryUpdate::new("row_focus", "focus", "focus=row")
    }

    fn apply_row_resize(&mut self) -> LayoutStoryUpdate {
        self.page = ROW_PAGE;
        self.alignment = "overflow=scroll";
        self.callback = "callback=row-resize";
        self.resized = true;
        LayoutStoryUpdate::new("row_resize", "layout_resized", "resize=row")
    }

    fn apply_column_operation(
        &mut self,
        action: &'static str,
        event: &'static str,
        state: &'static str,
    ) -> LayoutStoryUpdate {
        self.page = COLUMN_PAGE;
        self.alignment = "alignment=center";
        self.callback = "callback=column";
        if action == "column_hover" {
            self.hovered = true;
        }
        LayoutStoryUpdate::new(action, event, state)
    }

    fn apply_column_focus(&mut self) -> LayoutStoryUpdate {
        self.page = COLUMN_PAGE;
        self.alignment = "alignment=center";
        self.callback = "callback=column-focus";
        self.focused = true;
        LayoutStoryUpdate::new("column_focus", "focus", "focus=column")
    }

    fn apply_column_resize(&mut self) -> LayoutStoryUpdate {
        self.page = COLUMN_PAGE;
        self.alignment = "overflow=scroll";
        self.callback = "callback=column-resize";
        self.resized = true;
        LayoutStoryUpdate::new("column_resize", "layout_resized", "resize=column")
    }

    fn apply_stack(&mut self) -> LayoutStoryUpdate {
        self.page = STACK_PAGE;
        self.selected_index = STACK_TOP_INDEX;
        self.callback = "callback=stack";
        LayoutStoryUpdate::new("stack_reorder", "z_order_changed", "z_order=2")
    }

    fn apply_stack_operation(
        &mut self,
        action: &'static str,
        event: &'static str,
        state: &'static str,
    ) -> LayoutStoryUpdate {
        self.page = STACK_PAGE;
        self.selected_index = STACK_TOP_INDEX;
        self.callback = "callback=stack";
        if action == "stack_hover" {
            self.hovered = true;
        }
        LayoutStoryUpdate::new(action, event, state)
    }

    fn apply_stack_focus(&mut self) -> LayoutStoryUpdate {
        self.page = STACK_PAGE;
        self.selected_index = STACK_TOP_INDEX;
        self.callback = "callback=stack-focus";
        self.focused = true;
        LayoutStoryUpdate::new("stack_focus", "focus", "focus=stack")
    }

    fn apply_stack_resize(&mut self) -> LayoutStoryUpdate {
        self.page = STACK_PAGE;
        self.selected_index = STACK_TOP_INDEX;
        self.callback = "callback=stack-resize";
        self.resized = true;
        LayoutStoryUpdate::new("stack_resize", "layout_resized", "resize=stack")
    }

    fn apply_grid(&mut self) -> LayoutStoryUpdate {
        self.page = GRID_PAGE;
        self.selected_index = SELECTED_CELL_INDEX;
        self.callback = "callback=grid";
        LayoutStoryUpdate::new("grid_select", "grid_cell_selected", "selected=1")
    }

    fn apply_grid_operation(
        &mut self,
        action: &'static str,
        event: &'static str,
        state: &'static str,
    ) -> LayoutStoryUpdate {
        self.page = GRID_PAGE;
        self.selected_index = SELECTED_CELL_INDEX;
        self.callback = "callback=grid";
        if action == "grid_hover" {
            self.hovered = true;
        }
        LayoutStoryUpdate::new(action, event, state)
    }

    fn apply_grid_focus(&mut self) -> LayoutStoryUpdate {
        self.page = GRID_PAGE;
        self.selected_index = SELECTED_CELL_INDEX;
        self.callback = "callback=grid-focus";
        self.focused = true;
        LayoutStoryUpdate::new("grid_focus", "focus", "focus=grid")
    }

    fn apply_grid_resize(&mut self) -> LayoutStoryUpdate {
        self.page = GRID_PAGE;
        self.selected_index = SELECTED_CELL_INDEX;
        self.callback = "callback=grid-resize";
        self.resized = true;
        LayoutStoryUpdate::new("grid_resize", "layout_resized", "resize=grid")
    }

    fn apply_align_center_operation(
        &mut self,
        action: &'static str,
        event: &'static str,
        state: &'static str,
    ) -> LayoutStoryUpdate {
        self.page = ALIGN_CENTER_PAGE;
        self.alignment = align_center_contract();
        self.callback = "callback=align-center";
        if action == "align_center_hover" {
            self.hovered = true;
        }
        LayoutStoryUpdate::new(action, event, state)
    }

    fn apply_align_center_focus(&mut self) -> LayoutStoryUpdate {
        self.page = ALIGN_CENTER_PAGE;
        self.alignment = align_center_contract();
        self.callback = "callback=align-center-focus";
        self.focused = true;
        LayoutStoryUpdate::new("align_center_focus", "focus", "focus=center")
    }

    fn apply_align_center_resize(&mut self) -> LayoutStoryUpdate {
        self.page = ALIGN_CENTER_PAGE;
        self.alignment = align_center_contract();
        self.callback = "callback=align-center-resize";
        self.resized = true;
        LayoutStoryUpdate::new("align_center_resize", "layout_resized", "resize=center")
    }

    fn apply_layout_option(&mut self, page: &str, state: &'static str) -> LayoutStoryUpdate {
        self.page = layout_page(page);
        self.alignment = state;
        self.callback = "callback=layout-option";
        LayoutStoryUpdate::new("layout_option_changed", "layout_option_changed", state)
    }
}
