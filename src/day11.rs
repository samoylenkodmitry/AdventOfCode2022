use std::collections::{VecDeque};
use crate::day::Day;

pub(crate) struct Day11;

enum OpMode {
    Multiply,
    Power,
    Add,
}

struct Op {
    value: u128,
    op_mode: OpMode,
}

struct Monkey {
    ind: usize,
    operation: Op,
    test: u128,
    next_true: usize,
    next_false: usize,
}

impl Day for Day11 {
    fn part1(&self, input: &str) -> String {
        Self::process(input, 3, 20).to_string()
    }

    fn part2(&self, input: &str) -> String {
        Self::process(input, 1, 10_000).to_string()
    }

    fn get_test_data(&self) -> String {
        let a = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1".to_string();

        a
    }

    fn get_day_number(&self) -> i32 {
        return 11;
    }
}

impl Day11 {
    fn parse_input(input: &str, mut monkeys: &mut Vec<Monkey>, mut all_monkeys_items: &mut Vec<VecDeque<u128>>) {
        let s_monkeys = input.split("\n\n");
        s_monkeys.enumerate().for_each(|(ind, s_monkey)| {
            let mut lines = s_monkey.lines();
            let num_str = lines.next().unwrap().get(7..).unwrap();
            let items_str = lines.next().unwrap().split_once(": ").unwrap().1;
            let items: VecDeque<u128> = items_str.split(", ").map(|s| s.parse().unwrap()).collect();
            let operation_str = lines.next().unwrap().split_once(" = ").unwrap().1;
            let operation: Op;
            if operation_str.starts_with("old * old") {
                operation = Op { value: 0, op_mode: OpMode::Power }
            } else if operation_str.starts_with("old + ") {
                operation = Op { value: operation_str.get(6..).unwrap().parse().unwrap(), op_mode: OpMode::Add }
            } else if operation_str.starts_with("old * ") {
                operation = Op { value: operation_str.get(6..).unwrap().parse().unwrap(), op_mode: OpMode::Multiply }
            } else {
                panic!("Unknown operation: {}", operation_str);
            }
            let test_str = lines.next().unwrap().get(21..).unwrap();
            let test: u128 = test_str.parse().unwrap();
            let next_true_str = lines.next().unwrap().get(29..).unwrap();
            let next_true: usize = next_true_str.parse().unwrap();
            let next_false: usize = lines.next().unwrap().get(30..).unwrap().parse().unwrap();
            all_monkeys_items.push(items);
            let monkey = Monkey {
                ind,
                operation,
                test,
                next_true,
                next_false,
            };
            monkeys.push(monkey)
        });
    }

    fn process(input: &str, div_by: u128, rounds: i32) -> u128 {
        let mut monkeys: Vec<Monkey> = Vec::new();
        let mut all_monkeys_items: Vec<VecDeque<u128>> = Vec::new();
        Self::parse_input(input, &mut monkeys, &mut all_monkeys_items);
        
        let common_test = monkeys.iter().map(|monkey| monkey.test).product::<u128>();
        
        let mut counts: Vec<u128> = vec![0; monkeys.len()];
        for round in 0..rounds {
            for monkey in &monkeys {
                while !(&all_monkeys_items[monkey.ind]).is_empty() {
                    counts[monkey.ind] += 1;
                    let item = (&mut all_monkeys_items[monkey.ind]).pop_front().unwrap();

                    let new_item = match monkey.operation.op_mode {
                        OpMode::Multiply => item * monkey.operation.value,
                        OpMode::Power => item * item,
                        OpMode::Add => item + monkey.operation.value,
                    };
                    let new_value = if div_by > 1 { new_item / div_by } else { new_item % common_test };
                    if new_value % monkey.test == 0 {
                        all_monkeys_items[monkey.next_true].push_back(new_value);
                    } else {
                        all_monkeys_items[monkey.next_false].push_back(new_value);
                    }
                }
            }
        }
        counts.sort();
        counts.reverse();
        let res = counts[0] * counts[1];
        res
    }
}
