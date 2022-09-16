#[derive(Debug, Clone)]
pub(crate) enum CollectionEdge {
    ApplicationContents,
    FileContents,
    SharedScope,
    ScopedChild,
}
