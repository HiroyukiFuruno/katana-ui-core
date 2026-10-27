pub(super) struct GlobalActionGuard {
    pub(super) generation: std::rc::Weak<std::cell::Cell<u64>>,
    pub(super) epoch: u64,
    pub(super) committed: std::rc::Rc<dyn Fn() -> bool>,
}
impl GlobalActionGuard {
    pub(super) fn is_valid(&self) -> bool {
        self.generation
            .upgrade()
            .is_some_and(|value| value.get() == self.epoch)
            && (self.committed)()
    }
}
