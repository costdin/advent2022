// Does this work will all input?
pub fn day17() {
    let figures = [
        vec![2, 3, 4, 5],
        vec![3, 9, 10, 11, 17],
        vec![2, 3, 4, 11, 18],
        vec![2, 9, 16, 23],
        vec![2, 3, 9, 10],
    ];
    let bytes = include_bytes!("../../day17.txt");
    let figures_iter = LoopIterator::new(&figures);
    let moves = LoopIterator::new(bytes).map(|c| if c == b'<' { -1 } else { 1i32 });

    let (result1, result2): (usize, usize) = match figures_iter.enumerate().try_fold(
        (moves, vec![false; 300000], vec![], 0, 0, 0u16),
        |(mut moves, mut acc, mut states, mut top, mut move_count, mut seq),
         (block_count, mut block)| {
            if block_count >= 2022 && move_count > bytes.len() {
                let (_, _, last_state) = states.last().unwrap();

                let cycle = states
                    .iter()
                    .filter(move |(_, bc, v)| v == last_state && *bc > (bytes.len() / 4))
                    .collect::<Vec<&(usize, usize, u128)>>();

                if cycle.len() > 1 {
                    let step = cycle[1].1 - cycle[0].1;
                    let cycle_growth = cycle[1].0 - cycle[0].0;
                    let cycle_start = 1000000000000 - cycle[1].1;
                    let cycle_count = cycle_start / step;
                    let missing = cycle_start % step;
                    let final_partial_growth =
                        states[(cycle[0].1 + missing)].0 - states[cycle[0].1].0;

                    return Err((
                        states[2021].0,
                        cycle_growth * cycle_count + cycle[1].0 + final_partial_growth - 1,
                    ));
                }
            }

            block.iter_mut().for_each(|v| *v += (top + 3) * 7);
            while acc.len() < block.last().unwrap() + 1 {
                acc.extend_from_slice(&[false; 28])
            }

            loop {
                seq <<= 1;

                let (bb, moved) = match moves.next().unwrap() {
                    -1 => {
                        seq |= 1;
                        left(block, &acc)
                    }
                    1 => right(block, &acc),
                    _ => unreachable!(),
                };

                move_count += 1;

                block = bb;

                if !moved {
                    block.iter().for_each(|ix| acc[*ix] = true);
                    top = top.max(1 + block.last().unwrap() / 7);
                    let row_number = (row_to_number2(top, &acc) << 16)
                        + (seq | (block_count % figures.len()) as u16) as u128;
                    states.push((top, block_count, row_number));

                    break;
                }
            }

            Ok((moves, acc, states, top, move_count, seq))
        },
    ) {
        Ok(_) => unreachable!(),
        Err((result1, result2)) => (result1, result2),
    };

    println!("DAY 17\nSolution 1: {result1:?}\nSolution 2: {result2:?}");
}

fn row_to_number2(top: usize, table: &Vec<bool>) -> u128 {
    (0..7).fold(0, |acc, x| {
        (acc << 16)
            + (0..65536)
                .take_while(|y| top >= *y && !table[(top - y) * 7 + x])
                .last()
                .unwrap() as u128
    })
}

fn left(mut block: Vec<usize>, map: &[bool]) -> (Vec<usize>, bool) {
    if !block.iter().any(|b| b % 7 == 0 || map[b - 1]) {
        block.iter_mut().for_each(|v| *v -= 1);
    }

    down(block, &map)
}

fn right(mut block: Vec<usize>, map: &[bool]) -> (Vec<usize>, bool) {
    if !block.iter().any(|b| b % 7 == 6 || map[b + 1]) {
        block.iter_mut().for_each(|v| *v += 1);
    }

    down(block, &map)
}

fn down(mut block: Vec<usize>, map: &[bool]) -> (Vec<usize>, bool) {
    let moved = if block.iter().any(|b| b < &7 || map[b - 7]) {
        false
    } else {
        block.iter_mut().for_each(|v| *v -= 7);

        true
    };

    (block, moved)
}

struct LoopIterator<'a, I> {
    array: &'a [I],
    counter: usize,
}

impl<'a, I> LoopIterator<'a, I> {
    pub fn new(array: &'a [I]) -> LoopIterator<'a, I> {
        LoopIterator { array, counter: 0 }
    }
}

impl<'a, I> Iterator for LoopIterator<'a, I>
where
    I: Clone,
{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        self.counter += 1;

        Some(self.array[(self.counter - 1) % self.array.len()].clone())
    }
}
