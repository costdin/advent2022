use super::get_input_lines;
use itertools::Itertools;
use std::num::ParseIntError;

pub fn day1() {
    let (solution1, solution2) =
        ElfIterator::new(get_input_lines(1).into_iter().map(|l| l.parse::<i32>()))
            .sorted()
            .rev()
            .take(3)
            .chunks(3)
            .into_iter()
            .map(|mut chunk| (chunk.next(), chunk))
            .map(|(first, others)| (first.iter().sum::<i32>(), others.sum::<i32>()))
            .map(|(first, others)| (first, first + others))
            .fold((0, 0), |acc, v| (acc.0 + v.0, acc.1 + v.1));

    println!(
        "DAY 1\nSolution 1: {}\nSolution 2: {}",
        solution1, solution2
    );
}

struct ElfIterator<I>
where
    I: Iterator<Item = Result<i32, ParseIntError>>,
{
    iter: I,
}

impl<I> ElfIterator<I>
where
    I: Iterator<Item = Result<i32, ParseIntError>>,
{
    fn new(i: I) -> ElfIterator<I> {
        ElfIterator { iter: i }
    }
}

impl<I> Iterator for ElfIterator<I>
where
    I: Iterator<Item = Result<i32, ParseIntError>>,
{
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut total = match self.iter.next() {
            Some(Ok(v)) => v,
            _ => return None,
        };

        loop {
            total += match self.iter.next() {
                Some(Ok(v)) => v,
                _ => break,
            }
        }

        Some(total)
    }
}
