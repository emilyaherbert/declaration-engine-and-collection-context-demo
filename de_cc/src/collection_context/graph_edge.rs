#[derive(Debug, Clone)]
pub(crate) enum GraphEdge {
    ApplicationContents,
    FileContents,
    SharedScope,
    NodeContents,
    DeclarationContents,
}
