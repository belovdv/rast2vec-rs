use super::{ImageRGBA, Pos, PosGood};

use crate::utility::Color;

pub trait View {
    fn size(&self) -> Pos;

    /// Not efficient (O(`sq_dist`)) - assumes low `sq_dist`.
    /// Not efficient - it would be better to cache results.
    // To be done: check, if `smallvec` incerase performance.
    // To be done: check, if using `u16` will benefit.
    fn neigh_if(
        &self,
        of: Pos,
        sq_dist: usize,
        filter: &dyn Fn(PosGood) -> bool,
    ) -> smallvec::SmallVec<[PosGood; 4]> {
        let filter = |p| self.check_coord(p).map(|p| filter(p)).unwrap_or(false);
        of.neigh_if(sq_dist, &filter)
            .into_iter()
            .filter_map(|p| self.check_coord(p))
            .collect()
    }
    fn neigh(&self, of: Pos, sq_dist: usize) -> smallvec::SmallVec<[PosGood; 4]> {
        self.neigh_if(of, sq_dist, &|_| true)
    }
    /// For consistency with `neigh2` - direct (straight) neighbours.
    fn neigh1_if(
        &self,
        of: Pos,
        filter: &dyn Fn(PosGood) -> bool,
    ) -> smallvec::SmallVec<[PosGood; 4]> {
        self.neigh_if(of, 1, filter)
    }
    fn neigh1(&self, of: Pos) -> smallvec::SmallVec<[PosGood; 4]> {
        self.neigh1_if(of, &|_| true)
    }
    /// Direct and diagonal neighbours (or dist = 1 or 2).
    fn neigh2_if(
        &self,
        of: Pos,
        filter: &dyn Fn(PosGood) -> bool,
    ) -> smallvec::SmallVec<[PosGood; 8]> {
        let filter = |p| self.check_coord(p).map(|p| filter(p)).unwrap_or(false);
        of.neigh2_if(&filter)
            .into_iter()
            .filter_map(|p| self.check_coord(p))
            .collect()
    }
    fn neigh2(&self, of: Pos) -> smallvec::SmallVec<[PosGood; 8]> {
        self.neigh2_if(of, &|_| true)
    }

    fn contains(&self, c: Pos) -> bool {
        self.check_coord(c).is_some()
    }
    fn check_coord(&self, pos: Pos) -> Option<PosGood>;

    fn rows(&self) -> Rows {
        Rows {
            h: self.size().y,
            w: self.size().x,
            y: 0,
        }
    }

    /// To be done: expects, that `pos` is `PosGood` for this `View`.
    /// It isn't checked.
    fn get(&self, pos: PosGood) -> Color;

    fn neigh_of_knot(&self, pos: Pos) -> smallvec::SmallVec<[PosGood; 4]> {
        [Some(pos), pos.mv_lu(), pos.mv_u(), pos.mv_l()]
            .into_iter()
            .filter_map(|p| p)
            .filter_map(|p| self.check_coord(p))
            .collect()
    }

    fn between_knots(&self, k1: Pos, k2: Pos) -> Option<PosGood> {
        let dx = k1.x.abs_diff(k2.x);
        let dy = k1.y.abs_diff(k2.y);
        if dx != 1 || dy != 1 {
            return None;
        }
        match (k1, k2) {
            (k1, k2) if k1 < k2 => Some(unsafe { PosGood::from_pos(k1) }),
            (k1, k2) if k1 > k2 => Some(unsafe { PosGood::from_pos(k2) }),
            _ => Some(unsafe {
                PosGood::from_pos(Pos::new(
                    std::cmp::min(k1.y, k2.y),
                    std::cmp::min(k1.x, k2.x),
                ))
            }),
        }
    }
}

pub struct Rows {
    h: usize,
    w: usize,
    y: usize,
}
impl Iterator for Rows {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y != self.h {
            let res = Row {
                y: self.y,
                x: 0,
                w: self.w,
            };
            self.y += 1;
            return Some(res);
        }
        None
    }
}
pub struct Row {
    y: usize,
    x: usize,
    w: usize,
}
impl Iterator for Row {
    type Item = PosGood;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x != self.w {
            let res = Pos::new(self.y, self.x);
            let res = unsafe { PosGood::from_pos(res) };
            self.x += 1;
            return Some(res);
        }
        None
    }
}

impl View for ImageRGBA {
    fn size(&self) -> Pos {
        self.size()
    }

    fn get(&self, pos: PosGood) -> Color {
        self.get(pos.into()).unwrap()
    }

