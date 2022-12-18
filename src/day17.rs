use std::borrow::Borrow;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, RwLock};
use rayon::iter::IntoParallelIterator;
use crate::day::Day;
use rayon::prelude::*;

pub(crate) struct Day17;

impl Day for Day17 {
    fn part1(&self, input: &str) -> String {
        "".to_string()
    }

    fn part2(&self, input: &str) -> String {
        "".to_string()
    }

    fn get_test_data(&self) -> String {
        let a = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

        a.to_string()
        //"".to_string()
    }

    fn get_day_number(&self) -> i32 {
        return 17;
    }
}
