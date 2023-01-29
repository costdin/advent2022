pub fn day17() {
    let figures = [
        vec![2, 3, 4, 5],
        vec![3, 9, 10, 11, 17],
        vec![2, 3, 4, 11, 18],
        vec![2, 9, 16, 23],
        vec![2, 3, 9, 10],
    ];
    let bytes = include_bytes!("../../day17.txt");

    let loop_len = figures.len() * bytes.len();
    println!("{loop_len}");

    let figures_iter = LoopIterator::new(&figures);
    let moves = LoopIterator::new(bytes).map(|c| if c == b'<' { -1 } else { 1i32 });

    let (result1, result2) : (usize, usize) = match figures_iter.enumerate().try_fold(
        (moves, vec![false; 300000], 0, 0, 0),
        |(mut moves, mut acc, top2022, mut top, mut move_count), (block_count, mut block)| {
            if block_count >= 2022 && move_count > bytes.len() {
                let c1 = row_to_number(top - 1, &acc);
                let vvvv = (1..)
                    .take_while(|vv| *vv + 16 <= top)
                    .map(|vv| top - vv)
                    .map(|r| row_to_number(r, &acc))
                    .enumerate()
                    .filter(move |(_, v)| *v == c1)
                    .collect::<Vec<_>>();

                if vvvv.len() > 2 {
                    println!("{move_count}");
                    println!("{vvvv:?}");

                    return Err((top2022, top));
                }
            }

            block.iter_mut().for_each(|v| *v += (top + 3) * 7);
            while acc.len() < block.last().unwrap() + 1 {
                acc.extend_from_slice(&[false; 28])
            }

            loop {
                let (bb, moved) = match moves.next().unwrap() {
                    -1 => left(block, &acc),
                    1 => right(block, &acc),
                    _ => unreachable!(),
                };

                move_count += 1;

                block = bb;

                if !moved {
                    block.iter().for_each(|ix| {
                        top = top.max(1 + *ix / 7);
                        acc[*ix] = true;
                    });
                    break;
                }
            }

            Ok((moves, acc, if block_count >= 2022 { top2022 } else { top }, top, move_count))
        },
    ) {
        Ok(_) => unreachable!(),
        Err((result1, result2)) => (result1, result2)
    };

    println!("DAY 17\nSolution 1: {result1:?}\nSolution 2: \n{result1:?}");
}

fn row_to_number(row: usize, table: &Vec<bool>) -> u128 {
    (0..16).map(|i| row - i).fold(0, |acc, r| {
        (acc << 8)
            + (0..7)
                .map(|i| r * 7 + i)
                .fold(0, |acc, v| (acc << 1) + if table[v] { 1 } else { 0 })
    })
}

fn print_row(row: usize, table: &Vec<bool>) {
    (0..16).map(|i| row - i).for_each(|r| {
        (0..7)
            .map(|i| r * 7 + i)
            .for_each(|v| print!("{}", if table[v] { '#' } else { '.' }));

        println!("");
    });

    println!("");
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
