use rayon::prelude::*;
use std::collections::HashMap;

pub fn day19() {
    let input = include_str!("../../day19.txt")
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
                    (
                        ((clay_ore_cost + obs_ore_cost + geode_ore_cost) << 48)
                            | 0x0000_FFFF_FFFF_FFFF,
                        0,
                    ),
                    (ore_ore_cost.min(clay_ore_cost), 0)
                ]
            },
        )
        .zip(1..)
        .collect::<Vec<_>>();

    let result1 = input
        .par_iter()
        .map(|(blueprint, ix)| ix * solve(blueprint, (1 << 48, blueprint[5].0 << 48), 24 - blueprint[5].0 as i32, 0, &mut HashMap::new()))
        .sum::<i32>();

    let result2 = input
        .par_iter()
        .take(3)
        .map(|(blueprint, _)| solve(blueprint, (1 << 48, blueprint[5].0 << 48), 32 - blueprint[5].0 as i32, 0, &mut HashMap::new()))
        .reduce(|| 1, |acc, e| acc * e);
    println!("DAY 19\nSolution 1: {result1}\nSolution 2: {result2}");
}

fn solve(
    blueprint: &[(u64, u64); 6],
    state: (u64, u64),
    count: i32,
    current_max: i32,
    seen: &mut HashMap<u128, i32>,
) -> i32 {
    let key = (state.0 as u128) << 64 | state.1 as u128;

    match seen.get_mut(&key) {
        Some(i) if *i >= count => return 0,
        Some(i) => *i = count,
        None => {
            seen.insert(key, count);
        }
    };

    if count == 1 {
        ((state.0 + state.1) & 0xFFFF) as i32
    } else {
        let max_achievable =
            (state.1 & 0xFFFF) as i32 + (state.0 & 0xFFFF) as i32 * count + count * (count + 1) / 2;
        if current_max > max_achievable {
            return 0;
        }

        if state.1 >= blueprint[3].1 && (state.1 & 0xFFFF_0000) >= (blueprint[3].1 & 0xFFFF_0000) {
            return solve(
                blueprint,
                (state.0 + blueprint[3].0, state.1 + state.0 - blueprint[3].1),
                count - 1,
                current_max,
                seen,
            );
        }

        let need_obsidian = state.1 & 0xFFFF_0000 < blueprint[3].1 & 0xFFFF_0000;
        let need_clay = state.0 & 0xFFFF_0000_0000 < blueprint[2].1 & 0xFFFF_0000_0000;
        let need_ore = state.0 < blueprint[4].0;

        if need_obsidian
            && !need_ore
            && !need_clay
            && !(state.1 >= blueprint[2].1
                && state.1 & 0xFFFF_0000_0000 >= blueprint[2].1 & 0xFFFF_0000_0000)
        {
            println!("NO?");
        }

        let mut r = current_max;
        if need_obsidian
            && state.1 >= blueprint[2].1
            && state.1 & 0xFFFF_0000_0000 >= blueprint[2].1 & 0xFFFF_0000_0000
        {
            r = r.max(solve(
                blueprint,
                (state.0 + blueprint[2].0, state.1 + state.0 - blueprint[2].1),
                count - 1,
                r,
                seen,
            ));
        }

        if need_clay && state.1 >= blueprint[1].1 {
            r = r.max(solve(
                blueprint,
                (state.0 + blueprint[1].0, state.1 + state.0 - blueprint[1].1),
                count - 1,
                r,
                seen,
            ));
        }

        if need_ore && state.1 >= blueprint[0].1 {
            r = r.max(solve(
                blueprint,
                (state.0 + blueprint[0].0, state.1 + state.0 - blueprint[0].1),
                count - 1,
                r,
                seen,
            ));
        }

        r = r.max(solve(
            blueprint,
            (state.0, state.1 + state.0),
            count - 1,
            r,
            seen,
        ));

        r
    }
}
