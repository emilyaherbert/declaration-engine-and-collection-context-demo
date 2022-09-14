pub(crate) struct VisitMap {
    visited: Vec<bool>,
}

impl VisitMap {
    pub(crate) fn new(n: usize) -> VisitMap {
        VisitMap {
            visited: vec![false; n],
        }
    }

    /// returns true if it has not been visited
    pub(crate) fn visit(&mut self, i: usize) -> bool {
        let visited = self.visited[i];
        self.visited[i] = true;
        !visited
    }
}
