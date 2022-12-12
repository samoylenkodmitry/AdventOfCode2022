use std::collections::{HashSet, VecDeque};
use crate::day::Day;

pub(crate) struct Day12;


impl Day for Day12 {
    fn part1(&self, input: &str) -> String {
        Self::solve(input, |x: usize, y: usize, s_x: usize, s_y: usize, _: i32| -> bool {
            x == s_x && y == s_y
        }).to_string()
    }

    fn part2(&self, input: &str) -> String {
        Self::solve(input, |x: usize, y: usize, s_x: usize, s_y: usize, curr: i32| -> bool {
            curr == 'a' as i32
        }).to_string()
    }

    fn get_test_data(&self) -> String {
        let a = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi".to_string();

        a
    }

    fn get_day_number(&self) -> i32 {
        return 12;
    }
}

impl Day12 {
    fn solve(input: &str, finish_condition: fn(usize, usize, usize, usize, i32) -> bool) -> usize {
        let mut s_x = 0;
        let mut s_y = 0;
        let mut e_x = 0;
        let mut e_y = 0;
        let xy: Vec<Vec<i32>> = input.lines().enumerate().map(|(y, line)| {
            line.chars().enumerate().map(|(x, c)| {
                let mut v = 0;
                if c == 'S' {
                    s_x = x;
                    s_y = y;
                    v = 'a' as i32;
                } else if c == 'E' {
                    e_x = x;
                    e_y = y;
                    v = 'z' as i32;
                } else {
                    v = c as i32;
                }

                v
            }).collect()
        }).collect();
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        q.push_back((e_x, e_y));


        let mut visited_set: HashSet<(usize, usize)> = HashSet::new();
        let mut min = 0;
        'outer:
        while !q.is_empty() {
            let sz = q.len();
            for _ in 0..sz {
                let (x, y) = q.pop_front().unwrap();
                let curr = xy[y][x];
                if finish_condition(x, y, s_x, s_y, curr) {
                    break 'outer;
                }
                if x > 0 {
                    compare_and_try_add(&xy, &mut q, &mut visited_set, curr, x - 1, y);
                }
                if x < xy[0].len() - 1 {
                    compare_and_try_add(&xy, &mut q, &mut visited_set, curr, x + 1, y);
                }
                if y > 0 {
                    compare_and_try_add(&xy, &mut q, &mut visited_set, curr, x, y - 1);
                }
                if y < xy.len() - 1 {
                    compare_and_try_add(&xy, &mut q, &mut visited_set, curr, x, y + 1);
                }
            }
            min += 1;
            if min > xy.len() * xy[0].len() {
                panic!("not found");
            }
        }
        min
    }
}

fn compare_and_try_add(
    xy: &Vec<Vec<i32>>,
    mut q: &mut VecDeque<(usize, usize)>,
    mut set: &mut HashSet<(usize, usize)>,
    curr: i32,
    new_x: usize,
    new_y: usize,
) {
    let v = xy[new_y][new_x];
    if -v + curr <= 1 {
        try_add(&mut q, set, &(new_x, new_y));
    }
}

fn try_add(
    q: &mut VecDeque<(usize, usize)>,
    visited_set: &mut HashSet<(usize, usize)>,
    new_v: &(usize, usize),
) {
    if visited_set.insert(*new_v) {
        q.push_back(*new_v);
    }
}
