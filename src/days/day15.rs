use std::{collections::HashSet, ops::RangeInclusive};

use itertools::Itertools;

const ROW: i64 = 2000000;
const MAX: i64 = 4000000;

pub fn day15() {
    let input = include_str!("../../day15.txt")
        .trim()
        .lines()
        .filter_map(|l| l.split_once(":"))
        .map(|(s, b)| (s.split_once(",").unwrap(), b.split_once(",").unwrap()))
        .map(|((sx, sy), (bx, by))| {
            (
                (
                    sx[12..].parse::<i64>().unwrap(),
                    sy[3..].parse::<i64>().unwrap(),
                ),
                (
                    bx[24..].parse::<i64>().unwrap(),
                    by[3..].parse::<i64>().unwrap(),
                ),
            )
        })
        .map(|(s, b)| (s, b, distance(&s, &b)))
        .collect_vec();

    let (ranges, beacons) = get_ranges(&input, ROW);
    let result1 = ranges
        .iter()
        .fold(0, |acc, range| acc + range.end() - range.start() + 1)
        - beacons.len() as i64;

    let result2 = input
        .iter()
        .enumerate()
        .flat_map(|(ix, s)| input.iter().skip(ix + 1).map(move |s2| (s, s2)))
        .flat_map(|(sensor1, sensor2)| intersection(sensor1, sensor2))
        .filter(|(x, y)| x >= &0 && x <= &MAX && y >= &0 && y <= &MAX)
        .filter(|point| !input.iter().any(|sensor| covers(sensor, point)))
        .take(1)
        .map(|(x, y)| x * 4000000 + y)
        .next()
        .unwrap();

    println!("DAY 15\nSolution 1: {result1}\nSolution 2: {result2}");
}

fn intersection(
    &(s0, _, d0): &((i64, i64), (i64, i64), i64),
    &(s1, _, d1): &((i64, i64), (i64, i64), i64),
) -> impl Iterator<Item = (i64, i64)> {
    fn inters(
        d0: i64,
        d1: i64,
        (x0, y0): (i64, i64),
        (x1, y1): (i64, i64),
    ) -> impl Iterator<Item = (i64, i64)> {
        [
            (d0 + d1 + x0 + x1 + y0 - y1, d0 - d1 + x0 - x1 + y0 + y1),
            (d0 - d1 + x0 + x1 + y0 - y1, d0 + d1 + x0 - x1 + y0 + y1),
            (-d0 + d1 + x0 + x1 + y0 - y1, -d0 - d1 + x0 - x1 + y0 + y1),
            (-d0 - d1 + x0 + x1 + y0 - y1, -d0 + d1 + x0 - x1 + y0 + y1),
        ]
        .into_iter()
        .enumerate()
        .filter(|(_, (x, y))| x % 2 == 0 && y % 2 == 0)
        .map(|(ix, (x, y))| (ix, (x / 2, y / 2)))
        .filter(move |(ix, (x, y))| match ix {
            0 => x >= &x0 && y >= &y0 && x >= &x1 && y < &y1,
            1 => x >= &x0 && y >= &y0 && x < &x1 && y >= &y1,
            2 => x < &x0 && y < &y0 && x >= &x1 && y < &y1,
            3 => x < &x0 && y < &y0 && x < &x1 && y >= &y1,
            _ => unreachable!(),
        })
        .map(|(_, i)| i)
    }

    inters(d0 + 1, d1 + 1, s0, s1).chain(inters(d1 + 1, d0 + 1, s1, s0))
}

fn covers((sensor, _, d): &((i64, i64), (i64, i64), i64), point: &(i64, i64)) -> bool {
    d >= &distance(sensor, point)
}

fn distance(point1: &(i64, i64), point2: &(i64, i64)) -> i64 {
    (point1.0 - point2.0).abs() + (point1.1 - point2.1).abs()
}

fn get_ranges(
    input: &[((i64, i64), (i64, i64), i64)],
    row: i64,
) -> (Vec<RangeInclusive<i64>>, HashSet<i64>) {
    input
        .iter()
        .map(|((x, y), b, d)| (x, (y - row).abs(), b, d))
        .filter(|(_, yd, _, d)| yd <= *d)
        .map(|(x, yd, b, d)| (x, d - yd, b))
        .map(|(sx, range, bx)| ((sx - range)..=(sx + range), bx))
        .sorted_by(|(range, _), (range2, _)| range.start().cmp(range2.start()))
        .fold(
            (Vec::<RangeInclusive<i64>>::new(), HashSet::new()),
            |(mut acc, mut beacons), (range, b)| {
                if b.1 == row {
                    beacons.insert(b.0);
                };

                match acc.pop() {
                    Some(r) if range.start() <= r.end() => {
                        acc.push(*r.start()..=*range.end().max(r.end()))
                    }
                    Some(r) => acc.extend_from_slice(&[r, range]),
                    None => acc.push(range),
                };

                (acc, beacons)
            },
        )
}
