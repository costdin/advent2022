use std::collections::HashSet;

pub fn day12() {
    let map = include_bytes!("../../day12.txt");

    let (start, end, line_len, terminator_len) = match (
        map.iter().position(|&x| x == b'S'),
        map.iter().position(|&x| x == b'E'),
        map.iter().position(|&x| x == 10),
        map.iter().position(|&x| x == 13),
    ) {
        (Some(s), Some(e), Some(t), None) | (Some(s), Some(e), None, Some(t)) => (s, e, t + 1, 1),
        (Some(s), Some(e), Some(t), Some(n)) => (s, e, t.min(n) + 2, 2),
        _ => unreachable!(),
    };

    let result1 = spf(
        &map,
        start,
        |map, ix| neighborhood(map, ix, line_len, terminator_len, |s, c| s <= c + 1),
        |(ix, _)| ix == end,
    );
    let result2 = spf(
        &map,
        end,
        |map, ix| neighborhood(map, ix, line_len, terminator_len, |s, c| s >= c - 1),
        |(_, p)| p == b'a',
    );

    println!("DAY 12\nSolution 1: {result1}\nSolution 2: {result2}");
}

fn spf<const L: usize, NF, EF>(
    map: &[u8; L],
    start_position: usize,
    neighbors: NF,
    exit_condition: EF,
) -> usize
where
    NF: Fn(&[u8; L], usize) -> Vec<(usize, u8)>,
    EF: Fn((usize, u8)) -> bool,
{
    match (1usize..).try_fold(
        (
            HashSet::from([start_position]),
            HashSet::from([start_position]),
        ),
        |(frontier, mut visited), ix| {
            visited.extend(frontier.iter());

            Ok((
                frontier
                    .iter()
                    .flat_map(|&ix| neighbors(map, ix)) //neighborhood(map, ix, &is_valid_step))
                    .filter(|(ix, _)| !visited.contains(ix))
                    .try_fold(HashSet::new(), |mut acc, curr| {
                        if exit_condition(curr) {
                            Err(ix)
                        } else {
                            acc.insert(curr.0);
                            Ok(acc)
                        }
                    })?,
                visited,
            ))
        },
    ) {
        Ok(_) => unreachable!(),
        Err(result) => result,
    }
}

fn neighborhood<const L: usize, F>(
    map: &[u8; L],
    ix: usize,
    line_len: usize,
    terminator_len: usize,
    is_valid_step: F,
) -> Vec<(usize, u8)>
where
    F: Fn(u8, u8) -> bool,
{
    let mut n = vec![];
    let c = match map[ix] {
        b'S' => b'a',
        b'E' => b'z',
        c => c,
    };

    if ix % line_len > 0 && is_valid_step(map[ix - 1], c) {
        n.push((ix - 1, map[ix - 1]));
    }

    if ix % line_len < (line_len - terminator_len - 1) && is_valid_step(map[ix + 1], c) {
        n.push((ix + 1, map[ix + 1]));
    }

    if ix > line_len && is_valid_step(map[ix - line_len], c) {
        n.push((ix - line_len, map[ix - line_len]));
    }

    if ix < L - (line_len + 1) && is_valid_step(map[ix + line_len], c) {
        n.push((ix + line_len, map[ix + line_len]));
    }

    n
}
