use std::collections::HashSet;
const LEN: usize = 99;

fn to_square(a: &[u8; (LEN + 2) * LEN - 2]) -> [[[u8; LEN]; LEN]; 2] {
    let mut result = [[0; LEN]; LEN];
    let mut rotated = [[0; LEN]; LEN];

    for (ix, b) in a.iter().enumerate().filter(|(ix, _)| ix % (LEN + 2) < LEN) {
        result[ix % (LEN + 2)][ix / (LEN + 2)] = *b - b'0';
        rotated[ix / (LEN + 2)][ix % (LEN + 2)] = *b - b'0';
    }

    [result, rotated]
}

pub fn day8() {
    let x = include_bytes!("../../day8.txt");
    let matrices = to_square(x);

    let result1 = matrices
        .iter()
        .enumerate()
        .flat_map(|(mix, matrix)| {
            matrix
                .iter()
                .enumerate()
                .filter(|(ix, _)| *ix != 0 && *ix != LEN - 1)
                .map(move |(rix, row)| {
                    row.iter()
                        .enumerate()
                        .skip(1)
                        .take_while(|(ix, _)| *ix != LEN - 1)
                        .fold((vec![], rix, mix == 1, row[0], 0), f1)
                })
                .zip(matrix.iter().skip(1))
                .map(|((set, rix, rotated, _, pos), row)| {
                    row.iter()
                        .enumerate()
                        .skip((pos + 1).into())
                        .rev()
                        .skip(1)
                        .fold((set, rix, rotated, row[LEN - 1], 0), f1)
                })
                .map(|(set, _, _, _, _)| set)
        })
        .flat_map(|v| v)
        .collect::<HashSet<_>>()
        .len()
        + LEN * 4
        - 4;

    let result2 = matrices
        .iter()
        .map(|matrix| {
            matrix
                .iter()
                .map(|row| {
                    row.iter().enumerate().fold(
                        ([0; 10], [None; 10], [0; LEN]),
                        |(mut counters, mut last_seen, mut row), (ix, &curr)| {
                            row[ix] = counters[curr as usize];

                            last_seen
                                .iter_mut()
                                .take(curr as usize + 1)
                                .filter(|c| c.is_some())
                                .for_each(|x| {
                                    row[x.unwrap()] *= ix - x.unwrap();
                                    *x = None;
                                });

                            last_seen[curr as usize] = Some(ix);
                            counters
                                .iter_mut()
                                .take(curr as usize + 1)
                                .for_each(|v| *v = 1);

                            counters
                                .iter_mut()
                                .skip(curr as usize + 1)
                                .for_each(|v| *v += 1);

                            (counters, last_seen, row)
                        },
                    )
                })
                .flat_map(|(_, last_seen, mut row)| {
                    last_seen
                        .iter()
                        .filter(|c| c.is_some())
                        .for_each(|x| row[x.unwrap()] *= LEN - 1 - x.unwrap());

                    row[LEN - 1] = 0;

                    row
                })
                .collect::<Vec<_>>()
        })
        .enumerate()
        .map(|(mix, v)| if mix == 1 { rotate(v, LEN) } else { v })
        .fold(vec![1; LEN * LEN], |mut acc, curr| {
            acc.iter_mut().zip(curr).for_each(|(a, c)| *a *= c);
            acc
        })
        .into_iter()
        .max()
        .unwrap();

    println!("DAY 8\nSolution 1: {result1:?}\nSolution 2: {result2:?}\n");
}

pub fn rotate(mut v: Vec<usize>, len: usize) -> Vec<usize> {
    (0..len)
        .flat_map(|x| (x..len).map(move |y| (x, y)))
        .map(|(x, y)| (x * len + y, y * len + x))
        .for_each(|(src, dst)| v.swap(src, dst));

    v
}

pub fn f1(
    (mut map, rix, rotated, tallest, tallest_pos): (Vec<[usize; 2]>, usize, bool, u8, u8),
    (ix, curr): (usize, &u8),
) -> (Vec<[usize; 2]>, usize, bool, u8, u8) {
    if *curr > tallest {
        map.push(if rotated { [rix, ix] } else { [ix, rix] });
        (map, rix, rotated, *curr, ix as u8)
    } else {
        (map, rix, rotated, tallest, tallest_pos)
    }
}
