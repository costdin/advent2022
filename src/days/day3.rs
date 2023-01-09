use std::collections::HashSet;

macro_rules! to_score {
    () => {
        |c| {
            if c >= &'a' {
                *c as u32 - 96
            } else {
                *c as u32 - 38
            }
        }
    };
}

pub fn day3(input: Vec<Vec<char>>) {
    let result1 = input
        .iter()
        .map(|l| (l.len() / 2, l))
        .map(|(half_len, l)| {
            (
                l[0..half_len].iter().collect::<HashSet<_>>(),
                &l[half_len..],
            )
        })
        .filter_map(|(first, second)| second.iter().find(|c| first.contains(c)))
        .map(to_score!())
        .sum::<u32>();

    let result2 = input
        .chunks(3)
        .map(|chunks| {
            (
                &chunks[0],
                chunks[1..].iter().map(|a| a.iter().collect::<HashSet<_>>()),
            )
        })
        .map(|(first, others)| {
            others.fold::<Box<dyn Iterator<Item = _>>, _>(Box::new(first.iter()), |acc, set| {
                Box::new(acc.filter(move |c| set.contains(c)))
            })
        })
        .filter_map(|c| c.into_iter().next())
        .map(to_score!())
        .sum::<u32>();

    println!("DAY 3\nSolution 1: {result1:?}\nSolution 2: {result2:?}");
}
