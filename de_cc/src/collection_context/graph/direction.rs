#[derive(PartialEq, Clone, Copy)]
pub(crate) enum Direction {
    Incoming,
    Outgoing,
}

impl Direction {
    /// Return `0` for `Outgoing` and `1` for `Incoming`.
    pub(super) fn index(&self) -> usize {
        match self {
            Direction::Incoming => 1,
            Direction::Outgoing => 0,
        }
    }
}
