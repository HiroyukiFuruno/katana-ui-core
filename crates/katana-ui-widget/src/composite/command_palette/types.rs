use std::rc::Rc;

use crate::primitive::icon::IconSource;

/// 検索で表示される 1 行分の情報。
const DEFAULT_SCORE: i32 = 0;

#[derive(Debug, Clone)]
pub struct CommandPaletteItem<P: Clone + 'static> {
    pub label: String,
    pub icon: Option<IconSource>,
    pub shortcut: Option<String>,
    pub score: i32,
    pub payload: P,
}

impl<P: Clone + 'static> CommandPaletteItem<P> {
    #[must_use]
    pub fn new(label: impl Into<String>, payload: P) -> Self {
        Self {
            label: label.into(),
            icon: None,
            shortcut: None,
            score: DEFAULT_SCORE,
            payload,
        }
    }

    #[must_use]
    pub fn icon(mut self, icon: IconSource) -> Self {
        self.icon = Some(icon);
        self
    }

    #[must_use]
    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    #[must_use]
    pub fn score(mut self, score: i32) -> Self {
        self.score = score;
        self
    }
}

/// 検索クエリを受けて候補を返す provider。
pub trait CommandPaletteProvider<P: Clone + 'static>: 'static {
    fn query(&self, query: &str) -> Vec<CommandPaletteItem<P>>;
}

/// コールバックで provider を作るためのアダプタ。
pub struct CallbackCommandPaletteProvider<P: Clone + 'static, F>
where
    F: Fn(&str) -> Vec<CommandPaletteItem<P>> + 'static,
{
    query_fn: F,
}

impl<P: Clone + 'static, F> CallbackCommandPaletteProvider<P, F>
where
    F: Fn(&str) -> Vec<CommandPaletteItem<P>> + 'static,
{
    #[must_use]
    pub fn new(query_fn: F) -> Self {
        Self { query_fn }
    }
}

impl<P: Clone + 'static, F> CommandPaletteProvider<P> for CallbackCommandPaletteProvider<P, F>
where
    F: Fn(&str) -> Vec<CommandPaletteItem<P>> + 'static,
{
    fn query(&self, query: &str) -> Vec<CommandPaletteItem<P>> {
        (self.query_fn)(query)
    }
}

pub type ExecuteCallback<P> = dyn Fn(String, usize, P);
pub type SelectionCallback = dyn Fn(String, usize);
pub type QueryCallback = dyn Fn(String);
pub type CloseCallback = dyn Fn();

pub(crate) type OnExecute<P> = Rc<ExecuteCallback<P>>;
pub(crate) type OnSelection = Rc<SelectionCallback>;
pub(crate) type OnQuery = Rc<QueryCallback>;
pub(crate) type OnClose = Rc<CloseCallback>;

pub(crate) struct CommandPaletteDefaults;

impl CommandPaletteDefaults {
    pub(crate) fn noop_execute<P: Clone + 'static>(_query: String, _index: usize, _payload: P) {}

    pub(crate) fn noop_selection(_query: String, _index: usize) {}

    pub(crate) fn noop_query(_query: String) {}

    pub(crate) fn noop_close() {}
}

#[derive(Clone)]
pub(crate) struct CommandPaletteProps<P: Clone + 'static> {
    pub provider: Rc<dyn CommandPaletteProvider<P>>,
    pub on_execute: OnExecute<P>,
    pub on_selection_change: OnSelection,
    pub on_query: OnQuery,
    pub on_close: OnClose,
    pub placeholder: String,
    pub disabled: bool,
}

pub struct CommandPalette<P: Clone + 'static> {
    pub(crate) props: CommandPaletteProps<P>,
}
