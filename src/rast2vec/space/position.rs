#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub(super) y: usize,
    pub(super) x: usize,
}

impl Pos {
    pub(super) fn new(y: usize, x: usize) -> Self {
        Self { y, x }
    }

    pub fn dist_sq(self, other: Pos) -> usize {
        let dy = self.y.abs_diff(other.y);
        let dx = self.x.abs_diff(other.x);
        dy * dy + dx * dx
    }

    pub unsafe fn get(self) -> (usize, usize) {
        (self.y, self.x)
    }

    pub fn mv_l(self) -> Option<Self> {
        match (self.y, self.x) {
            (y, x) if x > 0 => Some(Self::new(y, x - 1)),
            _ => None,
        }
    }
    pub fn mv_lu(self) -> Option<Self> {
        match (self.y, self.x) {
            (y, x) if y > 0 && x > 0 => Some(Self::new(y - 1, x - 1)),
            _ => None,
        }
    }
    pub fn mv_u(self) -> Option<Self> {
        match (self.y, self.x) {
            (y, x) if y > 0 => Some(Self::new(y - 1, x)),
            _ => None,
        }
    }

    pub fn mv_d(self) -> Self {
        Self::new(self.y + 1, self.x)
    }
    pub fn mv_r(self) -> Self {
        Self::new(self.y, self.x + 1)
    }
    pub fn mv_rd(self) -> Self {
        Self::new(self.y + 1, self.x + 1)
    }

    pub fn neigh_if(
        self,
        sq_dist: usize,
        filter: &dyn Fn(Pos) -> bool,
    ) -> smallvec::SmallVec<[Pos; 4]> {
        let r = (0..).find(|r| (r + 1) * (r + 1) > sq_dist).unwrap();
        let ymi = std::cmp::max(self.y, r) - r;
        let xmi = std::cmp::max(self.x, r) - r;
        (ymi..self.y + r + 1)
            .flat_map(|y| (xmi..self.x + r + 1).map(move |x| Pos::new(y, x)))
            .filter(|&p| self.dist_sq(p) == sq_dist)
            .filter(|&p| filter(p))
            .collect()
    }

    /// Direct and diagonal neighbours (or dist = 1 or 2).
    pub fn neigh2_if(self, filter: &dyn Fn(Pos) -> bool) -> smallvec::SmallVec<[Pos; 8]> {
        let ymi = std::cmp::max(self.y, 1) - 1;
        let xmi = std::cmp::max(self.x, 1) - 1;
        let all = (ymi..self.y + 2).flat_map(|y| (xmi..self.x + 2).map(move |x| Pos::new(y, x)));
        all.filter(|&p| self != p && filter(p)).collect()
    }
}

impl std::ops::Add<Pos> for Pos {
    type Output = Pos;

    fn add(self, rhs: Pos) -> Self::Output {
        Pos::new(self.y + rhs.y, self.x + rhs.x)
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.y.partial_cmp(&other.y), self.x.partial_cmp(&other.x)) {
            (y, x) if y == x => y,
            _ => return None,
        }
    }
}

impl std::fmt::Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pos(y: {}, x: {})", self.y, self.x)
    }
}
