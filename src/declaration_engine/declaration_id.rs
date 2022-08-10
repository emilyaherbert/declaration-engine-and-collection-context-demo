#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct DeclarationId(usize);

impl std::ops::Deref for DeclarationId {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<usize> for DeclarationId {
    fn from(o: usize) -> Self {
        DeclarationId(o)
    }
}
