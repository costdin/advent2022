use std::{
    collections::{HashMap, HashSet},
    ops::Index,
};

use itertools::Itertools;

fn to_id(s: &str) -> u16 {
    s.chars()
        .map(|c| c as u16 - 'A' as u16)
        .fold(0, |acc, c| (acc << 8) + c)
}

pub fn day16() {
    let (mut press, all) = include_str!("../../day16.txt")
        .trim()
        .lines()
        .map(|l| l.split_once('=').unwrap())
        .map(|(s1, s2)| (&s1[6..8], s2.split_once(';').unwrap()))
        .map(|(id, (s1, s2))| {
            (
                to_id(id),
                s1.parse::<u32>().unwrap(),
                s2[23..].trim().split(", ").map(to_id).collect_vec(),
            )
        })
        .fold(
            (vec![], HashMap::new()),
            |(mut press, mut all), (id, pressure, conn)| {
                if pressure != 0 {
                    press.push(id);
                }
                all.insert(id, (pressure, conn));

                (press, all)
            },
        );

    let map = [0]
        .iter()
        .chain(press.iter())
        .map(|p| (*p, spf(*p, &press, &all)))
        .collect::<HashMap<_, _>>();

    let (first_step, result1) = gaga(&press, 0, &map[&0], &map);
    let xx = &map[&first_step];
    let yy = &map[&0][&first_step];

    press.retain(|v| v != &first_step);
    let result2 = gaga2(&press, yy.0 + 1, 0, xx, &map[&0], &map) + (25 - yy.0) * yy.1;

    println!("DAY 16\nSolution 1: {result1}\nSolution 2: {result2}");
}

fn gaga<'a>(
    nodes: &'a Vec<u16>,
    distance: u32,
    next: &'a HashMap<u16, (u32, u32)>,
    map: &'a HashMap<u16, HashMap<u16, (u32, u32)>>,
) -> (u16, u32) {
    let mut n = nodes.clone();
    let mut result = (0, 0);

    for i in 0..n.len() {
        let e = n.remove(i);

        let new_distance = distance + next[&e].0 + 1;
        if new_distance < 30 {
            let max = gaga(&n, new_distance, &map[&e], map).1 + (30 - new_distance) * next[&e].1;

            if max > result.1 {
                result = (e, result.1.max(max));
            }
        }

        n.insert(i, e);
    }

    result
}

fn gaga2<'a>(
    nodes: &'a Vec<u16>,
    distance: u32,
    distance2: u32,
    next: &'a HashMap<u16, (u32, u32)>,
    next2: &'a HashMap<u16, (u32, u32)>,
    map: &'a HashMap<u16, HashMap<u16, (u32, u32)>>,
) -> u32 {
    let mut n = nodes.clone();
    let mut result = 0;

    for i in 0..n.len() {
        let e = &n.remove(i);

        let (new_distance, n1, new_distance2, n2, nd) = if distance <= distance2 {
            (
                distance + next[e].0 + 1,
                &map[e],
                distance2,
                next2,
                distance + next[e].0 + 1,
            )
        } else {
            (
                distance,
                next,
                distance2 + next2[e].0 + 1,
                &map[e],
                distance2 + next2[e].0 + 1,
            )
        };

        if new_distance < 26 && new_distance2 < 26 {
            let max = gaga2(&n, new_distance, new_distance2, n1, n2, map);

            result = result.max(max + (26 - nd) * next[e].1);
        }

        n.insert(i, *e);
    }

    result
}

fn spf<'a>(
    start: u16,
    destinations: &[u16],
    table: &'a HashMap<u16, (u32, Vec<u16>)>,
) -> HashMap<u16, (u32, u32)> {
    let mut res = HashMap::new();
    let mut frontier = vec![start];
    let mut ix = 0;
    let root = !destinations.contains(&start);

    while (root && res.len() < destinations.len()) || (res.len() < destinations.len() - 1) {
        let mut new_frontier = HashSet::new();

        while let Some(e) = frontier.pop() {
            if !res.contains_key(&e) && destinations.contains(&e) && e != start {
                res.insert(e, (ix, table[&e].0));
            }

            new_frontier.extend(&table[&e].1);
        }

        frontier = new_frontier.into_iter().collect();
        ix += 1;
    }

    res
}
