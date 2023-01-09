use std::collections::HashSet;

pub fn day12() {
    let map = include_bytes!("../../day12.txt");
    let start = map.iter().position(|&x| x == b'S').unwrap();
    let end = map.iter().position(|&x| x == b'E').unwrap();

    let result1 = spf(&map, start, b'E', |m, ix| neighborhood(m, ix, |s, c| s <= c + 1));
    let result2 = spf(&map, end, b'a', |m, ix| neighborhood(m, ix, |s, c| s >= c - 1));

    println!("DAY 12\nSolution 1: {result1}\nSolution 2: {result2}");
}

fn spf<const L: usize, F>(
    map: &[u8; L],
    start_position: usize,
    end: u8,
    neighborhood_fn: F,
) -> usize
where
    F: Fn(&[u8; L], usize) -> Vec<usize>,
{
    match (1..).try_fold(
        (
            HashSet::from([start_position]),
            HashSet::from([start_position]),
            1,
        ),
        |(frontier, mut visited, result), ix| {
            if frontier.iter().any(|&ix| map[ix] == end) {
                Err((frontier, visited, result))
            } else {
                visited.extend(frontier.iter());

                Ok((
                    frontier
                        .iter()
                        .flat_map(|&ix| neighborhood_fn(map, ix))
                        .filter(|v| !visited.contains(v))
                        .collect(),
                    visited,
                    ix,
                ))
            }
        },
    ) {
        Ok(_) => unreachable!(),
        Err((_, _, result)) => result,
    }
}

fn neighborhood<const L: usize, F>(map: &[u8; L], ix: usize, is_valid_step: F)  -> Vec<usize>
where
    F: Fn(u8, u8) -> bool {
    let mut n = vec![];
    let c = match map[ix] {
        b'S' => b'a',
        b'E' => b'z',
        c => c,
    };

    if ix % 183 > 0 && is_valid_step(map[ix - 1], c) {
        n.push(ix - 1);
    }

    if ix % 183 < 180 && is_valid_step(map[ix + 1], c) {
        n.push(ix + 1)
    }

    if ix > 183 && is_valid_step(map[ix - 183], c) {
        n.push(ix - 183)
    }

    if ix < L - 184 && is_valid_step(map[ix + 183], c) {
        n.push(ix + 183)
    }

    n
}
