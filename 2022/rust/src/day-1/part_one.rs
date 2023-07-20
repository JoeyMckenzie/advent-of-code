use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_max_calorie_count(file_path: &str) -> anyhow::Result<u32> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // we'll keep track of the maximum calorie count along
    // with a temporary rolling calorie count per elf
    let mut max_calorie_count: u32 = 0;
    let mut current_calorie_count: u32 = 0;

    for line in reader.lines() {
        let calorie_count = line?;

        // in the case we detect a break, we'll determine if the current
        // temporary calorie sum should be moved into the maximum detected sum
        if calorie_count.is_empty() {
            max_calorie_count = if current_calorie_count > max_calorie_count {
                current_calorie_count
            } else {
                max_calorie_count
            };

            // reset the count as we'll begin counting for another elf
            current_calorie_count = 0;
        } else if let Ok(parsed_calorie_count) = calorie_count.parse::<u32>() {
            current_calorie_count += parsed_calorie_count;
        }
    }

    Ok(max_calorie_count)
}
