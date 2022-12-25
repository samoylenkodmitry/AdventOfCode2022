use std::cmp::{max, min};

use std::collections::HashSet;

use std::iter::repeat;

use crate::day::Day;

pub(crate) struct Day23;

struct Elf {
    y: i32,
    x: i32,
    move_y: i32,
    move_x: i32,
    direction: usize,
}


impl Elf {
    fn new(y: i32, x: i32) -> Self {
        Self {
            y,
            x,
            move_y: y,
            move_x: x,
            direction: 0,
        }
    }
    pub(crate) fn forget_move(&mut self) {
        self.move_y = self.y;
        self.move_x = self.x;
    }
    fn same_move(&self, other: &Elf) -> bool {
        self.move_y == other.move_y && self.move_x == other.move_x
    }
    fn have_neighbours(&self, yx: &HashSet<(i32, i32)>) -> bool {
        yx.contains(&(self.y - 1, self.x)) ||
            yx.contains(&(self.y + 1, self.x)) ||
            yx.contains(&(self.y, self.x - 1)) ||
            yx.contains(&(self.y, self.x + 1)) ||
            yx.contains(&(self.y - 1, self.x - 1)) ||
            yx.contains(&(self.y - 1, self.x + 1)) ||
            yx.contains(&(self.y + 1, self.x - 1)) ||
            yx.contains(&(self.y + 1, self.x + 1))
    }
    fn do_move(&mut self) {
        self.y = self.move_y;
        self.x = self.move_x;
    }
    fn change_direction(&mut self) {
        self.direction = (self.direction + 1) % 4;
    }
    fn plan_by_direction(&self, direction: usize) -> (i32, i32) {
        plan_direct_move(self.y as i32, self.x as i32, direction)
    }
    fn plan_move(&mut self, yx: &HashSet<(i32, i32)>) -> bool {
        for i in 0..4 {
            if !contains_direction(yx, self.y as i32, self.x as i32, (self.direction + i) % 4) {
                let (y, x) = self.plan_by_direction((self.direction + i) % 4);
                self.move_y = y;
                self.move_x = x;
                return true;
            }
        }
        self.move_x = self.x;
        self.move_y = self.y;
        return false;
    }
}

fn contains_direction(yx: &HashSet<(i32, i32)>, y: i32, x: i32, direction: usize) -> bool {
    //println!("y={}, x={}, direction={}", y, x, direction);
    //println!("y-1={}, x={}, direction={}", y - 1, x, direction);
    // direct
    let (dy, dx) = plan_direct_move(y, x, direction);
    // clock wise
    let (cwy, cwx) = match direction {
        0 => (y - 1, x + 1),
        3 => (y + 1, x + 1),
        1 => (y + 1, x - 1),
        2 => (y - 1, x - 1),
        _ => panic!("Impossible")
    };
    // counter clock wise
    let (ccwy, ccwx) = match direction {
        0 => (y - 1, x - 1),
        3 => (y - 1, x + 1),
        1 => (y + 1, x + 1),
        2 => (y + 1, x - 1),
        _ => panic!("Impossible")
    };
    let res = yx.contains(&(dy, dx)) || yx.contains(&(cwy, cwx)) || yx.contains(&(ccwy, ccwx));
    //println!("blocked direct ?  {} {} {}", dy, dx, yx.contains(&(dy, dx)));
    //println!("blocked clock ?  {} {} {}", cwy, cwx, yx.contains(&(cwy, cwx)));
    //println!("blocked counter ?  {} {} {}", ccwy, ccwx, yx.contains(&(ccwy, ccwx)));
    //println!("{} {} {} {}", y, x, direction, res);
    res
}

fn plan_direct_move(y: i32, x: i32, direction: usize) -> (i32, i32) {
    let (dy, dx) = match direction {
        0 => (y - 1, x),
        3 => (y, x + 1),
        1 => (y + 1, x),
        2 => (y, x - 1),
        _ => panic!("Impossible")
    };
    (dy, dx)
}

