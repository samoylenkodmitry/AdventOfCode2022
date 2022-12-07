use std::cmp::min;
use std::collections::HashMap;
use crate::day::Day;

pub(crate) struct Day7;


impl Day for Day7 {
    fn part1(&self, input: &str) -> String {
        let path_to_size = Self::read_tree(input);
        // print contents of path_to_size
        let mut keys: Vec<String> = path_to_size.keys().map(|x| x.to_string()).collect();
        keys.sort();
        let mut sum = 0;
        keys.iter().for_each(|key| {
            let size = *path_to_size.get(key.as_str()).unwrap();
            if size <= 100_000 {
                sum += size;
            }
        });

        sum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let path_to_size = Self::read_tree(input);
        let mut keys: Vec<String> = path_to_size.keys().map(|x| x.to_string()).collect();
        keys.sort();
        let used = path_to_size["///"];
        let free = 70_000_000 - used;
        let mut min_to_remove = i32::MAX;
        keys.iter().for_each(|key| {
            let to_remove = *path_to_size.get(key.as_str()).unwrap();
            if free + to_remove >= 30_000_000 {
                min_to_remove = min(min_to_remove, to_remove);
            }
        });

        min_to_remove.to_string()
    }

    fn get_test_data(&self) -> String {
        "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k".to_string()
    }

    fn get_day_number(&self) -> i32 {
        return 7;
    }
}

impl Day7 {
    fn read_tree(input: &str) -> HashMap<String, i32> {
        let mut path: Vec<&str> = Vec::new();
        path.push("/");
        let mut path_to_size: HashMap<String, i32> = HashMap::new();

        input.lines().for_each(|line| {
            let mut parts = line.split_whitespace();
            let first = parts.next().unwrap();
            if first.starts_with("$") {
                let second = parts.next().unwrap();
                // command
                if second == "ls" {
                    // skip
                } else if second == "cd" {
                    // go into directory
                    let name = parts.next().unwrap();
                    if name == "/" {
                        path.clear();
                        path.push("/");
                    } else if name == ".." {
                        path.pop();
                    } else {
                        path.push(name);
                    }
                }
            } else {
                if first.starts_with("dir") {
                    // skip dir
                } else {
                    // add file
                    // first part is size
                    // second part is name
                    let size = first.parse::<i32>().unwrap();
                    let name = parts.next().unwrap();
                    let mut key = "/".to_string();
                    for p in path.iter() {
                        key.push_str(p);
                        key.push_str("/");
                        let key_copy = key.clone();
                        let current_size = path_to_size.get(key_copy.as_str()).unwrap_or(&0);
                        path_to_size.insert(key_copy, current_size + size);
                    }
                }
            }
        });
        path_to_size
    }
}
