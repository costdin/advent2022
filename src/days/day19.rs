use itertools::Itertools;
use std::collections::HashSet;
use rayon::prelude::*;

pub fn day19() {
    let result1 = include_str!("../../day19.txt")
        .trim()
        .lines()
        .map(|l| l.split(' ').collect::<Vec<_>>())
        .map(|v| {
            (
                v[6].parse::<i32>().unwrap(),
                v[12].parse::<i32>().unwrap(),
                v[18].parse::<i32>().unwrap(),
                v[21].parse::<i32>().unwrap(),
                v[27].parse::<i32>().unwrap(),
                v[30].parse::<i32>().unwrap(),
            )
        })
        .zip(1..)
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|(blueprint, ix)| ix * xxxx(blueprint, (1, 0, 0, 0, 0, 0, 0, 0), 1))
        .sum::<i32>();

    println!("DAY 18\nSolution 1: {result1:?}\nSolution 2: {result1:?}");
}

fn xxxx(
    blueprint: (i32, i32, i32, i32, i32, i32),
    state: (i32, i32, i32, i32, i32, i32, i32, i32),
    minute: i32,
) -> i32 {

    if minute == 24 {
        state.7 + state.3
    } else {
        let mut r = 0;

        if state.4 >= blueprint.4 && state.6 >= blueprint.5 {
            return xxxx(
                blueprint,
                (
                    state.0,
                    state.1,
                    state.2,
                    state.3 + 1,
                    state.4 + state.0 - blueprint.4,
                    state.5 + state.1,
                    state.6 + state.2 - blueprint.5,
                    state.7 + state.3,
                ),
                minute + 1,
            );
        }

        if state.4 >= blueprint.2 && state.5 >= blueprint.3 {
            r = r.max(xxxx(
                blueprint,
                (
                    state.0,
                    state.1,
                    state.2 + 1,
                    state.3,
                    state.4 + state.0 - blueprint.2,
                    state.5 + state.1 - blueprint.3,
                    state.6 + state.2,
                    state.7 + state.3,
                ),
                minute + 1,
            ));
        }

        if state.4 >= blueprint.1 {
            r = r.max(xxxx(
                blueprint,
                (
                    state.0,
                    state.1 + 1,
                    state.2,
                    state.3,
                    state.4 + state.0 - blueprint.1,
                    state.5 + state.1,
                    state.6 + state.2,
                    state.7 + state.3,
                ),
                minute + 1,
            ));
        }

        if state.4 >= blueprint.0 {
            r = r.max(xxxx(
                blueprint,
                (
                    state.0 + 1,
                    state.1,
                    state.2,
                    state.3,
                    state.4 + state.0 - blueprint.0,
                    state.5 + state.1,
                    state.6 + state.2,
                    state.7 + state.3,
                ),
                minute + 1,
            ));
        }

        r = r.max(xxxx(
            blueprint,
            (
                state.0,
                state.1,
                state.2,
                state.3,
                state.4 + state.0,
                state.5 + state.1,
                state.6 + state.2,
                state.7 + state.3,
            ),
            minute + 1,
        ));

        r
    }
}

