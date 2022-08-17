use std::fmt;

#[derive(PartialEq, Default, Clone)]
pub(crate) struct Path {
    path: Vec<String>,
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.path.join(""))
    }
}

impl Path {
    pub(crate) fn scoped(self, name: String) -> Path {
        let mut next_path = self.path;
        next_path.push(name);
        Path { path: next_path }
    }
}
