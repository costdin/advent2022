use std::{collections::HashSet, ops::RangeInclusive};

use itertools::Itertools;

const ROW: i64 = 10;
const MAX: i64 = 20;

pub fn day15() {
    let input = include_str!("../../day15.txt")
        .trim()
        .lines()
        .filter_map(|l| l.split_once(":"))
        .map(|(s, b)| (s.split_once(",").unwrap(), b.split_once(",").unwrap()))
        .map(|((sx, sy), (bx, by))| {
            (
                sx[12..].parse::<i64>().unwrap(),
                sy[3..].parse::<i64>().unwrap(),
                bx[24..].parse::<i64>().unwrap(),
                by[3..].parse::<i64>().unwrap(),
            )
        })
        .collect_vec();

    let (ranges, beacons) = get_ranges(&input, ROW);
    let result1 = ranges
        .iter()
        .fold(0, |acc, range| acc + range.end() - range.start() + 1)
        - beacons.len() as i64;

    let result2 = (1..)
        .map(|row| (row, get_ranges(&input, row)))
        .filter(|(_, (range, _))| range.len() > 1 || range[0].start() > &0 || range[0].end() < &MAX)
        .map(|(row, (range, _))| if  range.len() > 1 { (row, range[0].end() + 1) } else if range[0].start() > &0 { (row, 0) } else { (row, MAX) } )
        .map(|(row, x)| row + x * 4000000)
        .take(1)
        .next().unwrap();

    println!("DAY 14\nSolution 1: {result1}\nSolution 2: {result2}");
}

fn get_ranges(
    input: &[(i64, i64, i64, i64)],
    row: i64,
) -> (Vec<RangeInclusive<i64>>, HashSet<i64>) {
    input
        .iter()
        .map(|(sx, sy, bx, by)| {
            (
                sx,
                (sx - bx).abs() + (sy - by).abs(),
                (sy - row).abs(),
                if by == &row { Some(bx) } else { None },
            )
        })
        .filter_map(|(sx, d, yd, bx)| {
            if yd <= d {
                Some((sx, d - yd, bx))
            } else {
                None
            }
        })
        .map(|(sx, range, bx)| ((sx - range)..=(sx + range), bx))
        .sorted_by(|(range, _), (range2, _)| range.start().cmp(range2.start()))
        .fold(
            (Vec::<RangeInclusive<i64>>::new(), HashSet::new()),
            |(mut acc, mut beacons), (range, bx)| {
                if let Some(b) = bx {
                    beacons.insert(*b);
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
