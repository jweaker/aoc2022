use std::fs;
pub fn solve() {
    println!("** Day1");
    let file_path = "src/days/day2/input.txt";
    let input = fs::read_to_string(file_path).unwrap();
    let sol1 = part1(&input);
    let sol2 = part2(&input);
    println!("sol1: {} | sol2: {}", sol1, sol2);
}
fn part1(input: &String) -> String {
    input.to_string()
}

fn part2(input: &String) -> String {
    input.to_string()
}

#[test]
fn test() {
    let input = "A B".to_string();

    let sol1 = part1(&input);
    assert_eq!(sol1, input);
    let sol2 = part2(&input);
    assert_eq!(sol2, input);
}
