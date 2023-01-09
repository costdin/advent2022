use std::collections::{HashMap, HashSet};

pub fn day9() {
    let map = HashMap::from([("L", [-1, 0]), ("R", [1, 0]), ("U", [0, 1]), ("D", [0, -1])]);

    let res = (0..10).map(|_| HashSet::from([[0, 0]])).collect::<Vec<_>>();

    let result = include_str!("../../day9.txt")
        .trim()
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|l| (map[l.0], l.1.parse::<i32>().unwrap()))
        .flat_map(|(d, s)| (0..s).into_iter().map(move |_| d))
        .fold((res, [[0, 0]; 10]), |(mut acc, mut chain), input| {
            chain[0] = [chain[0][0] + input[0], chain[0][1] + input[1]];

            for i in 1..10 {
                chain[i] = next(chain[i - 1], chain[i]);

                acc[i].insert(chain[i]);
            }

            (acc, chain)
        })
        .0;

    let (result1, result2) = (result[1].len(), result[9].len());

    println!("DAY 9\nSolution 1: {result1:?}\nSolution 2: {result2:?}");
}

fn are_adjacent(p1: [i32; 2], p2: [i32; 2]) -> bool {
    (-1..=1).contains(&(p1[0] - p2[0])) && (-1..=1).contains(&(p1[1] - p2[1]))
}

fn next(head: [i32; 2], tail: [i32; 2]) -> [i32; 2] {
    if are_adjacent(head, tail) {
        tail
    } else {
        [
            tail[0] + (head[0] - tail[0]).signum(),
            tail[1] + (head[1] - tail[1]).signum(),
        ]
    }
}
