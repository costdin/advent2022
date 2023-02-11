use itertools::Itertools;
use std::collections::HashSet;

pub fn day18() {
    let (result1, blocks, (x, y, z)) = include_str!("../../day18.txt")
        .trim()
        .lines()
        .flat_map(|l| l.split(',').map(|n| n.parse::<i32>().unwrap()))
        .tuples()
        .map(|(x, y, z)| (x + 1, y + 1, z + 1)) // translate droplet to avoid negatives in `fold` and `diffuse`
        .fold((0, HashSet::new(), (i32::MIN, i32::MIN, i32::MIN)), fold);

    let (result2, _, _) = diffuse(blocks, (x, y, z)).into_iter().fold(
        (
            -surface(x + 2, y + 2, z + 2), // +2 because we wrapped the droplet in a cuboid
            // that is 1 cube larger that the droplet in all
            // dimensions
            HashSet::new(),
            (i32::MIN, i32::MIN, i32::MIN),
        ),
        fold,
    );

    println!("DAY 18\nSolution 1: {result1}\nSolution 2: {result2}");
}

fn surface(x: i32, y: i32, z: i32) -> i32 {
    2 * ((x * y) + (x * z) + (y * z)) as i32
}

fn fold(
    (count, mut space, (bx, by, bz)): (i32, HashSet<(i32, i32, i32)>, (i32, i32, i32)),
    (x, y, z): (i32, i32, i32),
) -> (i32, HashSet<(i32, i32, i32)>, (i32, i32, i32)) {
    space.insert((x, y, z));

    (
        6 + count
            - 2 * (if space.contains(&(x, y, z + 1)) { 1 } else { 0 }
                + if space.contains(&(x, y, z - 1)) { 1 } else { 0 }
                + if space.contains(&(x, y + 1, z)) { 1 } else { 0 }
                + if space.contains(&(x, y - 1, z)) { 1 } else { 0 }
                + if space.contains(&(x + 1, y, z)) { 1 } else { 0 }
                + if space.contains(&(x - 1, y, z)) { 1 } else { 0 }),
        space,
        (bx.max(x), by.max(y), bz.max(z)),
    )
}

fn diffuse(
    blocks: HashSet<(i32, i32, i32)>,
    (bx, by, bz): (i32, i32, i32),
) -> HashSet<(i32, i32, i32)> {
    let mut frontier = HashSet::from([(0, 0, 0)]);
    let mut result = HashSet::from([(0, 0, 0)]);

    while frontier.len() > 0 {
        frontier = frontier
            .into_iter()
            .flat_map(|(x, y, z)| {
                [
                    (x + 1, y, z),
                    (x - 1, y, z),
                    (x, y + 1, z),
                    (x, y - 1, z),
                    (x, y, z + 1),
                    (x, y, z - 1),
                ]
            })
            .filter(|(x, y, z)| {
                x >= &0 && y >= &0 && z >= &0 && x <= &(bx + 1) && y <= &(by + 1) && z <= &(bz + 1)
            })
            .filter(|p| !blocks.contains(p))
            .filter(|p| !result.contains(p))
            .collect();

        result.extend(frontier.clone());
    }

    result
}
