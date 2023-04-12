use std::collections::HashSet;

use super::Pos;

pub fn pos_debug_hashset<P: Copy + Into<Pos>>(data: &HashSet<P>) -> String {
    let data: HashSet<Pos> = data.iter().map(|&p| p.into()).collect();

    let xm = data.iter().map(|p| p.x).max().unwrap_or(0);
    let ym = data.iter().map(|p| p.y).max().unwrap_or(0);

    let mut strs = vec![vec![0; xm + 1]; ym + 1];
    for pos in data {
        strs[pos.y][pos.x] = 1;
    }

    strs.into_iter()
        .map(|v| {
            v.into_iter()
                .map(|s| match s {
                    0 => " ",
                    1 => "1",
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<_>>()
        .join("\n")
}
