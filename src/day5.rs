use crate::day::Day;

pub(crate) struct Day5;


impl Day for Day5 {
    fn part1(&self, input: &str) -> String {
        let mut stacks = Self::parse_table(input);

        let f = |from: usize, to: usize, count: usize| {
            let mut stack_from = stacks[from].clone();
            let mut stack_to = stacks[to].clone();
            for _ in 0..count {
                let x = stack_from.pop().unwrap();
                stack_to.push(x);
            }
            stacks[from] = stack_from;
            stacks[to] = stack_to;
        };

        Self::do_moves(input, f);

        Self::stacks_to_string(&mut stacks)
    }

    fn part2(&self, input: &str) -> String {
        let mut stacks = Self::parse_table(input);

        let f = |from: usize, to: usize, count: usize| {
            let mut stack_from = stacks[from].clone();
            let mut stack_to = stacks[to].clone();
            let mut stack_temp = Vec::new();
            for _ in 0..count {
                let x = stack_from.pop().unwrap();
                stack_temp.push(x);
            }
            for _ in 0..count {
                let x = stack_temp.pop().unwrap();
                stack_to.push(x);
            }
            stacks[from] = stack_from;
            stacks[to] = stack_to;
        };

        Self::do_moves(input, f);

        Self::stacks_to_string(&mut stacks)
    }

    fn get_test_data(&self) -> String {
        // read the input from a file day5_test.txt
        // and return it as a string
        use std::fs::File;
        use std::io::prelude::*;
        let mut file = File::open("./inputs/day5_test.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    }

    fn get_day_number(&self) -> i32 {
        return 5;
    }
}

impl Day5 {
    fn parse_table(input: &str) -> Vec<Vec<char>> {
        let table = input.lines().take_while(|line| {
            !line.is_empty()
        }).collect::<Vec<&str>>();
        let count_stacks = table.last().unwrap().split_whitespace().last()
            .map(|s| { s.parse::<usize>().unwrap() }).unwrap();
        // create a stack<char> for each column and put them in a vector
        let mut stacks: Vec<Vec<char>> = Vec::with_capacity(count_stacks);
        for _ in 0..count_stacks {
            stacks.push(Vec::new());
        }
        let mut row = (table.len() - 2) as i32;
        while row >= 0 {
            let line = table[row as usize].chars().collect::<Vec<char>>();
            // each 3 chars is a column, and they are separated by a space
            // like this: "[Z] [M] [P]"
            //             012345678910
            // we need only Z, M, P, they are at indexes 1, 5, 9
            let mut col = 1;
            for stack in stacks.iter_mut() {
                let x = line[col];
                // if the char is not a space, push it to the stack
                if x != ' ' {
                    stack.push(x);
                }
                col += 4;
            }

            row -= 1;
        }
        stacks
    }

    fn do_moves<F>(input: &str, mut f: F) where F: FnMut(usize, usize, usize) {
        input.lines().skip_while(|line| {
            !line.is_empty()
        }).skip(1)
            .for_each(|line| {
                let words = line.split_whitespace().collect::<Vec<&str>>();
                let count = words[1].parse::<usize>().unwrap();
                let from = words[3].parse::<usize>().unwrap() - 1;
                let to = words[5].parse::<usize>().unwrap() - 1;
                f(from, to, count);
            });
    }

    fn stacks_to_string(stacks: &Vec<Vec<char>>) -> String {
        let mut result = String::new();
        for stack in stacks {
            result.push(*stack.last().unwrap());
        }

        result
    }
}