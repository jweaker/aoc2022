use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("** Day1");
    let file_path = "src/days/day1/input.txt";
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut current_elf = 0;
    let mut max_elf = 0;
    let mut max_calories = i32::min_value();

    for line in reader.lines() {
        let line = line.unwrap();
        let entry = map.entry(current_elf).or_insert(0);

        if line.is_empty() {
            // Update the maximum Elf.
            if *entry > max_calories {
                max_elf = current_elf;
                max_calories = *entry;
            }

            // Switch to the next Elf.
            current_elf += 1;
            continue;
        }

        let calories = line.parse::<i32>().unwrap_or(0);

        // Use the entry API to insert or update the calories.
        *entry += calories;
    }

    println!(
        "The Elf with the most calories is Elf {}, with {} calories.",
        max_elf, max_calories
    );
}
