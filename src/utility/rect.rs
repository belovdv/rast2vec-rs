use super::V;

#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_new::new)]
pub struct Rect {
    pub from: V,
    pub size: V,
}

impl Rect {
    pub fn new0(size: V) -> Self {
        Self::new((0, 0).into(), size)
    }

    pub fn check(self, v: V) -> Option<V> {
        match self.lu().leq(v) && v.le(self.rd()) {
            true => Some(v),
            false => None,
        }
    }
    pub fn contains(self, other: Rect) -> bool {
        self.lu().leq(other.lu()) && other.rd().leq(self.rd())
    }
    pub fn lu(self) -> V {
        self.from
    }
    pub fn rd(self) -> V {
        self.from + self.size
    }
    pub fn iter(self) -> TwoDimIter {
        TwoDimIter::new(self.lu(), self.rd())
    }
}

pub struct TwoDimIter {
    from: V,
    to: V,
    pos: V,
}

impl TwoDimIter {
    pub fn new(from: V, to: V) -> Self {
        assert!(from.le(to));
        let pos = from;
        Self { from, to, pos }
    }
}

impl Iterator for TwoDimIter {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.y == self.to.y {
            return None;
        }
        let res = self.pos;
        self.pos.x += 1;
        if self.pos.x == self.to.x {
            self.pos.x = self.from.x;
            self.pos.y += 1;
        }
        Some(res)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn two_dim_iter() {
        let r = Rect::new((20, 20).into(), (1, 1).into());
        assert_eq!(r.iter().take(10).count(), 1);

        let r = Rect::new((15, 16).into(), (2, 2).into());
        let r: Vec<_> = r.iter().collect();
        assert_eq!(
            r,
            vec![
                V::new(15, 16),
                V::new(15, 17),
                V::new(16, 16),
                V::new(16, 17)
            ]
        )
    }
}
