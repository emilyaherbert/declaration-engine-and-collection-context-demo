#[derive(Debug)]
pub(crate) enum GraphEdge {
    ApplicationContents,
    FileContents,
    SharedScope,
    NodeContents,
    DeclarationContents,
}
