use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use crate::day::Day;

pub(crate) struct Day8;


impl Day for Day8 {
    fn part1(&self, input: &str) -> String {
        let mut visible: HashSet<(usize, usize)> = HashSet::new();
        let xy = Self::to_table(input);
        // from left to right
        for y in 0..xy.len() {
            let mut prev_max = -1;
            for x in 0..xy[y].len() {
                if xy[y][x] > prev_max {
                    prev_max = xy[y][x];
                    visible.insert((x, y));
                }
            }
        }
        // from right to left
        for y in 0..xy.len() {
            let mut prev_max = -1;
            for x in (0..xy[y].len()).rev() {
                if xy[y][x] > prev_max {
                    prev_max = xy[y][x];
                    visible.insert((x, y));
                }
            }
        }
        // from top to bottom
        for x in 0..xy[0].len() {
            let mut prev_max = -1;
            for y in 0..xy.len() {
                if xy[y][x] > prev_max {
                    prev_max = xy[y][x];
                    visible.insert((x, y));
                }
            }
        }
        // from bottom to top
        for x in 0..xy[0].len() {
            let mut prev_max = -1;
            for y in (0..xy.len()).rev() {
                if xy[y][x] > prev_max {
                    prev_max = xy[y][x];
                    visible.insert((x, y));
                }
            }
        }

        visible.len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let xy = Self::to_table(input);
        let w = xy[0].len();
        let mut max_score = 0;
        for y in 0..xy.len() {
            for x in 0..w {
                let val = xy[y][x];
                let mut count_r = 0;
                // to the right
                let mut xx = x + 1;
                while xx < w {
                    count_r += 1;
                    if xy[y][xx] >= val {
                        break;
                    }
                    xx += 1;
                }
                // to the left
                let mut count_l = 0;
                let mut xx = x as i32 - 1;
                while xx >= 0 {
                    count_l += 1;
                    if xy[y][xx as usize] >= val {
                        break;
                    }
                    xx -= 1;
                }
                // to the top
                let mut count_t = 0;
                let mut yy = y as i32 - 1;
                while yy >= 0 {
                    count_t += 1;
                    if xy[yy as usize][x] >= val {
                        break;
                    }
                    yy -= 1;
                }
                // to the bottom
                let mut count_b = 0;
                yy = y as i32 + 1;
                while yy < xy.len() as i32 {
                    count_b += 1;
                    if xy[yy as usize][x] >= val {
                        break;
                    }
                    yy += 1;
                }

                let score = count_r * count_l * count_t * count_b;
                if score > max_score {
                    max_score = score;
                }
            }
        }
        max_score.to_string()
    }

    fn get_test_data(&self) -> String {
        "30373
25512
65332
33549
35390".to_string()
    }

    fn get_day_number(&self) -> i32 {
        return 8;
    }
}

impl Day8 {
    fn to_table(input: &str) -> Vec<Vec<i32>> {
        let xy: Vec<Vec<i32>> = input.lines().map(|line| {
            line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect()
        }).collect();
        xy
    }
}

