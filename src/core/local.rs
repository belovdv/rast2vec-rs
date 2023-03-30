#![allow(dead_code)]
use super::Coord;

pub const U: Coord = Coord { y: 0, x: 1 };
pub const D: Coord = Coord { y: 0, x: -1 };
pub const L: Coord = Coord { y: -1, x: 0 };
pub const R: Coord = Coord { y: 1, x: 0 };
pub const UR: Coord = Coord { y: 1, x: 1 };
pub const RU: Coord = Coord { y: 1, x: 1 };
pub const RD: Coord = Coord { y: 1, x: -1 };
pub const DR: Coord = Coord { y: 1, x: -1 };
pub const DL: Coord = Coord { y: -1, x: -1 };
pub const LD: Coord = Coord { y: -1, x: -1 };
pub const LU: Coord = Coord { y: -1, x: 1 };
pub const UL: Coord = Coord { y: -1, x: 1 };

lazy_static::lazy_static! {
    static ref RINGS: Vec<Vec<Coord>> = {
        let mut r = vec![vec![]; 100];
        (-5..5).for_each(|y| (-5..5).for_each(|x|
            r[(x * x + y * y) as usize].push((y, x).into())
        ));
        r
    };
}

fn _rings25() -> &'static Vec<Vec<Coord>> {
    &RINGS
}
/// Get set of `Coord`s at distance `dist` from `from` with applied `filter`.
pub fn get_dist(from: Coord, dist: usize, filter: &dyn Fn(Coord) -> bool) -> Vec<Coord> {
    assert!(dist < 25);
    _rings25()[dist]
        .iter()
        .map(|&c| c + from)
        .filter(|&c| filter(c))
        .collect()
}

/// Return 7 next dirs after `dir`.
pub fn rotations(dir: Coord) -> Vec<Coord> {
    assert!(dir.abs_sq() <= 2);
    let cycle = [U, UR, R, RD, D, DL, L, LU, U, UR, R, RD, D, DL, L, LU];
    let mut cycle = cycle.into_iter();
    while cycle.next().unwrap() != dir {}
    cycle.take(7).collect()
}

pub fn angle(before: Coord, center: Coord, after: Coord) -> isize {
    let r = rotations(before - center);
    match after - center {
        a if a == r[0] => -3,
        a if a == r[1] => -2,
        a if a == r[2] => -1,
        a if a == r[3] => 0,
        a if a == r[4] => 1,
        a if a == r[5] => 2,
        a if a == r[6] => 3,
        _ => {
            dbg!(before, center, after);
            panic!()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rings() {
        let r = [
            vec![Coord::new(0, 0)],
            vec![U, R, D, L],
            vec![UR, RD, DL, LU],
            vec![],
            vec![U + U, R + R, D + D, L + L],
        ];
        for d in 0..5 {
            for &s in &r[d] {
                assert!(_rings25()[d].iter().any(|&c| c == s));
            }
            for &s in &_rings25()[d] {
                assert!(r[d].iter().any(|&c| c == s));
            }
        }

        assert!(_rings25()[5].len() == 8);
        assert!(_rings25()[5].contains(&(-2, 1).into()));
        assert!(_rings25()[5].contains(&(-1, 2).into()));
        assert!(_rings25()[6].len() == 0);
        assert!(_rings25()[7].len() == 0);
        assert!(_rings25()[8].len() == 4);
        assert!(_rings25()[8].contains(&(-2, 2).into()));
    }
}
