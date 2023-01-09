use std::collections::HashSet;

pub fn day6() {
    let list = include_str!("../../day6.txt").chars().collect::<Vec<_>>();
    let result1 = &list
        .windows(4)
        .position(|x| x.iter().collect::<HashSet<_>>().len() == 4)
        .unwrap()
        + 4;

    let result2 = &list
        .windows(14)
        .skip(result1)
        .position(|x| x.iter().collect::<HashSet<_>>().len() == 14)
        .unwrap()
        + 14
        + result1;

    println!("DAY 6\nSolution 1: {result1:?}\nSolution 2: {result2:?}");
}
