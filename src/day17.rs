use std::borrow::Borrow;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::repeat;
use std::sync::{Arc, Mutex, RwLock};
use glium::vertex::MultiVerticesSource;
use rayon::iter::IntoParallelIterator;
use crate::day::Day;
use rayon::prelude::*;

pub(crate) struct Day17;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum UnitType {
    Horizontal,
    Cross,
    Corner,
    Vertical,
    Block,
}

struct Unit {
    x: usize,
    y: usize,
    unit_type: UnitType,
    is_placed: bool,
}

impl Unit {
    fn new(counter: usize) -> Self {
        Self {
            x: 2,
            y: 0,
            unit_type: match counter % 5 {
                0 => UnitType::Horizontal,
                1 => UnitType::Cross,
                2 => UnitType::Corner,
                3 => UnitType::Vertical,
                4 => UnitType::Block,
                _ => panic!("Impossible")
            },
            is_placed: false,
        }
    }
    fn right_border(&self) -> usize {
        match self.unit_type {
            UnitType::Horizontal => self.x + 3,
            UnitType::Cross => self.x + 2,
            UnitType::Corner => self.x + 2,
            UnitType::Vertical => self.x,
            UnitType::Block => self.x + 1,
            _ => panic!("Impossible")
        }
    }
    fn height(&self) -> usize {
        match self.unit_type {
            UnitType::Horizontal => 1,
            UnitType::Cross => 3,
            UnitType::Corner => 3,
            UnitType::Vertical => 4,
            UnitType::Block => 2,
            _ => panic!("Impossible")
        }
    }
    fn cells(&self) -> Vec<(usize, usize)> {
        match self.unit_type {
            UnitType::Horizontal => vec![(self.x, self.y), (self.x + 1, self.y), (self.x + 2, self.y), (self.x + 3, self.y)],
            UnitType::Cross => vec![(self.x + 1, self.y), (self.x + 1, self.y + 1), (self.x + 1, self.y + 2),
                                    (self.x, self.y + 1), (self.x + 1, self.y + 1), (self.x + 2, self.y + 1)],
            UnitType::Corner => vec![(self.x + 2, self.y), (self.x + 2, self.y + 1), (self.x + 2, self.y + 2),
                                     (self.x + 1, self.y + 2), (self.x, self.y + 2)],
            UnitType::Vertical => vec![(self.x, self.y), (self.x, self.y + 1), (self.x, self.y + 2), (self.x, self.y + 3)],
            UnitType::Block => vec![(self.x, self.y), (self.x + 1, self.y), (self.x, self.y + 1), (self.x + 1, self.y + 1)],
            _ => panic!("Impossible")
        }
    }
    fn is_collide(&self, board: &VecDeque<Vec<char>>) -> bool {
        self.cells().iter().any(|(x, y)| match board[*y][*x] {
            '.' => false,
            '#' => true,
            '-' => true,
            _ => panic!("Unexpected value in board: {}", board[*y][*x])
        })
    }

    fn put_to_board(&mut self, board: &mut VecDeque<Vec<char>>) {
        for (x, y) in self.cells() {
            board[y][x] = '#';
        }
        self.is_placed = true
    }
    fn move_down(&mut self, board: &VecDeque<Vec<char>>) -> bool {
        self.y += 1;
        if self.is_collide(board) {
            self.y -= 1;
            return false;
        }
        return true;
    }
    fn move_right(&mut self, board: &VecDeque<Vec<char>>) -> bool {
        if self.right_border() == board[0].len() - 1 {
            return false;
        }
        self.x += 1;
        if self.is_collide(board) {
            self.x -= 1;
            return false;
        }
        return true;
    }
    fn move_left(&mut self, board: &VecDeque<Vec<char>>) -> bool {
        if self.x == 0 {
            return false;
        }
        self.x -= 1;
        if self.is_collide(board) {
            self.x += 1;
            return false;
        }
        return true;
    }
}

impl Day for Day17 {
    /*
####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
## 
     */
    /*
    The first rock begins falling:
|..@@@@.|
|.......|
|.......|
|.......|
+-------+

Jet of gas pushes rock right:
|...@@@@|
|.......|
|.......|
|.......|
+-------+

Rock falls 1 unit:
|...@@@@|
|.......|
|.......|
+-------+

Jet of gas pushes rock right, but nothing happens:
|...@@@@|
|.......|
|.......|
+-------+

Rock falls 1 unit:
|...@@@@|
|.......|
+-------+

Jet of gas pushes rock right, but nothing happens:
|...@@@@|
|.......|
+-------+

Rock falls 1 unit:
|...@@@@|
+-------+

Jet of gas pushes rock left:
|..@@@@.|
+-------+

Rock falls 1 unit, causing it to come to rest:
|..####.|
+-------+

A new rock begins falling:
|...@...|
|..@@@..|
|...@...|
|.......|
|.......|
|.......|
|..####.|
+-------+

Jet of gas pushes rock left:
|..@....|
|.@@@...|
|..@....|
|.......|
|.......|
|.......|
|..####.|
+-------+

Rock falls 1 unit:
|..@....|
|.@@@...|
|..@....|
|.......|
|.......|
|..####.|
+-------+

Jet of gas pushes rock right:
|...@...|
|..@@@..|
|...@...|
|.......|
|.......|
|..####.|
+-------+

Rock falls 1 unit:
|...@...|
|..@@@..|
|...@...|
|.......|
|..####.|
+-------+

Jet of gas pushes rock left:
|..@....|
|.@@@...|
|..@....|
|.......|
|..####.|
+-------+

Rock falls 1 unit:
|..@....|
|.@@@...|
|..@....|
|..####.|
+-------+

Jet of gas pushes rock right:
|...@...|
|..@@@..|
|...@...|
|..####.|
+-------+

Rock falls 1 unit, causing it to come to rest:
|...#...|
|..###..|
|...#...|
|..####.|
+-------+

A new rock begins falling:
|....@..|
|....@..|
|..@@@..|
|.......|
|.......|
|.......|
|...#...|
|..###..|
|...#...|
|..####.|
+-------+
     */

