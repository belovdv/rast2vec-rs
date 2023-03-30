use std::{
    fmt::Debug,
    ops::{Add, Sub},
};

#[derive(Default, Hash, PartialEq, Eq, Clone, Copy, derive_new::new, PartialOrd, Ord)]
pub struct Coord {
    pub y: isize,
    pub x: isize,
}

impl Coord {
    pub fn abs_sq(&self) -> usize {
        (self.x * self.x + self.y * self.y) as usize
    }
    pub fn dist_sq(&self, other: Coord) -> usize {
        (*self - other).abs_sq()
    }
    /// [0,8). (1, 0) is 0, (0, 1) is 2.
    pub fn _angle(&self) -> f32 {
        match (self.x, self.y) {
            (0, 0) => panic!(),
            (0, y) if y < 0 => 2.,
            (0, y) if y > 0 => 6.,
            (x, 0) if x > 0 => 0.,
            (x, 0) if x < 0 => 4.,

            (x, y) if x > 0 && y < 0 && x.abs() >= y.abs() => y.abs() as f32 / x.abs() as f32,
            (x, y) if x < 0 && y < 0 && x.abs() <= y.abs() => x.abs() as f32 / y.abs() as f32 + 2.,
            (x, y) if x < 0 && y > 0 && x.abs() >= y.abs() => y.abs() as f32 / x.abs() as f32 + 4.,
            (x, y) if x > 0 && y > 0 && x.abs() <= y.abs() => x.abs() as f32 / y.abs() as f32 + 6.,

            (x, y) if x > 0 && y < 0 && x.abs() <= y.abs() => 2. - x.abs() as f32 / y.abs() as f32,
            (x, y) if x < 0 && y < 0 && x.abs() >= y.abs() => 4. - y.abs() as f32 / x.abs() as f32,
            (x, y) if x < 0 && y > 0 && x.abs() <= y.abs() => 6. - x.abs() as f32 / y.abs() as f32,
            (x, y) if x > 0 && y > 0 && x.abs() >= y.abs() => 8. - y.abs() as f32 / x.abs() as f32,

            _ => unreachable!(),
        }
    }
}

impl From<(usize, usize)> for Coord {
    fn from(value: (usize, usize)) -> Self {
        Self {
            y: value.0 as isize,
            x: value.1 as isize,
        }
    }
}
impl From<(isize, isize)> for Coord {
    fn from(value: (isize, isize)) -> Self {
        Self {
            y: value.0 as isize,
            x: value.1 as isize,
        }
    }
}
impl From<(i32, i32)> for Coord {
    fn from(value: (i32, i32)) -> Self {
        Self {
            y: value.0 as isize,
            x: value.1 as isize,
        }
    }
}
impl From<&(usize, usize)> for Coord {
    fn from(value: &(usize, usize)) -> Self {
        Self {
            y: value.0 as isize,
            x: value.1 as isize,
        }
    }
}
impl From<&(isize, isize)> for Coord {
    fn from(value: &(isize, isize)) -> Self {
        Self {
            y: value.0 as isize,
            x: value.1 as isize,
        }
    }
}

impl<T: Into<Coord>> Add<T> for Coord {
    type Output = Coord;

    fn add(self, rhs: T) -> Self::Output {
        let other: Coord = rhs.into();
        (self.y + other.y, self.x + other.x).into()
    }
}
impl<T: Into<Coord>> Sub<T> for Coord {
    type Output = Coord;

    fn sub(self, rhs: T) -> Self::Output {
        let other: Coord = rhs.into();
        (self.y - other.y, self.x - other.x).into()
    }
}

impl Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coord(y: {}, x: {})", self.y, self.x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn angle() {
        let v: [Coord; 12] = [
            (2, -0).into(),
            (2, -1).into(),
            (1, -2).into(),
            (0, -2).into(),
            (-1, -2).into(),
            (-2, -1).into(),
            (-2, -0).into(),
            (-2, 1).into(),
            (-1, 2).into(),
            (0, 2).into(),
            (1, 2).into(),
            (2, 1).into(),
        ];
        let a: Vec<_> = v
            .into_iter()
            .map(|c| Coord::from((c.x, c.y))._angle())
            .collect();
        dbg!(&a);
        assert!(a[0] == 0.);
        assert!(a[0] < a[1]);
        assert!(a[1] < a[2]);
        assert!(a[2] < a[3]);
        assert!(a[3] == 2.);
        assert!(a[3] < a[4]);
        assert!(a[4] < a[5]);
        assert!(a[5] < a[6]);
        assert!(a[6] == 4.);
        assert!(a[6] < a[7]);
        assert!(a[7] < a[8]);
        assert!(a[8] < a[9]);
        assert!(a[9] == 6.);
        assert!(a[9] < a[10]);
        assert!(a[10] < a[11]);
        // assert!(v[11]._angle() < v[12]._angle());
    }
}
