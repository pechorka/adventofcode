use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Error reading input.txt");

    let strategy = create_strategy(&input);
    println!("Total score: {}", calc_score(&strategy));
}

fn create_strategy(input: &str) -> String {
    let (win_map, loss_map, draw_map) = init_answer_maps();
    let mut strategy = String::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (opponent, my_action) = line.split_at(1);
        let my_response = match my_action {
            " X" => *loss_map.get(opponent).unwrap(),
            " Y" => *draw_map.get(opponent).unwrap(),
            " Z" => *win_map.get(opponent).unwrap(),
            _ => panic!("Unknown action: {}", my_action),
        };

        strategy.push_str(opponent);
        strategy.push_str(" ");
        strategy.push_str(my_response);
        strategy.push_str("\n");
    }

    return strategy;
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
    use crate::create_strategy;

    #[test]
    fn check_known_score() {
        let strategy = create_strategy("\
        A Y
        B X
        C Z
        ");
        let result = calc_score(&strategy);
        assert_eq!(result, 12);
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

fn init_answer_maps() -> (HashMap<&'static str, &'static str>,HashMap<&'static str, &'static str>,HashMap<&'static str, &'static str>) {
    let mut win_map = HashMap::new();
    let mut loss_map = HashMap::new();
    let mut draw_map = HashMap::new();

    win_map.insert("A", "Y");
    win_map.insert("B", "Z");
    win_map.insert("C", "X");

    loss_map.insert("A", "Z");
    loss_map.insert("B", "X");
    loss_map.insert("C", "Y");

    draw_map.insert("A", "X");
    draw_map.insert("B", "Y");
    draw_map.insert("C", "Z");

    return (win_map, loss_map, draw_map);
}