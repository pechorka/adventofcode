use std::fs;
use day3::solver;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let task1_answer = solver::solve_task1(&input);
    println!("Task 1 answer: {}", task1_answer);

    let task2_answer = solver::solve_task2(&input);
    println!("Task 2 answer: {}", task2_answer);
}
