mod days;
use days::*;
use std::time::Instant;

fn main() -> Result<(), ()> {
    //    day1();
    //    day2();
    //    day3(get_ascii_input_lines(3));
    //    day4();
    //    day5();
    //    day6();
    //    day7();
    //    day8();
    //    day9();
    //    day10();
    //    day11();
    //    day12();
    //    day13();
    //    day14();
    //    day15();
    //    day16();
    //    day17();
    //    day18();
    //    day19();
    //    day20();
    //    day21();
    //    day22();
    //    day23();
    let s1 = Instant::now();
    day24();
    println!("The time is probably {}µs", s1.elapsed().as_micros());

    Ok(())
}
