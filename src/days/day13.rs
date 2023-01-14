use itertools::Itertools;
use std::cmp::Ordering;

pub fn day13() {
    let result1 = include_str!("../../day13.txt")
        .trim()
        .lines()
        .filter(|l| !l.is_empty())
        .map(parse)
        .tuples()
        .zip(1..)
        .filter_map(|((a, b), ix)| if a < b { Some(ix) } else { None })
        .sum::<i32>();

    let new_elements = ["[[2]]", "[[6]]"]
        .into_iter()
        .map(parse)
        .collect::<Vec<_>>();

    let result2 = include_str!("../../day13.txt")
        .trim()
        .lines()
        .chain(["[[2]]", "[[6]]"])
        .filter(|l| !l.is_empty())
        .map(parse)
        .sorted()
        .zip(1..)
        .filter_map(|(a, ix)| {
            if new_elements.contains(&a) {
                Some(ix)
            } else {
                None
            }
        })
        .fold(1, |acc, curr| acc * curr);

    println!("DAY 12\nSolution 1: {result1}\nSolution 2: {result2}");
}

fn parse(line: &str) -> Packet {
    process_packet(&mut Tokenizer::new(line).skip(1).filter(|c| c != &","))
}

fn process_packet<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Packet {
    let mut result = vec![];

    while let Some(t) = tokens.next() {
        let p = match t {
            "[" => process_packet(tokens),
            "]" => break,
            n => Packet::Number(n.parse().unwrap()),
        };

        result.push(p);
    }

    Packet::List(result)
}

#[derive(PartialEq, Eq, Ord)]
enum Packet {
    Number(i32),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Number(n1), Packet::Number(n2)) => {
                if n1 < n2 {
                    Some(Ordering::Less)
                } else if n1 > n2 {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Equal)
                }
            }
            (Packet::Number(n), p @ Packet::List(_)) => {
                Packet::List(vec![Packet::Number(*n)]).partial_cmp(&p)
            }
            (p @ Packet::List(_), Packet::Number(n)) => {
                p.partial_cmp(&Packet::List(vec![Packet::Number(*n)]))
            }
            (Packet::List(l1), Packet::List(l2)) => {
                match (
                    l1.len(),
                    l2.len(),
                    l1.iter().zip(l2.iter()).try_fold(None, |_, (a, b)| {
                        let r = a.partial_cmp(&b);
                        if r != Some(Ordering::Equal) {
                            Err(r)
                        } else {
                            Ok(r)
                        }
                    }),
                ) {
                    (len1, len2, Ok(_)) => {
                        Packet::Number(len1 as i32).partial_cmp(&Packet::Number(len2 as i32))
                    }
                    (_, _, Err(r)) => r,
                }
            }
        }
    }
}

pub struct Tokenizer<'a> {
    completed: bool,
    slice: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn new(string: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            completed: false,
            slice: &string,
        }
    }
}

static TOKEN_BREAKER: &'static [char] = &['[', ']', ','];

impl<'a> Iterator for Tokenizer<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.completed {
            return None;
        }

        let mut iterator = self.slice.char_indices();
        let start = 0usize;
        loop {
            match iterator.next() {
                Some((pos, c)) if TOKEN_BREAKER.contains(&c) => {
                    if pos > start {
                        let result = &self.slice[start..pos];
                        self.slice = &self.slice[pos..];
                        return Some(result);
                    } else {
                        let result = &self.slice[start..pos + 1];
                        self.slice = &self.slice[pos + 1..];
                        return Some(result);
                    }
                }
                Some((_, _)) => {}
                None => {
                    self.completed = true;
                    return None;
                }
            }
        }
    }
}
