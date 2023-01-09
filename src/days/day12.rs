use std::collections::HashSet;

pub fn day12() {
    let map = include_bytes!("../../day12.txt");
    let start = map.iter().position(|&x| x == b'S').unwrap();
    let end = map.iter().position(|&x| x == b'E').unwrap();

    let result1 = spf(&map, start, b'E', neighborhood);
    let result2 = spf(&map, end, b'a', reverse_neighborhood);

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
    let mut visited = HashSet::from([start_position]);
    let mut frontier = HashSet::from([start_position]);
    for i in 1.. {
        frontier = frontier
            .iter()
            .flat_map(|&ix| neighborhood_fn(map, ix))
            .filter(|v| !visited.contains(v))
            .collect();

        if frontier.iter().any(|&ix| map[ix] == end) {
            return i;
        }

        visited.extend(frontier.iter());
    }

    unreachable!();
}

fn reverse_neighborhood<const L: usize>(map: &[u8; L], ix: usize) -> Vec<usize> {
    let mut n = vec![];
    let c = match map[ix] {
        b'S' => b'a',
        b'E' => b'z',
        c => c,
    };

    if ix % 183 > 0 && map[ix - 1] >= c - 1 {
        n.push(ix - 1);
    }

    if ix % 183 < 180 && map[ix + 1] >= c - 1 {
        n.push(ix + 1)
    }

    if ix > 183 && map[ix - 183] >= c - 1 {
        n.push(ix - 183)
    }

    if ix < L - 184 && map[ix + 183] >= c - 1 {
        n.push(ix + 183)
    }

    n
}

fn neighborhood<const L: usize>(map: &[u8; L], ix: usize) -> Vec<usize> {
    let mut n = vec![];
    let c = match map[ix] {
        b'S' => b'a',
        b'E' => b'z',
        c => c,
    };

    if ix % 183 > 0 && map[ix - 1] <= c + 1 {
        n.push(ix - 1);
    }

    if ix % 183 < 180 && map[ix + 1] <= c + 1 {
        n.push(ix + 1)
    }

    if ix > 183 && map[ix - 183] <= c + 1 {
        n.push(ix - 183)
    }

    if ix < L - 184 && map[ix + 183] <= c + 1 {
        n.push(ix + 183)
    }

    n
}
