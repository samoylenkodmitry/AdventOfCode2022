
mod day1;
use crate::day1::Day1;

fn main() {
    let input = std::fs::read_to_string("./inputs/day1.txt").unwrap();
    let day1 = Day1;
    println!("Day 1, part 1: {}", day1.part1(&input));
    println!("Day 1, part 2: {}", day1.part2(&input));
}
