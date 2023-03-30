use super::*;

#[test]
fn nearest_right() {
    let from: Coord = (5, 4).into();
    let to: Coord = (5, 5).into();
    let next = vec![(4, 6).into(), (6, 6).into()];
    let next = Graph::_nearest_right(from, to, next);
    assert_eq!(next, (6, 6).into());

    let from: Coord = (5, 4).into();
    let to: Coord = (5, 5).into();
    let next = vec![(4, 6).into(), (6, 6).into(), (4, 4).into()];
    let next = Graph::_nearest_right(from, to, next);
    assert_eq!(next, (6, 6).into());

    eprintln!();
    let from: Coord = (5, 4).into();
    let to: Coord = (5, 5).into();
    let next = vec![(4, 6).into(), (6, 6).into(), (4, 4).into(), (6, 4).into()];
    let next = Graph::_nearest_right(from, to, next);
    assert_eq!(next, (6, 4).into());
}

// Out of date.

#[allow(dead_code)]
// #[test]
fn bfs() {
    let input = vec![
        " 0            ",
        " 0000         ",
        "0 0           ",
        " 0 0          ",
        " 0            ",
    ];
    let plain = Plain::new_test_vsp(&input);

    let (layers, dists) = plain.bfs_cond((0, 1).into(), 10, &|c| {
        c != (1, 4).into() && c != (2, 2).into() && plain.contains(c)
    });

    for y in 0..plain.h() {
        for x in 0..plain.w() {
            if input[y].chars().nth(x) == Some(' ') {
                assert!(dists.get(&(y, x).into()).is_none());
            }
        }
    }
    assert_eq!(dists.get(&(1, 4).into()), None);
    assert_eq!(dists.get(&(2, 2).into()), None);
    assert_eq!(dists.get(&(3, 3).into()), None);

    macro_rules! check {
        ($(($coord:tt, $value:expr)),*) => {
            $(
                assert!(layers[$value].contains(&$coord.into()));
                assert!(dists.get(&$coord.into()) == Some(&$value));
            )*
        };
    }

    dbg!(&layers);

    check!(
        ((0, 1), 0),
        ((1, 1), 1),
        ((1, 2), 2),
        ((1, 3), 3),
        ((2, 0), 3),
        ((3, 1), 5),
        ((4, 1), 6)
    );
}

#[allow(dead_code)]
// #[test]
fn knots() {
    let input = vec![
        " 0 0 0 00   ",
        "  0 0   0   ",
        "  0      000",
        "0  0        ",
        "0  000000000",
        "0  0       0",
        "   0 0    0 ",
        "   0   00 0 ",
        "   0     00 ",
        "   0000000  ",
    ];
    let plain = Plain::new_test_vsp(&input);

    let mut nodes = vec![vec![" "; input[0].len()]; input.len()];
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if plain.contains((y, x)) {
                nodes[y][x] = match plain.determine_type_1((y, x).into()) {
                    _ => "?",
                }
            }
        }
    }
    eprintln!("===== ===== stderr ===== =====");
    nodes.iter().for_each(|l| eprintln!("{}", l.join("")));
    eprintln!("===== =====  end   ===== =====");

    panic!()
}

#[test]
fn test_copilot() {}
