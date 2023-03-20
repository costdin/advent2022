use std::collections::{HashMap, HashSet};

const COL_COUNT: usize = 74;

pub fn day23() {
    let map = include_bytes!("../../day23.txt")
        .into_iter()
        .enumerate()
        .filter(|(_, r)| r == &&b'#')
        .map(|(ix, _)| (((ix / COL_COUNT) as i16, (ix % COL_COUNT) as i16)))
        .collect::<HashSet<_>>();

    let (result1, result2) = match (0..).try_fold(
        (None, None, map),
        |(result10, result_moved, acc), round| match (result10, result_moved) {
            (Some(r10), Some(rm)) => Err((r10, rm)),
            _ => {
                let (map, moved) = acc
                    .iter()
                    .map(|c| (*c, next_position(round, &acc, *c)))
                    .fold(
                        (HashMap::<(i16, i16), Vec<(i16, i16)>>::new(), 0),
                        |(mut i_acc, move_count), (start, end)| match i_acc.get_mut(&end) {
                            Some(v) if v.len() == 1 => {
                                v.push(start);
                                (i_acc, move_count - 1)
                            }
                            Some(v) => {
                                v.push(start);
                                (i_acc, move_count)
                            }
                            None if start != end => {
                                i_acc.insert(end, vec![start]);
                                (i_acc, move_count + 1)
                            }
                            None => {
                                i_acc.insert(end, vec![start]);
                                (i_acc, move_count)
                            }
                        },
                    );

                let np = map
                    .into_iter()
                    .flat_map(|(key, value)| if value.len() == 1 { vec![key] } else { value })
                    .collect::<HashSet<_>>();

                Ok((
                    if round == 10 {
                        Some(result(&np))
                    } else {
                        result10
                    },
                    result_moved.or(if moved == 0 { Some(round + 1) } else { None }),
                    np,
                ))
            }
        },
    ) {
        Err((result1, result2)) => (result1, result2),
        _ => unreachable!(),
    };

    println!(
        "DAY 23\nSolution 1: {} \nSolution 2: {}\n",
        result1, result2
    );
}

fn result(map: &HashSet<(i16, i16)>) -> usize {
    let (l, u, r, d, cnt) = map.iter().fold(
        (&i16::MAX, &i16::MAX, &i16::MIN, &i16::MIN, 0),
        |(l, u, r, d, cnt), (x, y)| (l.min(x), u.min(y), r.max(x), d.max(y), cnt + 1),
    );

    (r - l + 1) as usize * (d - u + 1) as usize - cnt
}

fn next_position(round: usize, map: &HashSet<(i16, i16)>, c: (i16, i16)) -> (i16, i16) {
    (round..4 + round)
        .map(|r| match r % 4 {
            0 => north(map, c),
            1 => south(map, c),
            2 => west(map, c),
            3 => east(map, c),
            _ => unreachable!(),
        })
        .filter_map(|r| r)
        .fold((None, 0), |(res, cnt), curr| match (res, cnt, curr) {
            (_, 3, _) => (None, 4),
            (None, _, c) => (Some(c), cnt + 1),
            (v, _, _) => (v, cnt + 1),
        })
        .0
        .unwrap_or(c)
}

fn north(map: &HashSet<(i16, i16)>, (x, y): (i16, i16)) -> Option<(i16, i16)> {
    if (-1i8..=1).all(|ny| !map.contains(&(x - 1, (y as i8 + ny) as i16))) {
        Some((x - 1, y))
    } else {
        None
    }
}

fn south(map: &HashSet<(i16, i16)>, (x, y): (i16, i16)) -> Option<(i16, i16)> {
    if (-1i8..=1).all(|ny| !map.contains(&(x + 1, (y as i8 + ny) as i16))) {
        Some((x + 1, y))
    } else {
        None
    }
}

fn east(map: &HashSet<(i16, i16)>, (x, y): (i16, i16)) -> Option<(i16, i16)> {
    if (-1i8..=1).all(|nx| !map.contains(&((x as i8 + nx) as i16, y + 1))) {
        Some((x, y + 1))
    } else {
        None
    }
}

fn west(map: &HashSet<(i16, i16)>, (x, y): (i16, i16)) -> Option<(i16, i16)> {
    if (-1i8..=1).all(|nx| !map.contains(&((x as i8 + nx) as i16, y - 1))) {
        Some((x, y - 1))
    } else {
        None
    }
}
