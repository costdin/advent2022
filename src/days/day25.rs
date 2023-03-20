pub fn day25() {
    let result1 = to_result(
        to_base_5(
            include_str!("../../day25.txt")
                .trim()
                .lines()
                .map(|l| {
                    l.chars().fold(0i64, |acc, c| {
                        acc * 5
                            + match c {
                                '2' => 2,
                                '1' => 1,
                                '0' => 0,
                                '-' => -1,
                                '=' => -2,
                                _ => unreachable!(),
                            }
                    })
                })
                .sum::<i64>(),
        )
        .into_iter()
        .fold((vec![], 0), |(mut acc, carry), curr| match curr + carry {
            0 => {
                acc.push('0');
                (acc, 0)
            }
            1 => {
                acc.push('1');
                (acc, 0)
            }
            2 => {
                acc.push('2');
                (acc, 0)
            }
            3 => {
                acc.push('=');
                (acc, 1)
            }
            4 => {
                acc.push('-');
                (acc, 1)
            }
            5 => {
                acc.push('0');
                (acc, 1)
            }
            _ => unreachable!(),
        }),
    );

    println!("DAY 25\nSolution 1: {result1}");
}

fn to_base_5(mut n: i64) -> Vec<u8> {
    let mut result = vec![];

    while n > 5 {
        result.push((n % 5) as u8);
        n /= 5;
    }

    result.push(n as u8);

    result
}

fn to_result((v, carry): (Vec<char>, u8)) -> String {
    if carry == 1 {
        (v.into_iter().chain(['1']).rev()).collect()
    } else {
        v.into_iter().rev().collect()
    }
}
