use rayon::prelude::*;

pub fn day19() {
    let result1 = include_str!("../../day19.txt")
        .trim()
        .lines()
        .map(|l| l.split(' ').collect::<Vec<_>>())
        .map(|v| {
            (
                v[6].parse::<u64>().unwrap(),
                v[12].parse::<u64>().unwrap(),
                v[18].parse::<u64>().unwrap(),
                v[21].parse::<u64>().unwrap(),
                v[27].parse::<u64>().unwrap(),
                v[30].parse::<u64>().unwrap(),
            )
        })
        .map(
            |(
                ore_ore_cost,
                clay_ore_cost,
                obs_ore_cost,
                obs_clay_cost,
                geode_ore_cost,
                geode_obs_cost,
            )| {
                [
                    (1 << 48, ore_ore_cost << 48),
                    (1 << 32, clay_ore_cost << 48),
                    (1 << 16, obs_ore_cost << 48 | obs_clay_cost << 32),
                    (1, geode_ore_cost << 48 | geode_obs_cost << 16),
                ]
            },
        )
        .zip(1..)
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|(blueprint, ix)| ix * xxxx(blueprint, (1 << 48, 0), 1))
        .sum::<i32>();

    println!("DAY 18\nSolution 1: {result1:?}\nSolution 2: {result1:?}");
}

fn xxxx(blueprint: [(u64, u64); 4], state: (u64, u64), minute: i32) -> i32 {
    if minute == 24 {
        ((state.0 + state.1) & 0xFFFF) as i32
    } else {
        let mut r = 0;

        if state.1 >= blueprint[3].1 && (state.1 & 0xFFFF_0000) >= (blueprint[3].1 & 0xFFFF_0000) {
            return xxxx(
                blueprint,
                (state.0 + blueprint[3].0, state.1 + state.0 - blueprint[3].1),
                minute + 1,
            );
        }

        if state.1 >= blueprint[2].1
            && (state.1 & 0xFFFF_0000_0000) >= (blueprint[2].1 & 0xFFFF_0000_0000)
        {
            r = r.max(xxxx(
                blueprint,
                (state.0 + blueprint[2].0, state.1 + state.0 - blueprint[2].1),
                minute + 1,
            ));
        }

        if state.1 >= blueprint[1].1 {
            r = r.max(xxxx(
                blueprint,
                (state.0 + blueprint[1].0, state.1 + state.0 - blueprint[1].1),
                minute + 1,
            ));
        }

        if state.1 >= blueprint[0].1 {
            r = r.max(xxxx(
                blueprint,
                (state.0 + blueprint[0].0, state.1 + state.0 - blueprint[0].1),
                minute + 1,
            ));
        }

        r = r.max(xxxx(blueprint, (state.0, state.1 + state.0), minute + 1));

        r
    }
}
