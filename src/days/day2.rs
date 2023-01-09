use super::get_input_lines;

pub fn day2() {
    let (result1, result2) = get_input_lines(2)
        .iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .map(|l| (l[0], l[2]))
        .map(|l| (Shape::parse(l.0), Shape::parse(l.1), GameResult::parse(l.1)))
        .map(|(opponent, response, game_result)| {
            (
                Shape::get_response(&game_result, &opponent),
                opponent,
                response,
            )
        })
        .map(|(second_response, opponent, response)| {
            (
                response.evaluate(&opponent).value() + response.value(),
                (second_response.evaluate(&opponent).value() + second_response.value()),
            )
        })
        .fold((0, 0), |acc, (r1, r2)| (acc.0 + r1, acc.1 + r2));

    println!("DAY 2\nSolution 1: {}\nSolution 2: {}", result1, result2);
}

#[derive(PartialEq, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum GameResult {
    Win,
    Draw,
    Loss,
}

impl GameResult {
    pub fn value(&self) -> i32 {
        match self {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Loss => 0,
        }
    }

    pub fn parse(entry: char) -> GameResult {
        match entry {
            'X' => GameResult::Loss,
            'Y' => GameResult::Draw,
            'Z' => GameResult::Win,
            _ => panic!("We should never get here!"),
        }
    }
}

impl Shape {
    pub fn parse(entry: char) -> Shape {
        match entry {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => panic!("We should never get here!"),
        }
    }

    pub fn evaluate(&self, opponent: &Shape) -> GameResult {
        match (self, opponent) {
            (Shape::Rock, Shape::Scissors) => GameResult::Win,
            (Shape::Paper, Shape::Rock) => GameResult::Win,
            (Shape::Scissors, Shape::Paper) => GameResult::Win,
            (s, o) if s == o => GameResult::Draw,
            _ => GameResult::Loss,
        }
    }

    pub fn get_response(result: &GameResult, opponent: &Shape) -> Shape {
        match (result, opponent) {
            (GameResult::Draw, o) => o.clone(),
            (GameResult::Win, Shape::Rock) => Shape::Paper,
            (GameResult::Win, Shape::Paper) => Shape::Scissors,
            (GameResult::Win, Shape::Scissors) => Shape::Rock,
            (GameResult::Loss, Shape::Rock) => Shape::Scissors,
            (GameResult::Loss, Shape::Paper) => Shape::Rock,
            (GameResult::Loss, Shape::Scissors) => Shape::Paper,
        }
    }

    pub fn value(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}
