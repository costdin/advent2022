use itertools::Itertools;

type NextFn = Box<dyn Fn(&Vec<Vec<u8>>, usize, usize, Direction) -> ((usize, usize), Direction)>;

pub fn day22() {
    let (map, instructions) = include_bytes!("../../day22.txt")
        .iter()
        .group_by(|b| b.is_ascii_control())
        .into_iter()
        .map(|(_, r)| r.cloned().collect::<Vec<_>>())
        .filter(|r| !r.iter().any(|c| c.is_ascii_control()))
        .fold(
            (vec![], InstructionIterator::new(vec![].into_iter())),
            |(mut map, instructions), curr| {
                if curr[0].is_ascii_alphanumeric() {
                    (map, InstructionIterator::new(curr.into_iter()))
                } else {
                    map.push(curr);
                    (map, instructions)
                }
            },
        );

    let fns: [NextFn; 2] = [
        Box::new(|map, x, y, direction| next(map, x, y, direction)),
        Box::new(|map, x, y, direction| next_cube(map, x, y, direction)),
    ];

    let inst = instructions.collect::<Vec<_>>();

    let result1 = to_score(inst.iter().fold(
        (
            (0usize, map[0].iter().position(|p| p != &b' ').unwrap()),
            Direction::Right,
        ),
        |((x, y), direction), instruction| match (instruction, direction) {
            (Instruction::RotateLeft, d) => ((x, y), d.rotate_left()),
            (Instruction::RotateRight, d) => ((x, y), d.rotate_right()),
            (Instruction::Move(c), d) => {
                match (0..*c).try_fold(((x, y), d), |((x, y), d), _| {
                    let ((nx, ny), d) = next(&map, x, y, d);

                    if map[nx][ny] == b'#' {
                        Err(((x, y), d))
                    } else {
                        Ok(((nx, ny), d))
                    }
                }) {
                    Err((r, d)) | Ok((r, d)) => (r, d),
                }
            }
        },
    ));

    let result2 = to_score(inst.iter().fold(
        (
            (0usize, map[0].iter().position(|p| p != &b' ').unwrap()),
            Direction::Right,
        ),
        |((x, y), direction), instruction| match (instruction, direction) {
            (Instruction::RotateLeft, d) => ((x, y), d.rotate_left()),
            (Instruction::RotateRight, d) => ((x, y), d.rotate_right()),
            (Instruction::Move(c), d) => {
                match (0..*c).try_fold(((x, y), d), |((x, y), d), _| {
                    let ((nx, ny), d) = next_cube(&map, x, y, d);

                    if map[nx][ny] == b'#' {
                        Err(((x, y), d))
                    } else {
                        Ok(((nx, ny), d))
                    }
                }) {
                    Err((r, d)) | Ok((r, d)) => (r, d),
                }
            }
        },
    ));

    println!("DAY 21\nSolution 1: {result1:?} \nSolution 2: {result2:?}\n");
}

fn to_score(((x, y), direction): ((usize, usize), Direction)) -> usize {
    1000 * (x + 1) + 4 * (y + 1) + direction.to_score()
}

fn next(
    map: &Vec<Vec<u8>>,
    x: usize,
    y: usize,
    direction: Direction,
) -> ((usize, usize), Direction) {
    let (dx, dy) = direction.to_deltas();
    let (mut nx, mut ny) = (x, y);
    loop {
        (nx, ny) = if dx != 0 {
            (
                (nx as isize + dx).rem_euclid(map.len() as isize) as usize,
                ny,
            )
        } else {
            (
                nx,
                (ny as isize + dy).rem_euclid(map[nx].len() as isize) as usize,
            )
        };

        if map[nx].len() > ny && (map[nx][ny] == b'#' || map[nx][ny] == b'.') {
            return ((nx, ny), direction);
        }
    }
}

