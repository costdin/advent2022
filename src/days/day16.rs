use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn day16() {
    let (press, all) = include_str!("../../day16.txt")
        .trim()
        .lines()
        .map(|l| l.split_once('=').unwrap())
        .map(|(s1, s2)| (&s1[6..8], s2.split_once(';').unwrap()))
        .map(|(id, (s1, s2))| {
            (
                id,
                s1.parse::<u32>().unwrap(),
                s2[23..].trim().split(", ").collect_vec(),
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

    let map = ["AA"]
        .iter()
        .chain(press.iter())
        .map(|p| (*p, spf(p, &press, &all)))
        .collect::<HashMap<_, _>>();

    let result1 = gaga(&press, 0, &map["AA"], &map);
    let result2 = gaga2(&press, 0, 0, &map["AA"], &map["AA"], &map);

    println!("DAY 16\nSolution 1: {result1}\nSolution 2: {result2}");
}

fn gaga<'a>(
    nodes: &'a Vec<&'a str>,
    distance: u32,
    next: &'a HashMap<&str, (u32, u32)>,
    map: &'a HashMap<&str, HashMap<&str, (u32, u32)>>,
) -> u32 {
    let mut n = nodes.clone();
    let mut result = 0;

    for i in 0..n.len() {
        let e = n.remove(i);

        let new_distance = distance + next[e].0 + 1;
        if new_distance < 30 {
            let max = gaga(&n, new_distance, &map[e], map);
            result = result.max(max + (30 - new_distance) * next[e].1);
        }

        n.insert(i, e);
    }

    result
}

fn gaga2<'a>(
    nodes: &'a Vec<&'a str>,
    distance: u32,
    distance2: u32,
    next: &'a HashMap<&str, (u32, u32)>,
    next2: &'a HashMap<&str, (u32, u32)>,
    map: &'a HashMap<&str, HashMap<&str, (u32, u32)>>,
) -> u32 {
    let mut n = nodes.clone();
    let mut result = 0;

    for i in 0..n.len() {
        let e = n.remove(i);

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

        n.insert(i, e);
    }

    result
}

fn spf<'a>(
    start: &'a str,
    destinations: &[&str],
    table: &'a HashMap<&'a str, (u32, Vec<&'a str>)>,
) -> HashMap<&'a str, (u32, u32)> {
    let mut res = HashMap::new();
    let mut frontier = vec![start];
    let mut ix = 0;
    let root = !destinations.contains(&start);

    while (root && res.len() < destinations.len()) || (res.len() < destinations.len() - 1) {
        let mut new_frontier = HashSet::new();

        while let Some(e) = frontier.pop() {
            if !res.contains_key(e) && destinations.contains(&e) && e != start {
                res.insert(e, (ix, table[e].0));
            }

            new_frontier.extend(&table[e].1);
        }

        frontier = new_frontier.into_iter().collect();
        ix += 1;
    }

    res
}
