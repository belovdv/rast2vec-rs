use super::{Pos, View};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct PosGood {
    // To be done: track `View`.
    inner: Pos,
}

impl PosGood {
    /// Warning: it assumes caller knows what it's doing.
    pub fn try_from_pos(inner: Pos, view: &dyn View) -> Option<Self> {
        match inner < view.size() {
            true => Some(PosGood { inner }),
            false => None,
        }
    }

    pub unsafe fn from_pos(inner: Pos) -> Self {
        PosGood { inner }
    }
}

impl Into<Pos> for PosGood {
    fn into(self) -> Pos {
        self.inner
    }
}

impl std::fmt::Debug for PosGood {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PosGood(y: {}, x: {})", self.inner.y, self.inner.x)
    }
}
