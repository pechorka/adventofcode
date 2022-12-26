pub mod solver {
    use std::collections::LinkedList;
    use std::vec;

    pub fn part1(input: &str) -> String {
        let (mut stacks, moves) = parse_input(input);

        move_stacks_9000(&mut stacks, &moves);

        return top_containers(&stacks);
    }

    pub fn part2(input: &str) -> String {
        let (mut stacks, moves) = parse_input(input);

        move_stacks_9001(&mut stacks, &moves);

        return top_containers(&stacks);
    }

    fn top_containers(stacks: &Vec<LinkedList<char>>) -> String {
        let mut result = String::new();
        for stack in stacks {
            result.push(*stack.back().unwrap());
        }
        return result;
    }

    struct Move {
        amount: i32,
        from: usize,
        to: usize,
    }

    fn parse_input(input: &str) -> (Vec<LinkedList<char>>, Vec<Move>){
        let (raw_stacks, raw_moves) = input.split_at(input.find("move").unwrap());

        let stacks = parse_stacks(raw_stacks);
        let moves = parse_moves(raw_moves);
        return (stacks, moves);
    }

    fn parse_stacks(stacks: &str) -> Vec<LinkedList<char>> {
        let total = total_stacks(stacks);
        let mut result : Vec<LinkedList<char>> = vec::from_elem(LinkedList::new(), total);
        for line in stacks.lines().rev().skip(2) { // skip the last two lines
            let mut i = 0;
            for char_index in (1..line.len()).step_by(4) {
                let c = line.chars().nth(char_index).unwrap();
                if c.is_alphabetic() {
                    result[i].push_back(c);
                }
                i += 1;
            }
        }
        return result;
    }

    fn total_stacks(input: &str) -> usize {
        input.trim().lines()
            .last().expect("No lines in input")
            .chars().last().expect("No chars in last line")
            .to_digit(10).expect("Last char is not a number") as usize
    }

    // parses lines like "move 5 from 4 to 7" into a Move struct
    fn parse_moves(moves: &str) -> Vec<Move> {
        let mut result = Vec::new();
        for line in moves.lines() {
            let mut amount = 0;
            let mut from = 0;
            let mut to = 0;
            let mut i = 0;
            for part in line.split_whitespace() {
                match part.parse::<i32>() {
                    Ok(n) => match i{
                        1 => amount = n,
                        3 => from = n-1,
                        5 => to = n-1,
                        _ => panic!("Unexpected number in line {}", line),
                    },
                    Err(_) => (),
                }
                i += 1;
            }
            result.push(Move{amount, from: from as usize, to: to as usize});
        }
        return result;
    }

    fn move_stacks_9000(stacks: &mut Vec<LinkedList<char>>, moves: &Vec<Move>) {
        for m in moves {
            for _ in 0..m.amount {
                let c = stacks[m.from].pop_back().unwrap();
                stacks[m.to].push_back(c);
            }
        }
    }

    fn move_stacks_9001(stacks: &mut Vec<LinkedList<char>>, moves: &Vec<Move>) {
        for m in moves {
            // println!("move {} from {} to {}", m.amount, m.from, m.to);
            // print_stacks(stacks);
            let mut temp = LinkedList::new();
            for _ in 0..m.amount {
                let c = stacks[m.from].pop_back().unwrap();
                temp.push_front(c);
            }
            for _ in 0..m.amount {
                let c = temp.pop_front().unwrap();
                stacks[m.to].push_back(c);
            }
        }
    }

    // #[cfg(tests)]
    mod tests {
        use crate::solver;

        #[test]
        fn test_part1_example() {
            assert_eq!(solver::part1(EXAMPLE_INPUT), "CMZ");
        }

        #[test]
        fn test_part2_example() {
            assert_eq!(solver::part2(EXAMPLE_INPUT), "MCD");
        }

        #[test]
        fn test_part1() {
            assert_eq!(solver::part1(REAL_INPUT), "VJSFHWGFT");
        }

        #[test]
        fn test_part2() {
            assert_eq!(solver::part2(REAL_INPUT), "LCTQFBVZV");
        }

        static REAL_INPUT: &str = include_str!("../input.txt");
        static EXAMPLE_INPUT: &str = include_str!("../example.txt");
    }
}

