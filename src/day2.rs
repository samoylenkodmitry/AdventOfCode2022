/*
 */
use crate::day::Day;

pub(crate) struct Day2;


impl Day for Day2 {
    fn part1(&self, input: &str) -> String {
        let games = input.trim().split("\n");

        let mut total = 0;
        for game in games {
            // chars
            let mut chars = game.chars();
            // first char is opponent
            let opponent = chars.next().unwrap();
            chars.next().unwrap(); // skip space
            // third char is my move
            let my_move = chars.next().unwrap();
            // opponent: A - rock, B - paper, C - scissors
            // mine:     X - rock, Y - paper, Z - scissors
            let mut score = 0;
            match opponent {
                'A' => {
                    match my_move {
                        'X' => { score = 3 + 1; }
                        'Y' => { score = 6 + 2; }
                        'Z' => { score = 0 + 3; }
                        _ => println!("Invalid move"),
                    }
                }
                'B' => {
                    match my_move {
                        'X' => { score = 0 + 1; }
                        'Y' => { score = 3 + 2; }
                        'Z' => { score = 6 + 3; }
                        _ => println!("Invalid move"),
                    }
                }
                'C' => {
                    match my_move {
                        'X' => { score = 6 + 1; }
                        'Y' => { score = 0 + 2; }
                        'Z' => { score = 3 + 3; }
                        _ => println!("Invalid move"),
                    }
                }
                _ => println!("Invalid opponent"),
            }
            total += score;
        }

        return total.to_string();
    }

    fn part2(&self, input: &str) -> String {
        // lets map char to score
        // X - 0, Y - 3, Z - 6
        // create a map
        let mut score_map = std::collections::HashMap::new();
        score_map.insert('X', 0);
        score_map.insert('Y', 3);
        score_map.insert('Z', 6);

        let games = input.trim().split("\n");

        let mut total = 0;
        for game in games {
            // chars
            let mut chars = game.chars();
            // first char is opponent
            let opponent = chars.next().unwrap();
            chars.next().unwrap(); // skip space
            // third char is my move
            let my_move = chars.next().unwrap();
            // opponent: A - rock, B - paper, C - scissors
            // mine:     X - loose, Y - draw, Z - win
            // rock->1, paper->2, scissors->3

            let mut score = *score_map.get(&my_move).unwrap();
            match opponent {
                'A' => { // rock
                    match my_move {
                        'X' => { score += 3; } // lose -> scissors
                        'Y' => { score += 1; } // draw -> rock
                        'Z' => { score += 2; } // win -> paper
                        _ => println!("Invalid move"),
                    }
                }
                'B' => { // paper
                    match my_move {
                        'X' => { score += 1; } // lose -> rock
                        'Y' => { score += 2; } // draw -> paper
                        'Z' => { score += 3; } // win -> scissors
                        _ => println!("Invalid move"),
                    }
                }
                'C' => { // scissors
                    match my_move {
                        'X' => { score += 2; } // lose -> paper
                        'Y' => { score += 3; } // draw -> scissors
                        'Z' => { score += 1; } // win -> rock
                        _ => println!("Invalid move"),
                    }
                }
                _ => println!("Invalid opponent"),
            }
            total += score;
        }

        return total.to_string();
    }

    fn get_test_data(&self) -> String {
        "".to_string()
    }

    fn get_day_number(&self) -> i32 {
        return 2;
    }
}