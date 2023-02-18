use itertools::Itertools;

pub fn day20() {
    let input = include_str!("../../day20.txt")
        .trim()
        .lines()
        .filter_map(|l| l.parse::<i64>().ok())
        .enumerate()
        .collect_vec();

    let result1 = mix(input.clone());
    let pos = result1.iter().position(|e| e.1 == 0).unwrap();
    let result1 = (1..=3)
        .map(|i| pos + i * 1000)
        .map(|i| result1[i % result1.len()].1)
        .sum::<i64>();

    let result2 = (0..10).fold(
        input
            .into_iter()
            .map(|(i, v)| (i, v * 811589153))
            .collect::<Vec<_>>(),
        |acc, _| mix(acc),
    );
    let pos = result2.iter().position(|e| e.1 == 0).unwrap();
    let result2 = (1..=3)
        .map(|i| pos + i * 1000)
        .map(|i| result2[i % result2.len()].1)
        .sum::<i64>();

    println!("DAY 20\nSolution 1: {result1:?}\nSolution 2: {result2:?}");
}

fn mix(mut list: Vec<(usize, i64)>) -> Vec<(usize, i64)> {
    let mut i = 0;
    let mut curr = 0;
    while curr < list.len() {
        while list[i].0 == curr {
            let e = list[i];
            if e.1 != 0 {
                let ix = (i as i64 + e.1).rem_euclid(list.len() as i64 - 1) as usize;
                if ix > i {
                    list.copy_within((i + 1)..=ix, i);
                } else if ix < i {
                    list.copy_within((ix)..i, ix + 1);
                }

                list[ix] = e;
            }

            curr += 1;
        }

        i = (i + 1) % list.len();
    }

    list
}
