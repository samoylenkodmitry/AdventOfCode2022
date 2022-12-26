use std::borrow::Borrow;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::Read;
use std::iter::repeat;
use std::str::Chars;
use std::sync::{Arc, Mutex, RwLock};
use glium::vertex::MultiVerticesSource;
use rayon::iter::IntoParallelIterator;
use crate::day::Day;
use rayon::prelude::*;

pub(crate) struct Day25;


impl Day for Day25 {
    fn part1(&self, input: &str) -> String {
        let mut sum: i64 = 0;
        input.lines().for_each(|line| {
            sum += snafu_to_decimal(line);
        });
        decimal_to_snafu(sum).to_string()
    }


    fn part2(&self, input: &str) -> String {
        "0".to_string()
    }

    fn get_test_data(&self) -> String {
        let a = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

        a.to_string()
    }

    fn get_day_number(&self) -> i32 {
        return 25;
    }
}


fn snafu_to_decimal(snafu: &str) -> i64 {
    let digits: Vec<i64> = snafu.bytes().map(|c| {
        match c {
            b'1' => 1,
            b'0' => 0,
            b'2' => 2,
            b'=' => -2,
            b'-' => -1,

            _ => panic!("Unknown char {}", c as char)
        }
    }).collect();
    let mut d = 0;
    for i in 0..digits.len() {
        let pow = 5_i64.pow((digits.len() - i - 1) as u32);
        d += digits[i] * pow
    }
    d
}

fn decimal_to_snafu(d: i64) -> String {
    let mut s = String::new();
    let mut d = d;
    while d > 0 {
        let remainder = d % 5;
        let c = match remainder {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            _ => panic!("Unknown remainder {}", remainder)
        };
        if remainder > 2 {
            d += 5;
        }
        s.push(c);
        d = d / 5;
    }
    s.chars().rev().collect()
}
