use std::fmt;

#[derive(Debug, Clone)]
pub(crate) enum CollectionEdge {
    ApplicationContents,
    FileContents,
    SharedScope,
    ScopedChild,
}

impl fmt::Display for CollectionEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CollectionEdge::ApplicationContents => write!(f, "application contents"),
            CollectionEdge::FileContents => write!(f, "file contents"),
            CollectionEdge::SharedScope => write!(f, "shared scope"),
            CollectionEdge::ScopedChild => write!(f, "scoped child"),
        }
    }
}