    fn check_coord(&self, pos: Pos) -> Option<PosGood> {
        PosGood::try_from_pos(pos, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neigh() {
        let img = ImageRGBA::new(Pos::new(10, 5));

        let p00 = Pos::new(0, 0);
        let p01 = Pos::new(0, 1);
        let p02 = Pos::new(0, 2);
        let p84 = Pos::new(8, 4);
        let p96 = Pos::new(9, 6);

        assert_eq!(img.neigh(p00, 0).len(), 1);
        assert_eq!(img.neigh(p00, 1).len(), 2);
        assert_eq!(img.neigh(p00, 2).len(), 1);
        assert_eq!(img.neigh(p00, 3).len(), 0);
        assert_eq!(img.neigh(p00, 4).len(), 2);
        assert_eq!(img.neigh(p00, 5).len(), 2);
        assert_eq!(img.neigh(p00, 6).len(), 0);
        assert_eq!(img.neigh(p00, 7).len(), 0);
        assert_eq!(img.neigh(p00, 8).len(), 1);
        assert_eq!(img.neigh(p00, 9).len(), 2);
        assert_eq!(img.neigh(p00, 10).len(), 2);
        assert_eq!(img.neigh(p00, 11).len(), 0);
        assert_eq!(img.neigh(p00, 12).len(), 0);
        assert_eq!(img.neigh(p00, 13).len(), 2);

        assert_eq!(img.neigh(p01, 0).len(), 1);
        assert_eq!(img.neigh(p01, 1).len(), 3);
        assert_eq!(img.neigh(p01, 2).len(), 2);
        assert_eq!(img.neigh(p01, 3).len(), 0);
        assert_eq!(img.neigh(p01, 4).len(), 2);
        assert_eq!(img.neigh(p01, 5).len(), 3);
        assert_eq!(img.neigh(p01, 6).len(), 0);
        assert_eq!(img.neigh(p01, 7).len(), 0);
        assert_eq!(img.neigh(p01, 8).len(), 1);

        assert_eq!(img.neigh(p02, 0).len(), 1);
        assert_eq!(img.neigh(p02, 1).len(), 3);
        assert_eq!(img.neigh(p02, 2).len(), 2);
        assert_eq!(img.neigh(p02, 3).len(), 0);
        assert_eq!(img.neigh(p02, 4).len(), 3);
        assert_eq!(img.neigh(p02, 9).len(), 1);

        assert_eq!(img.neigh(p84, 0).len(), 1);
        assert_eq!(img.neigh(p84, 1).len(), 3);
        assert_eq!(img.neigh(p84, 2).len(), 2);
        assert_eq!(img.neigh(p84, 3).len(), 0);
        assert_eq!(img.neigh(p84, 4).len(), 2);
        assert_eq!(img.neigh(p84, 5).len(), 3);
        assert_eq!(img.neigh(p84, 6).len(), 0);
        assert_eq!(img.neigh(p84, 7).len(), 0);
        assert_eq!(img.neigh(p84, 8).len(), 1);
        assert_eq!(img.neigh(p96, 0).len(), 0);
        assert_eq!(img.neigh(p96, 1).len(), 0);
        assert_eq!(img.neigh(p96, 2).len(), 0);
        assert_eq!(img.neigh(p96, 3).len(), 0);
        assert_eq!(img.neigh(p96, 4).len(), 1);
        assert_eq!(img.neigh(p96, 5).len(), 1);
        assert_eq!(img.neigh(p96, 6).len(), 0);
        assert_eq!(img.neigh(p96, 7).len(), 0);
        assert_eq!(img.neigh(p96, 8).len(), 1);

        cmp_vec(&img.neigh(p02, 2)[..], &[(1, 1), (1, 3)]);
        cmp_vec(&img.neigh(p96, 13)[..], &[(6, 4), (7, 3)]);
    }

    #[test]
    fn neigh2() {
        let img = ImageRGBA::new(Pos::new(10, 5));

        let p00 = Pos::new(0, 0);
        let p01 = Pos::new(0, 1);
        let p02 = Pos::new(0, 2);
        let p84 = Pos::new(8, 4);
        let p96 = Pos::new(9, 6);
        let p95 = Pos::new(9, 5);

        cmp_vec(&img.neigh2(p00)[..], &[(0, 1), (1, 0), (1, 1)]);
        cmp_vec(
            &img.neigh2(p01)[..],
            &[(0, 0), (0, 2), (1, 0), (1, 1), (1, 2)],
        );
        cmp_vec(
            &img.neigh2(p02)[..],
            &[(0, 1), (0, 3), (1, 3), (1, 1), (1, 2)],
        );
        cmp_vec(
            &img.neigh2(p84)[..],
            &[(7, 3), (7, 4), (8, 3), (9, 3), (9, 4)],
        );
        cmp_vec(&img.neigh2(p96)[..], &[]);
        cmp_vec(&img.neigh2(p95)[..], &[(8, 4), (9, 4)]);
    }

    use std::collections::HashSet;
    fn cmp_vec(a: &[PosGood], b: &[(usize, usize)]) {
        let la = a.len();
        let a: HashSet<_> = a.iter().map(|&c| c.into()).collect();
        assert_eq!(la, a.len());

        let lb = b.len();
        let b: HashSet<_> = b.iter().map(|&(y, x)| Pos::new(y, x)).collect();
        assert_eq!(lb, b.len());

        assert_eq!(a, b);
    }

    #[test]
    fn rows() {
        let img = ImageRGBA::new(Pos::new(4, 3));

        let r = vec![
            vec![Pos::new(0, 0), Pos::new(0, 1), Pos::new(0, 2)],
            vec![Pos::new(1, 0), Pos::new(1, 1), Pos::new(1, 2)],
            vec![Pos::new(2, 0), Pos::new(2, 1), Pos::new(2, 2)],
            vec![Pos::new(3, 0), Pos::new(3, 1), Pos::new(3, 2)],
        ];
        let t: Vec<Vec<_>> = View::rows(&img)
            .map(|r| r.map(|c| c.into()).collect())
            .collect();

        assert_eq!(r, t);
    }
}
