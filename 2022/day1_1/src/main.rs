use std::fs;


fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Error reading input.txt");
    let mut max_calories = 0;
    let mut current_calories = 0;
    for line in input.lines() {
        match line.parse::<i32>() {
            Ok(calories) => current_calories += calories,
            Err(_) => { // end of input for current elf
                if current_calories > max_calories {
                    max_calories = current_calories;
                }
                current_calories = 0;
            },
        }
    }

    println!("Max calories: {}", max_calories);
}