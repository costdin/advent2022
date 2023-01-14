use itertools::Itertools;

pub fn day14() {
    let (map, max_y) = include_str!("../../day14.txt")
        .trim()
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|i| i.split_once(",").unwrap())
                .map(|i| (i.0.parse::<usize>().unwrap(), i.1.parse::<usize>().unwrap()))
        })
        .fold((vec![false; 200 * 1000], 0), |(v, max_y), r| {
            r.tuple_windows()
                .fold((v, max_y), |(mut acc, max_y), (c1, c2)| {
                    to_iter(c1, c2).for_each(|c| acc[to_index(c)] = true);
                    (acc, max_y.max(c1.1).max(c2.1))
                })
        });

    let (result1, result2) = match (0..).try_fold((map, None), |(mut acc, r1), count| {
        match (0..).try_fold((500, 0), |iacc, _| match next_move(&acc, iacc) {
            Some(p) if p.1 == max_y + 1 => Err(p),
            Some(p) if p == (500, 0) => Err(p),
            Some(p) => Ok(p),
            None => Err(iacc),
        }) {
            Ok(_) => unreachable!(),
            Err(p) if p == (500, 0) => Err((r1.unwrap(), count)),
            Err(p) if r1.is_none() && p.1 == max_y + 1 => Ok((acc, Some(count))),
            Err(p) => {
                acc[to_index(p)] = true;
                Ok((acc, r1))
            }
        }
    }) {
        Ok(_) => unreachable!(),
        Err(p) => p,
    };

    println!("DAY 14\nSolution 1: {result1}\nSolution 2: {result2}");
}

fn to_index((x, y): (usize, usize)) -> usize {
    x + y * 1000
}

fn to_iter(
    (s1, s2): (usize, usize),
    (e1, e2): (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    (s1.min(e1)..=s1.max(e1)).flat_map(move |x| (s2.min(e2)..=s2.max(e2)).map(move |y| (x, y)))
}

fn next_move(map: &Vec<bool>, (x, y): (usize, usize)) -> Option<(usize, usize)> {
    if !map[to_index((x, y + 1))] {
        Some((x, y + 1))
    } else if !map[to_index((x - 1, y + 1))] {
        Some((x - 1, y + 1))
    } else if !map[to_index((x + 1, y + 1))] {
        Some((x + 1, y + 1))
    } else {
        None
    }
}