impl Day for Day23 {
    fn part1(&self, input: &str) -> String {
        let max_round = 10;
        let (elves, iterations) = Self::simulate(input, max_round);
        let elves_count = elves.len();
        let mut new_max_x = elves[0].x;
        let mut new_max_y = elves[0].y;
        let mut new_min_x = new_max_x;
        let mut new_min_y = new_max_y;
        for i in 0..elves_count {
            new_max_x = max(new_max_x, elves[i].x);
            new_max_y = max(new_max_y, elves[i].y);
            new_min_x = min(new_min_x, elves[i].x);
            new_min_y = min(new_min_y, elves[i].y);
        }
        let mut new_xy: Vec<Vec<char>> = repeat(repeat('.').take((new_max_x - new_min_x + 1) as usize)
            .collect()).take((new_max_y - new_min_y + 1) as usize).collect();
        for i in 0..elves_count {
            new_xy[(elves[i].y - new_min_y) as usize][(elves[i].x - new_min_x) as usize] = '#';
        }
        // count empty
        let mut empty = 0;
        for y in 0..new_xy.len() {
            for x in 0..new_xy[y].len() {
                if new_xy[y][x] == '.' {
                    new_xy[y][x] = '@';
                    empty += 1;
                } else {
                    new_xy[y][x] = ' ';
                }
            }
        }
        //// print
        //for y in 0..new_xy.len() {
        //    for x in 0..new_xy[y].len() {
        //        print!("{}", new_xy[y][x]);
        //    }
        //    println!("");
        //}

        empty.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (elves, iterations) = Self::simulate(input, usize::MAX);
        iterations.to_string()
    }

    fn get_test_data(&self) -> String {
        let a = ".....
..##.
..#..
.....
..##.
.....";
        let b = "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";

        //a.to_string()
        b.to_string()
        //"".to_string()
    }

    fn get_day_number(&self) -> i32 {
        return 23;
    }
}

impl Day23 {
    fn simulate(input: &str, max_round: usize) -> (Vec<Elf>, usize) {
        let mut xy: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let mut elves: Vec<Elf> = Vec::new();
        for y in 0..xy.len() {
            for x in 0..xy[y].len() {
                if xy[y][x] == '#' {
                    elves.push(Elf::new(y as i32, x as i32));
                }
            }
        }
        let mut simulating = true;
        let elves_count = elves.len();
        let mut moving: Vec<bool> = vec![true; elves_count];
        let mut round = 0 as usize;
        while simulating {
            round += 1;
            //println!("Round {}", round);
            // print_map(max_x, max_y, &elves);
            let elves_yx: HashSet<(i32, i32)> = elves.iter().map(|e| (e.y as i32, e.x as i32)).collect();
            elves.iter_mut().enumerate().for_each(|(i, elf)| {
                if elf.have_neighbours(&elves_yx) {
                    //       println!("Elf {} has neighbours, y: {}, x: {}", i, elf.y, elf.x);
                    moving[i] = elf.plan_move(&elves_yx);
                    //        println!("Elf {} plan to move to {},{}, from {} {}, direction={}", i, elf.move_y, elf.move_x, elf.y, elf.x, elf.direction);
                } else {
                    moving[i] = false;
                }
            });
            for i in 0..elves_count {
                let elf1 = &elves[i];
                for j in 0..elves_count {
                    if i != j {
                        let elf2 = &elves[j];
                        if elf2.same_move(&elf1) {
                            moving[i] = false;
                            moving[j] = false;
                            //println!("Elf {} and {} are in the same move", i, j);
                            break;
                        }
                    }
                }
            }
            //print_map(max_x, max_y, &elves);
            let mut moves_count = 0;
            for i in 0..elves_count {
                if moving[i] {
                    //println!("Elf {} move to {},{} from {} {}", i, elves[i].move_y, elves[i].move_x, elves[i].y, elves[i].x);
                    elves[i].do_move();
                    //println!("Elf {} is now at {},{}", i, elves[i].y, elves[i].x);
                    moves_count += 1;
                } else {
                    elves[i].forget_move();
                }
                elves[i].change_direction();
                //println!("Elf {} change direction to {}", i, elves[i].direction);
            }
            if moves_count == 0 {
                simulating = false;
            }
            //println!("Moves: {}, round: {}", moves_count, round);
            //print_map( &elves);
            if round == max_round {
                break;
            }
        }
        (elves, round)
    }
}

fn print_map(elfs: &Vec<Elf>) {
    let mut max_x = elfs[0].x;
    let mut max_y = elfs[0].y;
    let mut min_x = max_x;
    let mut min_y = max_y;
    for i in 0..elfs.len() {
        max_x = max(max_x, elfs[i].x);
        max_y = max(max_y, elfs[i].y);
        min_x = min(min_x, elfs[i].x);
        min_y = min(min_y, elfs[i].y);
    }
    let mut xy: Vec<Vec<char>> = repeat(repeat('.').take((max_x - min_x + 1) as usize)
        .collect()).take((max_y - min_y + 1) as usize).collect();
    for i in 0..elfs.len() {
        let elf = &elfs[i];
        xy[(elf.y - min_y) as usize][(elf.x - min_x) as usize] = '#';//('0' as u8 + i as u8) as char;
        if elf.y != elf.move_y || elf.x != elf.move_x {
            xy[(elf.move_y - min_y) as usize][(elf.move_x - min_x) as usize] = '@';
        }
    }
    for y in 0..xy.len() {
        for x in 0..xy[y].len() {
            print!("{}", xy[y][x]);
        }
        println!();
    }
    println!();
}