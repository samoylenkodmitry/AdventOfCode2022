use crate::day::Day;

mod day;
mod day1;
mod day2;
mod day3;

fn main() {
    let days: Vec<Box<dyn Day>> = vec![
        Box::new(day1::Day1),
        Box::new(day2::Day2),
        Box::new(day3::Day3),
    ];
    for day in days {
        day.compute();
    }
}
