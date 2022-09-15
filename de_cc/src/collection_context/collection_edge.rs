#[derive(Debug, Clone)]
pub(crate) enum CollectionEdge {
    ApplicationContents,
    FileContents,
    SharedScope,
    NodeContents,
    DeclarationContents,
    ScopedChild,
}
