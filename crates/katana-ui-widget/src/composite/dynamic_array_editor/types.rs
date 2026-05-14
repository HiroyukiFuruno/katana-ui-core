use std::rc::Rc;

use floem::View;

pub type DynamicArrayItemRenderer<T> = dyn Fn(&DynamicArrayEditorItem<T>, usize) -> Box<dyn View>;

#[derive(Debug, Clone)]
pub struct DynamicArrayEditorItem<T> {
    pub value: T,
    pub deletable: bool,
    pub reorderable: bool,
}

impl<T> DynamicArrayEditorItem<T> {
    #[must_use]
    pub fn new(value: T) -> Self {
        Self {
            value,
            deletable: true,
            reorderable: true,
        }
    }

    #[must_use]
    pub fn deletable(mut self, deletable: bool) -> Self {
        self.deletable = deletable;
        self
    }

    #[must_use]
    pub fn reorderable(mut self, reorderable: bool) -> Self {
        self.reorderable = reorderable;
        self
    }
}

pub(crate) struct DynamicArrayEditorProps<T> {
    pub items: Vec<DynamicArrayEditorItem<T>>,
    pub max_items: Option<usize>,
    pub disabled: bool,
    pub empty_state: String,
    pub item_renderer: Rc<DynamicArrayItemRenderer<T>>,
    pub create_item: Rc<dyn Fn() -> DynamicArrayEditorItem<T>>,
    pub on_change: Rc<dyn Fn(Vec<DynamicArrayEditorItem<T>>)>,
    pub on_add: Rc<dyn Fn(usize)>,
    pub on_edit: Rc<dyn Fn(usize)>,
    pub on_delete: Rc<dyn Fn(usize)>,
    pub on_move: Rc<dyn Fn(usize, usize)>,
}
