use std::{collections::HashMap, ops};

pub fn day21() {
    let hs = include_str!("../../day21.txt")
        .trim()
        .lines()
        .map(|l| l.split_once(':').unwrap())
        .map(|(name, m)| (name.trim(), Monkey::parse(name, m.trim())))
        .collect::<HashMap<_, _>>();

    let result1 = calculate(&hs, "root");
    let result2 = part2(&hs, "root");

    println!("DAY 21\nSolution 1: {result1}\nSolution 2: {result2}\n");
}

fn calculate(hashmap: &HashMap<&str, Monkey>, name: &str) -> i128 {
    match &hashmap[name] {
        Monkey::Number(n) | Monkey::Human(n) => *n,
        Monkey::Add(m1, m2) => calculate(hashmap, &m1) + calculate(hashmap, &m2),
        Monkey::Sub(m1, m2) => calculate(hashmap, &m1) - calculate(hashmap, &m2),
        Monkey::Mul(m1, m2) => calculate(hashmap, &m1) * calculate(hashmap, &m2),
        Monkey::Div(m1, m2) => calculate(hashmap, &m1) / calculate(hashmap, &m2),
    }
}

fn part2(hashmap: &HashMap<&str, Monkey>, name: &str) -> i128 {
    match &hashmap[name] {
        Monkey::Add(m1, m2) | Monkey::Sub(m1, m2) | Monkey::Mul(m1, m2) | Monkey::Div(m1, m2) => {
            match (to_computable(hashmap, &m1), to_computable(hashmap, &m2)) {
                (Computable::Computed(r), other) | (other, Computable::Computed(r)) => {
                    other.solve(r)
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

fn to_computable(hashmap: &HashMap<&str, Monkey>, name: &str) -> Computable {
    match &hashmap[name] {
        Monkey::Number(n) => Computable::Computed(*n),
        Monkey::Human(_) => Computable::Unknown,
        Monkey::Add(m1, m2) => to_computable(hashmap, &m1) + to_computable(hashmap, &m2),
        Monkey::Sub(m1, m2) => to_computable(hashmap, &m1) - to_computable(hashmap, &m2),
        Monkey::Mul(m1, m2) => to_computable(hashmap, &m1) * to_computable(hashmap, &m2),
        Monkey::Div(m1, m2) => to_computable(hashmap, &m1) / to_computable(hashmap, &m2),
    }
}

#[derive(Debug)]
enum Computable {
    Computed(i128),
    Unknown,
    Sum(Box<Computable>, Box<Computable>),
    Mul(Box<Computable>, Box<Computable>),
    Div(Box<Computable>, Box<Computable>),
    Sub(Box<Computable>, Box<Computable>),
}

impl Computable {
    fn solve(self, result: i128) -> i128 {
        match self {
            Computable::Sum(l, r) => {
                if let Computable::Computed(v) = *l {
                    r.solve(result - v)
                } else if let Computable::Computed(v) = *r {
                    l.solve(result - v)
                } else {
                    unreachable!()
                }
            }
            Computable::Mul(l, r) => {
                if let Computable::Computed(v) = *l {
                    r.solve(result / v)
                } else if let Computable::Computed(v) = *r {
                    l.solve(result / v)
                } else {
                    unreachable!()
                }
            }
            Computable::Sub(l, r) => {
                if let Computable::Computed(v) = *l {
                    r.solve(v - result)
                } else if let Computable::Computed(v) = *r {
                    l.solve(result + v)
                } else {
                    unreachable!()
                }
            }
            Computable::Div(l, r) => {
                if let Computable::Computed(v) = *l {
                    r.solve(v / result)
                } else if let Computable::Computed(v) = *r {
                    l.solve(v * result)
                } else {
                    unreachable!()
                }
            }
            Self::Unknown => result,
            _ => unreachable!(),
        }
    }
}

impl ops::Add<Computable> for Computable {
    type Output = Computable;

    fn add(self, rhs: Computable) -> Computable {
        match (self, rhs) {
            (Computable::Computed(n), Computable::Computed(m)) => Computable::Computed(n + m),
            (n, m) => Computable::Sum(n.into(), m.into()),
        }
    }
}

impl ops::Sub<Computable> for Computable {
    type Output = Computable;

    fn sub(self, rhs: Computable) -> Computable {
        match (self, rhs) {
            (Computable::Computed(n), Computable::Computed(m)) => Computable::Computed(n - m),
            (n, m) => Computable::Sub(n.into(), m.into()),
        }
    }
}

impl ops::Mul<Computable> for Computable {
    type Output = Computable;

    fn mul(self, rhs: Computable) -> Computable {
        match (self, rhs) {
            (Computable::Computed(n), Computable::Computed(m)) => Computable::Computed(n * m),
            (n, m) => Computable::Mul(n.into(), m.into()),
        }
    }
}

impl ops::Div<Computable> for Computable {
    type Output = Computable;

    fn div(self, rhs: Computable) -> Computable {
        match (self, rhs) {
            (Computable::Computed(n), Computable::Computed(m)) => Computable::Computed(n / m),
            (n, m) => Computable::Div(n.into(), m.into()),
        }
    }
}

enum Monkey {
    Number(i128),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    Human(i128),
}

impl Monkey {
    pub fn parse(name: &str, s: &str) -> Monkey {
        match (name, s.parse::<i128>()) {
            ("humn", Ok(n)) => Monkey::Human(n),
            ("humn", _) => unreachable!(),
            (_, Ok(n)) => Monkey::Number(n),
            (_, Err(_)) => {
                let op = s.split(' ').collect::<Vec<_>>();
                match op[1] {
                    "+" => Monkey::Add(op[0].to_string(), op[2].to_string()),
                    "-" => Monkey::Sub(op[0].to_string(), op[2].to_string()),
                    "*" => Monkey::Mul(op[0].to_string(), op[2].to_string()),
                    "/" => Monkey::Div(op[0].to_string(), op[2].to_string()),
                    _ => unreachable!(),
                }
            }
        }
    }
}
