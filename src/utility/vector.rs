#[derive(derive_new::new, Clone, Copy, PartialEq, Eq, Hash)]
pub struct V {
    pub y: usize,
    pub x: usize,
}

impl V {
    pub fn pad(self, pad: usize) -> Self {
        Self::new(self.y + pad, self.x + pad)
    }

    pub fn le(self, v: V) -> bool {
        self.y < v.y && self.x < v.x
    }
    pub fn leq(self, v: V) -> bool {
        self.y <= v.y && self.x <= v.x
    }

    pub fn u(self) -> Self {
        Self::new(self.y - 1, self.x)
    }
    pub fn r(self) -> Self {
        Self::new(self.y, self.x + 1)
    }
    pub fn d(self) -> Self {
        Self::new(self.y + 1, self.x)
    }
    pub fn l(self) -> Self {
        Self::new(self.y, self.x - 1)
    }

    pub fn sq_dist(self, other: V) -> usize {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        dx * dx + dy * dy
    }

    pub fn min_common(a: V, b: V) -> V {
        V::new(std::cmp::min(a.y, b.y), std::cmp::min(a.x, b.x))
    }
}

impl V {
    /// Expects to be valid (more zero).
    pub fn neigh1(self) -> [V; 4] {
        [self.u(), self.r(), self.d(), self.l()]
    }
    /// Expects to be valid (more zero).
    pub fn neigh1f(self, filter: &dyn Fn(V) -> bool) -> smallvec::SmallVec<[V; 4]> {
        self.neigh1().into_iter().filter(|&v| filter(v)).collect()
    }
}

macro_rules! impl_from {
    ($($T:ty),*) => {
        $(impl From<($T, $T)> for V {
            fn from(value: ($T, $T)) -> Self {
                Self::new(value.0 as usize, value.1 as usize)
            }
        })*
    };
}

impl_from!(usize, i32, u32);

impl std::ops::Add<V> for V {
    type Output = Self;

    fn add(self, rhs: V) -> Self::Output {
        Self::new(self.y + rhs.y, self.x + rhs.x)
    }
}

impl std::fmt::Debug for V {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector(y: {}, x: {})", self.y, self.x)
    }
}
