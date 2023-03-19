use std::collections::HashSet;

const ROW_COUNT: usize = 26;
const COL_COUNT: usize = 120;
//const ROW_COUNT: usize = 5;
//const COL_COUNT: usize = 6;
const ROTATING_ROWS: usize = ROW_COUNT - 1;

// 272 too low

pub fn day24() {
    let map = include_str!("../../day24.txt")
        .trim()
        .lines()
        .enumerate()
        .fold([[[false; COL_COUNT]; ROW_COUNT]; 4], |mut acc, (row, l)| {
            for i in l.chars().skip(1).enumerate() {
                match i {
                    (c, '<') => acc[0][row][c] = true,
                    (c, '>') => acc[1][row][c] = true,
                    (c, '^') => acc[2][row][c] = true,
                    (c, 'v') => acc[3][row][c] = true,
                    _ => {}
                }
            }

            acc
        });

    let (result1, result2) = match (0..).try_fold(
        (
            HashSet::from([(0, 0)]),
            vec![
                (ROW_COUNT - 1, COL_COUNT - 1),
                (1, 0),
                (ROW_COUNT - 1, COL_COUNT - 1),
            ],
            vec![],
        ),
        |(positions, mut destination, mut results), turn| {
            if destination.is_empty() {
                Err(results)
            } else if positions.contains(&destination.last().unwrap()) {
                results.push(turn + 1);
                Ok((
                    HashSet::from([destination.pop().unwrap()]),
                    destination,
                    results,
                ))
            } else {
                Ok((
                    positions
                        .into_iter()
                        .flat_map(|p| valid_moves(p, turn + 1, &map))
                        .collect(),
                    destination,
                    results,
                ))
            }
        },
    ) {
        Err(r) => (r[0], r[2]),
        _ => unreachable!(),
    };

    println!(
        "DAY 24\nSolution 1: {result1}\nSolution 2: {result2}"
    );
}

fn valid_moves(
    (r, c): (usize, usize),
    turn: usize,
    map: &[[[bool; COL_COUNT]; ROW_COUNT]; 4],
) -> impl Iterator<Item = (usize, usize)> + '_ {
    [
        (ROW_COUNT - 1, 0),
        (1, 0),
        (0, COL_COUNT - 1),
        (0, 1),
        (0, 0),
    ]
    .iter()
    .map(move |(dr, dc)| ((r + dr) % ROW_COUNT, (c + dc) % COL_COUNT))
    .filter(|&(nr, nc)| (nr, nc) == (0, 0) || (nr > 0 && nr < ROW_COUNT && nc < COL_COUNT))
    .filter(move |&(nr, nc)| (nr.max(r) - nr.min(r)) <= 1 && (nc.max(c) - nc.min(c)) <= 1)
    .filter(
        move |&(nr, nc)| {
            !map[0][nr][(nc + turn) % COL_COUNT] // Left
            && !map[1][nr][(nc + (COL_COUNT - 1) * turn) % COL_COUNT] // Right
            && !map[2][(nr + ROTATING_ROWS - 1 + turn) % ROTATING_ROWS + 1][nc] // Up
            && !map[3][(nr + (ROTATING_ROWS - 1) * (turn + 1)) % ROTATING_ROWS + 1][nc]
        }, // Down
    )
}
