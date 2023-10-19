use std::fs;
pub fn solve() {
    println!("** Day1");
    let file_path = "src/days/day1/input.txt";
    let input = fs::read_to_string(file_path).unwrap();
    let sol1 = part1(&input);
    let sol2 = part2(&input);
    println!("sol1: {} | sol2: {}", sol1, sol2);
}
fn part1(input: &String) -> String {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|val| val.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
        .to_string()
}

fn part2(input: &String) -> String {
    let mut maxes: Vec<u32> = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|val| val.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect();
    maxes.sort_by(|a, b| b.cmp(a));
    maxes.iter().take(3).sum::<u32>().to_string()
}

#[test]
fn test() {
    let input = "1000
2000
3000

4000
5000

6000
7000
8000

9000

10000"
        .to_string();

    let sol1 = part1(&input);
    assert_eq!(sol1, "21000");
    let sol2 = part2(&input);
    assert_eq!(sol2, "40000");
}