fn next_cube(
    map: &Vec<Vec<u8>>,
    x: usize,
    y: usize,
    direction: Direction,
) -> ((usize, usize), Direction) {
    let (dx, dy) = direction.to_deltas();

    let (nx, ny, nd) = match (x as isize + dx, y as isize + dy, &direction) {
        (-1, y, Direction::Up) if y >= 50 && y < 100 => (y + 100, 0, Direction::Right), // 1 -> 6
        (x, 49, Direction::Left) if x >= 0 && x < 50 => (149 - x, 0, Direction::Right), // 1 -> 5
        (-1, y, Direction::Up) if y >= 100 && y < 150 => (199, y - 100, Direction::Up), // 2 -> 6
        (50, y, Direction::Down) if y >= 100 && y < 150 => (y - 50, 99, Direction::Left), // 2 -> 3
        (x, 150, Direction::Right) if x >= 0 && x < 50 => (149 - x, 99, Direction::Left), // 2 -> 4
        (x, 49, Direction::Left) if x >= 50 && x < 100 => (100, x - 50, Direction::Down), // 3 -> 5
        (x, 100, Direction::Right) if x >= 50 && x < 100 => (49, x + 50, Direction::Up), // 3 -> 2
        (x, 100, Direction::Right) if x >= 100 && x < 150 => (149 - x, 149, Direction::Left), // 4 -> 2
        (150, y, Direction::Down) if y >= 50 && y < 100 => (100 + y, 49, Direction::Left), // 4 -> 6
        (x, -1, Direction::Left) if x >= 100 && x < 150 => (149 - x, 50, Direction::Right), // 5 -> 1
        (99, y, Direction::Up) if y >= 0 && y < 50 => (y + 50, 50, Direction::Right), // 5 -> 3
        (x, -1, Direction::Left) if x >= 150 && x < 200 => (0, x - 100, Direction::Down), // 6 -> 1
        (200, y, Direction::Down) if y >= 0 && y < 50 => (0, y + 100, Direction::Down), // 6 -> 2
        (x, 50, Direction::Right) if x >= 150 && x < 200 => (149, x - 100, Direction::Up), // 6 -> 4
        (x, y, d) => (x, y, *d),
    };

    match map[nx as usize][ny as usize] {
        b'#' => ((nx as usize, ny as usize), direction),
        b'.' => ((nx as usize, ny as usize), nd),
        _ => unreachable!(),
    }
}

#[derive(Debug)]
enum Instruction {
    Move(usize),
    RotateLeft,
    RotateRight,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn rotate_left(self) -> Direction {
        match self {
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
        }
    }

    fn rotate_right(self) -> Direction {
        match self {
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
        }
    }

    fn to_deltas(&self) -> (isize, isize) {
        match self {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
        }
    }

    fn to_score(self) -> usize {
        match self {
            Direction::Left => 2,
            Direction::Right => 0,
            Direction::Up => 3,
            Direction::Down => 1,
        }
    }
}

struct InstructionIterator<I>
where
    I: Iterator<Item = u8>,
{
    iter: I,
    prev: Option<u8>,
}

impl<I> InstructionIterator<I>
where
    I: Iterator<Item = u8>,
{
    fn new(i: I) -> InstructionIterator<I> {
        InstructionIterator {
            iter: i,
            prev: None,
        }
    }
}

impl<I> Iterator for InstructionIterator<I>
where
    I: Iterator<Item = u8>,
{
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        match self.prev.or_else(|| self.iter.next()) {
            Some(d) if d.is_ascii_digit() => {
                let mut v = d - b'0';
                loop {
                    match self.iter.next() {
                        Some(dd) if dd.is_ascii_digit() => {
                            self.prev = None;
                            v = v * 10 + dd - b'0';
                        }
                        Some(v) => {
                            self.prev = Some(v);
                            break;
                        }
                        _ => break,
                    }
                }

                Some(Instruction::Move(v as usize))
            }
            Some(b'L') => {
                self.prev = None;
                Some(Instruction::RotateLeft)
            }
            Some(b'R') => {
                self.prev = None;
                Some(Instruction::RotateRight)
            }
            None => None,
            _ => unreachable!(),
        }
    }
}
