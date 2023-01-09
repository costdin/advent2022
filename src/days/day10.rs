pub fn day10() {
    let (_, result1, result2) = include_str!("../../day10.txt")
        .trim()
        .lines()
        .flat_map(|l| match l {
            "noop" => vec![0],
            l if l.starts_with("addx ") => vec![0, l[5..].parse::<i32>().unwrap()],
            _ => unreachable!(),
        })
        .zip(0..)
        .fold((1, 0, String::new()), |(count, res, s), (curr, ix)| {
            (
                count + curr,
                if (ix + 21) % 40 == 0 {
                    res + count * (ix + 1)
                } else {
                    res
                },
                s + if ((count - 1)..=(count + 1)).contains(&(ix % 40)) {
                    "#"
                } else {
                    "."
                } + if ix % 40 == 39 { "\n" } else { "" },
            )
        });

    println!("DAY 10\nSolution 1: {result1}\nSolution 2: \n{result2}");
}
