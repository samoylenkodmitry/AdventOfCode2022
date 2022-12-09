use std::collections::HashMap;
use crate::day::Day;
use rayon::prelude::*;

mod day;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let days: Vec<Box<dyn Day>> = vec![
        Box::new(day1::Day1),
        Box::new(day2::Day2),
        Box::new(day3::Day3),
        Box::new(day4::Day4),
        Box::new(day5::Day5),
        Box::new(day6::Day6),
        Box::new(day7::Day7),
        Box::new(day8::Day8),
        Box::new(day9::Day9),
    ];
    let mut results = (0..days.len()).into_par_iter().map(|i| {
        days[i].compute()
    }).collect::<Vec<String>>();

    results.sort();

    // write to file results.md
    use std::fs::File;
    use std::io::prelude::*;
    let mut file = File::create("./results.txt").unwrap();
    for result in results {
        file.write_all(result.as_bytes()).unwrap();
        println!("{}", result);
    }

    // this code written by the ChatGPT & Github Copilot, I'm sorry :)

    let mut table: HashMap<i32, (&str, &str, &str, &str)> = HashMap::new();
    //read the results from the file results.txt
    // into data
    let mut file = File::open("./results.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();


    for line in contents.lines() {
        let mut parts = line.split(":");

        let day = parts.next().unwrap();
        let part = parts.next().unwrap();
        let value = parts.next().unwrap();

        let day_num: i32 = day.split_whitespace().nth(1).unwrap().parse().unwrap();
        let part_num: i32 = part.split_whitespace().nth(1).unwrap().parse().unwrap();
        let value_num = value.split_whitespace().nth(0).unwrap();

        let entry = table.entry(day_num).or_insert(("", "", "", ""));

        if part_num == 1 {
            if part.contains("test") {
                entry.0 = value_num;
            } else {
                entry.2 = value_num;
            }
        } else if part_num == 2 {
            if part.contains("test") {
                entry.1 = value_num;
            } else {
                entry.3 = value_num;
            }
        }
    }

    //write it into a file results.md
    let mut file = File::create("./results.md").unwrap();
    file.write_all("| Day | Part 1 Test | Part 2 Test | Part 1 | Part 2 |\r\n".as_bytes()).unwrap();
    file.write_all("|-----|-------------|-------------|--------|--------|\r\n".as_bytes()).unwrap();
    // collect into string vector
    let mut results = Vec::new();
    for (day, (part1_test, part2_test, part1, part2)) in table {
        let line = format!("| {}   | {}         | {}          | {}   | {}   |\r\n",
                           day, part1_test, part2_test, part1, part2);
        results.push(line);
    }
    results.sort();
    for result in results {
        file.write_all(result.as_bytes()).unwrap();
    }
}