    fn part1(&self, input: &str) -> String {
        Self::solve(input, 2022).to_string()
    }

    fn part2(&self, input: &str) -> String {
        Self::solve(input, 1000000000000).to_string()
    }

    fn get_test_data(&self) -> String {
        let a = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

        a.to_string()
        //"".to_string()
    }

    fn get_day_number(&self) -> i32 {
        return 17;
    }
}

impl Day17 {
    fn simulate_falling(mut grid: &mut VecDeque<Vec<char>>, pattern: &Vec<bool>, command_ind: &mut usize, unit: &mut Unit) {
        for _ in 0..3 + unit.height() {
            grid.push_front(vec!['.', '.', '.', '.', '.', '.', '.']);
        }
        print_grid(&grid, &unit);
        let mut simulating = true;
        while simulating {
            let pattern_pos = *command_ind % pattern.len();
            let to_the_right = pattern[pattern_pos];
            print_grid(&grid, &unit);
            if to_the_right {
                unit.move_right(&grid);
                assert!(!unit.is_collide(&grid), "check 1");
            } else {
                unit.move_left(&grid);
                assert!(!unit.is_collide(&grid), "check 2");
            }
            if !unit.move_down(&grid) {
                assert!(!unit.is_collide(&grid), "check 3");
                simulating = false;
                unit.put_to_board(&mut grid);
            }
            print_grid(&grid, &unit);
            *command_ind += 1;
        }
        while grid[0].iter().all(|c| *c == '.') {
            grid.pop_front();
        }
        print_grid(&grid, &unit);
    }

    fn solve(input: &str, rocks_to_fall: usize) -> usize {
        let mut grid: VecDeque<Vec<char>> = VecDeque::new();
        grid.push_front(vec!['-', '-', '-', '-', '-', '-', '-']);

        let pattern: Vec<bool> = input.chars().map(|c| c == '>').collect();

        let mut command_ind = 0;
        let mut unit_ind = 0;
        let mut cache: HashMap<(usize, usize, [usize; 7]), (usize, usize)> = HashMap::new();
        while unit_ind < rocks_to_fall {
            let mut unit = Unit::new(unit_ind);
            let unit_key = unit_ind % 5;
            let command_key = command_ind % pattern.len();
            // add unit to grid
            let height_map = height_map(&grid);
            let key = &(unit_key, command_key, height_map);
            let grid_len_before_one_step = grid.len();
            if cache.contains_key(key) {
                let (grid_len_before, unit_ind_before) = cache.get(key).unwrap();

                assert_eq!(unit_ind_before % 5, unit_key);
                let blocks_passed = unit_ind - unit_ind_before;
                let pattern_height = grid.len() - grid_len_before;
                let remaining_blocks = rocks_to_fall - unit_ind;
                let pattern_count = remaining_blocks / blocks_passed;
                let blocks_to_skip = pattern_count * blocks_passed;

                let blocks_to_fall = remaining_blocks - blocks_to_skip;

                let skipped_height = pattern_height * pattern_count;

                let mut total_height = grid.len() + skipped_height;
                (0..blocks_to_fall).for_each(|i| {
                    let mut unit = Unit::new(unit_ind_before + i);
                    let height_before = grid.len();
                    Self::simulate_falling(&mut grid, &pattern, &mut command_ind, &mut unit);
                    total_height += grid.len() - height_before;
                });

                return total_height - 1;
            } else {
                Self::simulate_falling(&mut grid, &pattern, &mut command_ind, &mut unit);
                cache.insert(*key, (grid_len_before_one_step, unit_ind));
                unit_ind += 1;
            }
        }

        grid.len() - 1
    }
}

fn print_grid(grid: &VecDeque<Vec<char>>, unit: &Unit) {
    if true {
        return;
    }
    let unit_cells = unit.cells();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if !unit.is_placed && unit_cells.contains(&(x, y)) {
                print!("@");
            } else {
                print!("{}", grid[y][x]);
            }
        }
        println!("");
    }
}

fn height_map(grid: &VecDeque<Vec<char>>) -> [usize; 7] {
    let mut height_map = [0; 7];
    for x in 0..7 {
        for y in 0..grid.len() {
            if grid[y][x] != '.' {
                height_map[x] = y;
                break;
            }
        }
    }
    height_map
}
