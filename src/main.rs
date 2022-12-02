use crate::day::Day;

mod day;
mod day1;
mod day2;

fn main() {
    let days: Vec<Box<dyn Day>> = vec![
        Box::new(day1::Day1),
        Box::new(day2::Day2),
    ];
    for day in days {
        day.compute();
    }
}
