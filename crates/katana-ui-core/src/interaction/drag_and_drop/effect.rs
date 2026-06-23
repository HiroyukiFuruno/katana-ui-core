use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DropEffect {
    #[default]
    None,
    Move,
    Copy,
    Link,
}
