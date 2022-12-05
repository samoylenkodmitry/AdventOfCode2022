use std::cmp::max;
use crate::day::Day;


pub(crate) struct Day1;

impl Day for Day1 {
    fn part1(&self, input: &str) -> String {
        // split lines by a blank line
        let elves = input.trim().split("\n\n");
        // iterate over the lines
        let mut max_calories = 0;
        for elf in elves {
            let elves_snacks = elf.split("\n");
            // count how much calories this elve has
            // iterate over it's lines
            let mut calories_sum = 0;
            for snack in elves_snacks {
                let calories = snack.parse::<i32>().unwrap();
                calories_sum += calories;
            }
            max_calories = max(max_calories, calories_sum);
        }
        return max_calories.to_string();
    }

    fn part2(&self, input: &str) -> String {
        // let's use priority queue
        use std::collections::BinaryHeap;
        let mut max_heap = BinaryHeap::new();
        let elves = input.trim().split("\n\n");
        for elf in elves {
            let elves_snacks = elf.split("\n");
            let mut calories_sum = 0;
            for snack in elves_snacks {
                let calories = snack.parse::<i32>().unwrap();
                calories_sum += calories;
            }

            // Rust, are you serious? I have to wrap each value in Reverse?
            max_heap.push(-calories_sum);

            if max_heap.len() > 3 {
                max_heap.pop();
            }
        }
        let sum: i32 = max_heap.iter().sum();
        return (-sum).to_string();
    }

    fn get_test_data(&self) -> String {
        "".to_string()
    }

    fn get_day_number(&self) -> i32 {
        return 1;
    }
}
