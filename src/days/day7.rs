use std::collections::HashMap;

const DISK_SIZE: u32 = 70000000;
const SPACE_NEEDED: u32 = 30000000;

pub fn day7() {
    let (_, folders, space_left) = include_str!("../../day7.txt")
        .lines()
        .skip(1)
        .filter(|l| !l.starts_with("dir") && !l.starts_with("$ ls"))
        .fold(
            (Vec::<String>::new(), HashMap::new(), DISK_SIZE),
            |(mut stack, mut directories, space_left), line| match line {
                "$ cd .." => {
                    stack.pop();
                    (stack, directories, space_left)
                }
                l if l.starts_with("$ cd ") => {
                    stack.push(l[5..].to_string() + stack.last().unwrap_or(&"".to_string()));
                    (stack, directories, space_left)
                }
                l if l.chars().next().unwrap().is_numeric() => {
                    let dim = l.split_once(' ').unwrap().0.parse::<u32>().unwrap();
                    stack
                        .iter()
                        .for_each(|d| *directories.entry(d.to_string()).or_default() += dim);
                    (stack, directories, space_left - dim)
                }
                _ => unreachable!(),
            },
        );

    let (result1, result2) =
        folders
            .into_iter()
            .fold((0, u32::MAX), |(sum_dir, large), (_, curr)| {
                match (
                    curr < 100000,
                    curr < large,
                    curr + space_left >= SPACE_NEEDED,
                ) {
                    (true, _, _) => (sum_dir + curr, large),
                    (_, true, true) => (sum_dir, curr),
                    _ => (sum_dir, large),
                }
            });

    println!("DAY 7\nSolution 1: {result1:?}\nSolution 2: {result2:?}");
}
