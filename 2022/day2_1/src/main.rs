use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Error reading input.txt");

    println!("Total score: {}", calc_score(&input));
}

fn calc_score(input: &str) -> i32 {
    let scores = init_scores_map();

    let mut total_score = 0;
    for line in input.lines() {
        let score = scores.get(line.trim()).unwrap_or(&0);
        total_score += score;
    }

    total_score
}

#[cfg(test)]
mod tests {
    use crate::calc_score;

    #[test]
    fn check_known_score() {
        let result = calc_score("\
        A Y
        B X
        C Z
        ");
        assert_eq!(result, 15);
    }
}

fn init_scores_map() -> HashMap<&'static str, i32> {
    let mut scores = HashMap::new();

    // A,X - rock
    // B,Y - paper
    // C,Z - scissors

    // You get 1 point for selecting rock
    // You get 2 points for selecting paper
    // You get 3 points for selecting scissors

    // You get 0 points for loss
    // You get 3 points for draw
    // You get 6 points for win
    scores.insert("A X",4); // 1 (rock) + 3 (rock on rock, draw)
    scores.insert("A Y",8); // 2 (paper) + 0 (paper on rock, win)
    scores.insert("A Z",3); // 3 (scissors) + 0 (scissors on rock, loss)
    scores.insert("B X",1); // 1 (rock) + 0 (rock on paper, loss)
    scores.insert("B Y",5); // 2 (paper) + 3 (paper on paper, draw)
    scores.insert("B Z",9); // 3 (scissors) + 6 (scissors on paper, win)
    scores.insert("C X",7); // 1 (rock) + 6 (rock on scissors, win)
    scores.insert("C Y",2); // 2 (paper) + 0 (paper on scissors, loss)
    scores.insert("C Z",6); // 3 (scissors) + 3 (scissors on scissors, draw)
    scores
}
