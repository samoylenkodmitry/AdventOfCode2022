use std::collections::{VecDeque};
use crate::day::Day;

pub(crate) struct Day10;


impl Day for Day10 {
    fn part1(&self, input: &str) -> String {
        let mut sum = 0;
        let mut display = [[0u8;41];7];
        Self::do_cpu_cycles(input, &mut sum, &mut display);
        sum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut display = [[0u8;41];7];
        Self::do_cpu_cycles(input, &mut 0, &mut display);
        let mut display_str = String::new();
        for line in display {
            display_str.push('\n');
            for pixel in line {
                display_str.push_str(if pixel == 0 { " " } else { "#" });
            }
        }
        display_str
    }

    fn get_test_data(&self) -> String {
        let a = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
".to_string();
        a
    }

    fn get_day_number(&self) -> i32 {
        return 10;
    }
}

impl Day10 {
    fn do_cpu_cycles(input: &str, mut sum: &mut i32, mut display: &mut [[u8; 41]; 7]) {
        let mut cycle = 1;
        let mut signal = 1;
        input.lines()
            .for_each(|line| {
                draw_pixel_crt(&mut display, signal, cycle);
                if line.starts_with("noop") {
                    cycle += 1;
                    check_signal(&mut sum, signal, cycle);
                } else {
                    let mut parts = line.split_whitespace();
                    let value = parts.nth(1).unwrap().parse::<i32>().unwrap();
                    cycle += 1;
                    check_signal(&mut sum, signal, cycle);
                    draw_pixel_crt(&mut display, signal, cycle);
                    cycle += 1;
                    signal += value;
                    check_signal(&mut sum, signal, cycle);
                }
                draw_pixel_crt(&mut display, signal, cycle);
            });
    }
}

fn draw_pixel_crt(display: &mut [[u8; 41]; 7], signal: i32, cycle: i32) {
    let pixel_y = ((cycle-1) / 40);
    let pixel_x = ((cycle-1) % 40);
    if (signal - pixel_x).abs() <= 1 {
        display[pixel_y as usize][pixel_x as usize] = 1;
    } else {
        display[pixel_y as usize][pixel_x as usize] = 0;
    }
}

fn check_signal(sum: &mut i32, signal: i32, cycle: i32) {
    if cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220 {
        *sum = *sum + signal * (cycle as i32);
    }
}


