use std::{cmp::Ordering, fs, str::FromStr};
pub fn solve() {
    println!("** Day2");
    let file_path = "src/days/day2/input.txt";
    let input = fs::read_to_string(file_path).unwrap();
    let sol1 = part1(&input);
    // let sol2 = part2(&input);
    println!("sol1: {} | sol2: {}", sol1, 4);
}

#[derive(PartialEq, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "Y" => Ok(Move::Paper),
            "B" | "X" => Ok(Move::Rock),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("not a valid move".to_string()),
        }
    }
}
impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == &Move::Scissors && other == &Move::Rock {
            Some(Ordering::Less)
        } else if self == &Move::Rock && other == &Move::Scissors {
            Some(Ordering::Greater)
        } else {
            Some((*self as u8).cmp(&(*other as u8)))
        }
    }
}

fn round_score(m1: Move, m2: Move) -> u32 {
    match m1.partial_cmp(&m2).unwrap() {
        Ordering::Less => m1 as u32,
        Ordering::Equal => 3 + m1 as u32,
        Ordering::Greater => 6 + m1 as u32,
    }
}
fn part1(input: &str) -> String {
    let sum: u32 = input
        .lines()
        .map(|line| {
            let moves: Vec<Move> = line
                .split(' ')
                .map(|s| s.parse::<Move>().unwrap())
                .collect();
            round_score(moves[0], moves[1])
        })
        .sum();
    sum.to_string()
}

// fn part2(input: &str) -> String {
//     input.to_string()
// }

#[test]
fn test() {
    let input = "A Y
B X
C Z"
    .to_string();

    let sol1 = part1(&input);
    println!("{}", sol1);
    assert_eq!(sol1, 15.to_string());
    // let sol2 = part2(&input);
    // assert_eq!(sol2, input);
}
