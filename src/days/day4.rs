pub fn day4() {
    let (result1, result2) = include_str!("../../day4.txt")
        .trim()
        .lines()
        .map(|l| {
            l.split(',')
                .flat_map(|r| r.split('-').map(|s| s.parse::<u32>().unwrap()))
        })
        .map(|mut l| {
            (
                l.next().unwrap(),
                l.next().unwrap(),
                l.next().unwrap(),
                l.next().unwrap(),
            )
        })
        .fold((0, 0), |(full, partial), (l1, h1, l2, h2)| {
            if (l1 >= l2 && h1 <= h2) || (l2 >= l1 && h2 <= h1) {
                (full + 1, partial + 1)
            } else if h1 >= l2 && l1 <= l2 || h2 >= l1 && l2 <= l1 {
                (full, partial + 1)
            } else {
                (full, partial)
            }
        });

    println!("DAY 4\nSolution 1: {result1:?}\nSolution 2: {result2:?}");
}
