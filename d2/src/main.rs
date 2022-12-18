use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::{self, BufRead};

#[derive(PartialEq, Debug, Clone)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<i32> for RockPaperScissors {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        use RockPaperScissors::*;
        match v {
            x if x == Rock as i32 => Ok(Rock),
            x if x == Paper as i32 => Ok(Paper),
            x if x == Scissors as i32 => Ok(Scissors),
            _ => Err(()),
        }
    }
}

#[test]
fn test_try_from() {
    assert_eq!(RockPaperScissors::try_from(0), Ok(RockPaperScissors::Rock));
    assert_eq!(RockPaperScissors::try_from(1), Ok(RockPaperScissors::Paper));
    assert_eq!(
        RockPaperScissors::try_from(2),
        Ok(RockPaperScissors::Scissors)
    );
    assert_eq!(RockPaperScissors::try_from(3), Err(()));
}

fn score_game(p1: &RockPaperScissors, p2: &RockPaperScissors) -> i32 {
    use RockPaperScissors::*;
    match (p1, p2) {
        (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => 6,
        (Scissors, Rock) | (Rock, Paper) | (Paper, Scissors) => 0,
        _ => 3,
    }
}

fn score_hand(hand: &RockPaperScissors) -> i32 {
    use RockPaperScissors::*;
    match hand {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    }
}

fn score(line: &str) -> i32 {
    // split line on space
    let his_move = line.chars().nth(0).unwrap() as i32;
    let my_move = line.chars().nth(2).unwrap();

    let his_move: RockPaperScissors = (his_move - 'A' as i32).try_into().unwrap();

    use RockPaperScissors::*;

    let my_move = match (&his_move, my_move) {
        (_, 'Y') => his_move.clone(),
        (Rock, 'X') => Scissors,
        (Paper, 'X') => Rock,
        (Scissors, 'X') => Paper,
        (Rock, 'Z') => Paper,
        (Paper, 'Z') => Scissors,
        (Scissors, 'Z') => Rock,
        (_, _) => panic!("Invalid input"),
    };

    // print debug string
    println!("{:?} vs {:?}", his_move, my_move);
    score_game(&my_move, &his_move) + score_hand(&my_move)
}

fn main() {
    // iterate over stdin lines
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    // map lines into score and sum
    let total_score = lines.map(|line| score(&line.unwrap())).sum::<i32>();

    println!("{}", total_score);
}
