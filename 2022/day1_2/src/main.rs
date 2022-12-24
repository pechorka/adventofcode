use std::fs;


fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Error reading input.txt");
    let mut calory_list: Vec<i32> = Vec::new();
    let mut current_calories = 0;
    for line in input.lines() {
        match line.parse::<i32>() {
            Ok(calories) => current_calories += calories,
            Err(_) => { // end of input for current elf
                calory_list.push(current_calories);
                current_calories = 0;
            },
        }
    }

    calory_list.sort();


    println!("Max calories on 3 elfs: {}", calory_list.iter().rev().take(3).sum::<i32>());
}