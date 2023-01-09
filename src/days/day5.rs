pub fn day5() {
    let header = include_str!("../../day5.txt")
        .lines()
        .take_while(|l| !l.chars().any(|c| c.is_numeric()))
        .map(|s| {
            s.chars().enumerate().filter_map(|(ix, v)| {
                if (ix + 3) % 4 == 0 && v.is_alphabetic() {
                    Some(((ix - 1) / 4, v))
                } else {
                    None
                }
            })
        })
        .fold(vec![Vec::new(); 9], |mut acc, curr| {
            curr.for_each(|(ix, v)| acc[ix].insert(0, v));
            acc
        });

    let results = include_str!("../../day5.txt")
        .lines()
        .skip_while(|l| !l.starts_with("move"))
        .map(|l| {
            l.split(' ')
                .enumerate()
                .filter(|(ix, _)| [1, 3, 5].contains(ix))
                .filter_map(|(_, v)| v.parse::<usize>().ok())
        })
        .map(|mut iter| {
            (
                iter.next().unwrap(),
                iter.next().unwrap() - 1,
                iter.next().unwrap() - 1,
            )
        })
        .fold(
            [header.clone(), header],
            |[mut acc, mut acc_straight], (count, source, dest)| {
                let left = acc_straight[source].len() - count;

                let straight = acc_straight[source][left..].to_vec(); //iter().collect::<Vec<_>>();
                let rev = acc[source][left..]
                    .to_vec()
                    .into_iter()
                    .rev()
                    .collect::<Vec<_>>();

                acc[dest].extend(rev);
                acc[source].truncate(left);
                acc_straight[dest].extend(straight);
                acc_straight[source].truncate(left);

                [acc, acc_straight]
            },
        )
        .into_iter()
        .map(|r| r.iter().filter_map(|s| s.last()).collect::<String>())
        .collect::<Vec<_>>();

    println!(
        "DAY 5\nSolution 1: {}\nSolution 2: {}",
        results[0], results[1]
    );
}
