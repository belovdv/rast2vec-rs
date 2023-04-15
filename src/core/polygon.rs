use std::collections::HashSet;

// use crate::utility::debug::pos_debug_hashset;

use super::{Color, Image, Partition, V};

pub struct Polygon {
    pub frame: Vec<Knot>,
    pub color: Color,
}

impl Polygon {
    pub fn from_partition(pn: &Partition, src: &Image) -> Vec<Self> {
        pn.areas
            .iter()
            .map(|a| Polygon {
                frame: Self::from_area(a),
                color: src.color(a),
            })
            .collect()
    }

    fn from_area(area: &HashSet<V>) -> Vec<Knot> {
        let knots: HashSet<_> = area
            .iter()
            .flat_map(|&v| Knot::from_v(v))
            .filter(|&k| !k.neigh_v(&|v| !area.contains(&v)).is_empty())
            .collect();

        let knot_my = knots.iter().map(|&k| k.v.y).min().unwrap();
        let knot_mx = knots
            .iter()
            .filter(|&k| k.v.y == knot_my)
            .map(|&k| k.v.x)
            .min()
            .unwrap();
        let start = Knot {
            v: V::new(knot_my, knot_mx),
        };
        if !knots.contains(&start) {
            dbg!(knots, start);
            panic!()
        }

        // for y in 10..25 {
        //     let mut row = Vec::from_iter(knots.iter().filter(|k| k.v.y == y).map(|&k| k));
        //     row.sort_by(|a, b| a.v.x.cmp(&b.v.x));
        //     dbg!(row);
        // }
        // eprintln!("debug area:\n{}", pos_debug_hashset(&area));

        let start_2 = start.neigh(&|k| knots.contains(&k))[0];
        let mut frame = vec![start, start_2];
        while frame.first() != frame.last() {
            let len = frame.len();
            let (prev, cur) = (frame[len - 2], frame[len - 1]);
            let nexts = cur.neigh(&|k| {
                let r = k != prev
                    && knots.contains(&k)
                    && matches!(
                        Knot::between(cur, k),
                        BetweenKnotPair::Straight([a, b]) if area.contains(&a) != area.contains(&b)
                    );
                // dbg!((r, cur, k, Knot::between(cur, k)));
                r
            });

            match nexts.len() {
                1 => frame.push(nexts[0]),
                3 => {
                    let nexts = nexts.iter().filter_map(|&k| match Knot::between(prev, k) {
                        BetweenKnotPair::Diag(v) if area.contains(&v) => Some(k),
                        _ => None,
                    });
                    let next = nexts.clone().next().unwrap();
                    match nexts.clone().count() {
                        1 => frame.push(next),
                        _ => {
                            dbg!(frame, nexts.collect::<Vec<_>>(), next);
                            panic!()
                        }
                    }
                }
                _ => {
                    dbg!(frame, nexts);
                    panic!()
                }
            }
            if frame.len() > 15000 {
                dbg!(frame);
                panic!()
            }
        }
        // dbg!(&frame);
        frame
    }
}

#[derive(derive_new::new, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Knot {
    pub v: V,
}

#[derive(Debug, PartialEq, Eq)]
enum BetweenKnotPair {
    Err,
    Diag(V),
    Straight([V; 2]),
}

impl Knot {
    fn from_v(v: V) -> smallvec::SmallVec<[Knot; 4]> {
        let cands = [v, v.r(), v.d(), v.pad(1)];
        cands.into_iter().map(|c| Knot::new(c)).collect()
    }

    fn neigh_v(self, filter: &dyn Fn(V) -> bool) -> smallvec::SmallVec<[V; 4]> {
        let v = self.v;
        [v, v.u(), v.l(), (v.y - 1, v.x - 1).into()]
            .into_iter()
            .filter(|&v| filter(v))
            .collect()
    }

    fn neigh(self, filter: &dyn Fn(Knot) -> bool) -> smallvec::SmallVec<[Knot; 4]> {
        self.v
            .neigh1f(&|v| filter(Knot { v }))
            .into_iter()
            .map(|v| Knot { v })
            .collect()
    }

    fn between(a: Knot, b: Knot) -> BetweenKnotPair {
        let v = V::min_common(a.v, b.v);
        match a.v.sq_dist(b.v) {
            1 if a.v.x == b.v.x => BetweenKnotPair::Straight([v, v.l()]),
            1 if a.v.y == b.v.y => BetweenKnotPair::Straight([v, v.u()]),
            2 => BetweenKnotPair::Diag(v),
            _ => BetweenKnotPair::Err,
        }
    }
}

impl std::fmt::Debug for Knot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Knot({}, {})", self.v.y, self.v.x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn knots() {
        let v = V::new(5, 4);
        let knots = Knot::from_v(v);
        assert_eq!(
            &knots[..],
            &[
                Knot::new(V::new(5, 4)),
                Knot::new(V::new(5, 5)),
                Knot::new(V::new(6, 4)),
                Knot::new(V::new(6, 5)),
            ]
        );
        let neigh_k0 = knots[0].neigh_v(&|kv| kv != v);
        assert_eq!(&neigh_k0[..], &[V::new(4, 4), V::new(5, 3), V::new(4, 3),]);

        assert_eq!(
            Knot::between(knots[0], knots[1]),
            BetweenKnotPair::Straight([V::new(5, 4), V::new(4, 4)])
        );
        assert_eq!(
            Knot::between(knots[0], Knot { v: V::new(7, 4) }),
            BetweenKnotPair::Err
        );
        assert_eq!(
            Knot::between(knots[0], knots[3]),
            BetweenKnotPair::Diag(V::new(5, 4))
        );
    }
}
