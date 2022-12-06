mod part_one;
mod part_two;

use advent_of_code::NoOpResult;

const FILE_PATH: &str = "src/day-1/input.txt";

/// Advent of Code day 1: https://adventofcode.com/2022/day/1
fn main() -> NoOpResult {
    let max_calorie_count = part_one::get_max_calorie_count(FILE_PATH)?;
    let elf_calorie_counts = part_two::get_top_three_calorie_counts(FILE_PATH)?;

    println!("max_calorie_count {}", max_calorie_count);
    println!("elf_calorie_counts {:?}", elf_calorie_counts);

    Ok(())
}
