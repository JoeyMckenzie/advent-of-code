use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_top_three_calorie_counts(file_path: &str) -> anyhow::Result<u32> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // for part two, we'll store each of the calorie counts in a tuple list that we'll order later
    let mut sorted_calorie_counts = Vec::<u32>::new();
    let mut current_calorie_count = 0;

    for line in reader.lines() {
        let calorie_count = line?;

        // in the case we detect a break, we'll determine if the current
        // temporary calorie sum should be moved into the maximum detected sum
        if calorie_count.is_empty() {
            // reset the count as we'll begin counting for another elf
            sorted_calorie_counts.push(current_calorie_count);
            current_calorie_count = 0;
        } else if let Ok(parsed_calorie_count) = calorie_count.parse::<u32>() {
            current_calorie_count += parsed_calorie_count;
        }
    }

    sorted_calorie_counts.sort();
    sorted_calorie_counts.reverse();

    Ok(sorted_calorie_counts.into_iter().take(3).sum())
}
