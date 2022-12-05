use crate::day::Day;

mod day;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let days: Vec<Box<dyn Day>> = vec![
        Box::new(day1::Day1),
        Box::new(day2::Day2),
        Box::new(day3::Day3),
        Box::new(day4::Day4),
        Box::new(day5::Day5),
    ];
    for day in days {
        day.compute();
    }
}
