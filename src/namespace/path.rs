#[derive(PartialEq)]
pub(crate) struct Path {
    path: Vec<String>,
}

impl Path {
    pub(crate) fn scoped(self, name: String) -> Path {
        let mut next_path = self.path;
        next_path.push(name);
        Path { path: next_path }
    }
}
