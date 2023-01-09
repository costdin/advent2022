use itertools::Itertools;

type OperationFn = Box<dyn Fn(i64) -> i64>;

pub fn day11() {
    let (mut items, monkeys, prod) = include_str!("../../day11.txt")
        .trim()
        .lines()
        .filter(|l| !l.is_empty())
        .tuples()
        .map(|(_, items, operation, test, true_monkey, false_monkey)| {
            (
                items[18..]
                    .split(", ")
                    .map(|i| i.parse::<i64>().unwrap())
                    .collect::<Vec<_>>(),
                match &operation[23..] {
                    "* old" => Box::new(|n| n * n) as OperationFn,
                    s if s.starts_with("*") => {
                        let p = s[2..].parse::<i64>().unwrap();
                        Box::new(move |n| n * p) as OperationFn
                    }
                    s if s.starts_with("+") => {
                        let p = s[2..].parse::<i64>().unwrap();
                        Box::new(move |n| n + p) as OperationFn
                    }
                    _ => unreachable!(),
                },
                test[21..].parse::<i64>().unwrap(),
                true_monkey[29..].parse::<usize>().unwrap(),
                false_monkey[30..].parse::<usize>().unwrap(),
            )
        })
        .fold(
            (vec![], vec![], 1),
            |(mut acc_items, mut acc_monkeys, prod),
             (items, operation, test, true_monkey, false_monkey)| {
                acc_items.push(items);
                acc_monkeys.push((operation, test, true_monkey, false_monkey));

                (acc_items, acc_monkeys, prod * test)
            },
        );

    let mut results = vec![0; monkeys.len()];
    let mut results2 = vec![0; monkeys.len()];
    let mut items2 = items.clone();

    for _ in 0..20 {
        for (ix, (operation, test, true_monkey, false_monkey)) in monkeys.iter().enumerate() {
            results[ix] += items[ix].len();

            while let Some(item) = items[ix].pop() {
                let new_item = operation(item) / 3;
                if new_item % test == 0 {
                    items[*true_monkey].push(new_item);
                } else {
                    items[*false_monkey].push(new_item);
                }
            }
        }
    }

    for _ in 0..10000 {
        for (ix, (operation, test, true_monkey, false_monkey)) in monkeys.iter().enumerate() {
            results2[ix] += items2[ix].len();

            while let Some(item) = items2[ix].pop() {
                let new_item = operation(item) % prod;
                if new_item % test == 0 {
                    items2[*true_monkey].push(new_item);
                } else {
                    items2[*false_monkey].push(new_item);
                }
            }
        }
    }

    let result1 = results
        .iter()
        .sorted()
        .rev()
        .take(2)
        .fold(1, |acc, curr| acc * curr);
    let result2 = results2
        .iter()
        .sorted()
        .rev()
        .take(2)
        .fold(1, |acc, curr| acc * curr);

    println!("DAY 10\nSolution 1: {result1:#?}\nSolution 2: {result2:#?}");
}
