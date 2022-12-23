use std::borrow::Borrow;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::repeat;
use std::sync::{Arc, Mutex, RwLock};
use glium::vertex::MultiVerticesSource;
use rayon::iter::IntoParallelIterator;
use crate::day::Day;
use rayon::prelude::*;

pub(crate) struct Day18;


impl Day for Day18 {
    fn part1(&self, input: &str) -> String {
        let xyz = Self::parse(input);
        let mut xy_to_z: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
        let mut xz_to_y: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
        let mut yz_to_x: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
        let mut s = xyz.len() * 6;
        for v in xyz {
            let (x, y, z) = (v[0], v[1], v[2]);
            let xy = xy_to_z.entry((x, y)).or_insert_with(|| Vec::new());
            match xy
                .binary_search(&z) {
                Ok(pos) => {}
                Err(pos) => xy.insert(pos, z)
            }
            let xz = xz_to_y.entry((x, z)).or_insert_with(|| Vec::new());
            match xz
                .binary_search(&y) {
                Ok(pos) => {}
                Err(pos) => xz.insert(pos, y)
            }
            let yz = yz_to_x.entry((y, z)).or_insert_with(|| Vec::new());
            match yz
                .binary_search(&x) {
                Ok(pos) => {}
                Err(pos) => yz.insert(pos, x)
            }
        }

        Self::filter(&mut xy_to_z, &mut s);
        Self::filter(&mut xz_to_y, &mut s);
        Self::filter(&mut yz_to_x, &mut s);
            
        s.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let xyz = Self::parse(input);
        let mut max_x = i32::MIN;
        let mut min_x = i32::MAX;
        let mut max_y = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_z = i32::MIN;
        let mut min_z = i32::MAX;
        let mut hash_set: HashSet<(i32, i32, i32)> = HashSet::new();
        xyz.iter().for_each(|v| {
            max_x = max(max_x, v[0]);
            min_x = min(min_x, v[0]);
            max_y = max(max_y, v[1]);
            min_y = min(min_y, v[1]);
            max_z = max(max_z, v[2]);
            min_z = min(min_z, v[2]);
            hash_set.insert((v[0], v[1], v[2]));
        });
        max_x += 1;
        min_x -= 1;
        max_y += 1;
        min_y -= 1;
        max_z += 1;
        min_z -= 1;
        let mut deque: VecDeque<(i32, i32, i32)> = VecDeque::new();
        deque.push_back((min_x, min_y, min_z));
        // dfs until visit all cube, avoiding collision with hash_set
        let mut visited: HashSet<(i32, i32, i32)> = HashSet::new();
        let mut count_collisions = 0;
        while !deque.is_empty() {
            let (x, y, z) = deque.pop_front().unwrap();
            if visited.contains(&(x, y, z)) {
                continue;
            }
            if hash_set.contains(&(x, y, z)) {
                count_collisions += 1;
                continue;
            }
            visited.insert((x, y, z));
            if x - 1 >= min_x {
                deque.push_back((x - 1, y, z));
            }
            if x + 1 <= max_x {
                deque.push_back((x + 1, y, z));
            }
            if y - 1 >= min_y {
                deque.push_back((x, y - 1, z));
            }
            if y + 1 <= max_y {
                deque.push_back((x, y + 1, z));
            }
            if z - 1 >= min_z {
                deque.push_back((x, y, z - 1));
            }
            if z + 1 <= max_z {
                deque.push_back((x, y, z + 1));
            }
        }
        count_collisions.to_string()
    }

    fn get_test_data(&self) -> String {
        let a = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

        a.to_string()
        //"".to_string()
    }

    fn get_day_number(&self) -> i32 {
        return 18;
    }
}

impl Day18 {
    fn filter(ab_to_c: &mut HashMap<(i32, i32), Vec<i32>>, s: &mut usize) {
        for (k, v) in  ab_to_c.iter() {
            for i in 1..v.len() {
                if v[i] == v[i - 1] + 1 {
                    *s -= 2;
                }
            }
        }
    }

    fn parse(input: &str) -> Vec<Vec<i32>> {
        let xyz: Vec<Vec<i32>> = input.lines().map(|line| {
            line.split(",").map(|s| s.parse().unwrap()).collect()
        }).collect();
        xyz
    }
}