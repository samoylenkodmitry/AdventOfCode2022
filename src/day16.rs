use std::borrow::Borrow;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, RwLock};
use rayon::iter::IntoParallelIterator;
use crate::day::Day;
use rayon::prelude::*;

pub(crate) struct Day16;

impl Day for Day16 {
    fn part1(&self, input: &str) -> String {
        let (valves, positions, conn_num, flows_num) = Self::parse(input);
        fn dfs(pos: usize, time: i32, conn: &Vec<Vec<usize>>, flows: &Vec<i32>, opened: &mut Vec<bool>, cache: &mut HashMap<(usize, i32, Vec<bool>), i32>, min_depth: &mut usize) -> i32 {
            if time <= 0 {
                return 0;
            }
            if opened.iter().all(|v| *v) {
                return 0;
            }
            if let Some(v) = cache.get(&(pos, time, (*opened).clone())) {
                return *v;
            }
            let do_not_open = opened[pos] || flows[pos] == 0;

            let if_open: i32;
            if !do_not_open {
                opened[pos] = true;
                if_open = flows[pos] * (time - 1) + dfs(pos, time - 1, conn, flows, opened, cache, min_depth);
                opened[pos] = false;
            } else {
                if_open = 0;
            }
            let max_flow = max(if_open, conn[pos].iter().map(|s| {
                dfs(*s, time - 1, conn, flows, opened, cache, min_depth)
            }).max().unwrap());

            cache.insert((pos, time, (*opened).clone()), max_flow);
            max_flow
        }

        let mut visited = vec![false; valves.len()];
        let mut cache: HashMap<(usize, i32, Vec<bool>), i32> = HashMap::new();
        let mut min_depth = usize::MAX;
        let max_flow = dfs(positions["AA"], 30, &conn_num, &flows_num, &mut visited, &mut cache, &mut min_depth);
        max_flow.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (valves, positions, conn_num, flows_num) = Self::parse(input);
        fn dfs(one: usize, two: usize, time: i32, conn: &Vec<Vec<usize>>, flows: &Vec<i32>, opened: u64, cache: &mut HashMap<(usize, usize, i32, u64), i32>) -> i32 {
            if time <= 0 {
                return 0;
            }
            if opened == (1 << (1 + conn.len())) - 1 {
                return 0;
            }
            let my_pos = min(one, two);
            let el_pos = max(one, two);

            if let Some(&res) = cache.get(&(my_pos, el_pos, time, opened)) {
                return res;
            }
            let opened_el = (opened & (1 << el_pos)) != 0;
            let opened_my = (opened & (1 << my_pos)) != 0;
            let me_do_not_open = opened_my || flows[my_pos] == 0;
            let el_do_not_open = opened_el || flows[el_pos] == 0;
            let both_do_not_open = me_do_not_open && el_do_not_open;

            // both open
            let both_open: i32;
            if me_do_not_open || el_do_not_open || el_pos == my_pos {
                both_open = 0;
            } else {
                let opened = opened | (1 << my_pos) | (1 << el_pos);
                both_open = (flows[my_pos] + flows[el_pos]) * (time - 1) + dfs(my_pos, el_pos, time - 1, conn, flows, opened, cache);
            }
            let mut max_flow = both_open;

            if !both_do_not_open {
                if el_do_not_open {
                    let opened = opened | (1 << my_pos);
                    max_flow = max_flow.max(flows[my_pos] * (time - 1) + (&conn[el_pos]).iter().map(|el_next_pos| {
                        let me_open_and_el_go = dfs(my_pos, *el_next_pos, time - 1, conn, flows, opened, cache);
                        me_open_and_el_go
                    }).max().unwrap());
                } else if me_do_not_open {
                    let opened = opened | (1 << el_pos);
                    max_flow = max_flow.max(flows[el_pos] * (time - 1) + (&conn[my_pos]).iter().map(|my_next_pos| {
                        let el_open_and_me_go = dfs(*my_next_pos, el_pos, time - 1, conn, flows, opened, cache);
                        el_open_and_me_go
                    }).max().unwrap());
                } else {
                    let opened_1 = opened | (1 << my_pos);
                    max_flow = max_flow.max(flows[my_pos] * (time - 1) + (&conn[el_pos]).iter().map(|el_next_pos| {
                        let me_open_and_el_go = dfs(my_pos, *el_next_pos, time - 1, conn, flows, opened_1, cache);
                        me_open_and_el_go
                    }).max().unwrap());
                    let opened_2 = opened | (1 << el_pos);
                    max_flow = max_flow.max(flows[el_pos] * (time - 1) + (&conn[my_pos]).iter().map(|my_next_pos| {
                        let el_open_and_me_go = dfs(*my_next_pos, el_pos, time - 1, conn, flows, opened_2, cache);
                        el_open_and_me_go
                    }).max().unwrap());
                }
            }
            if both_open == 0 {
                let both_go = (&conn[my_pos]).iter().map(|my_next_pos| {
                    conn[el_pos].iter().map(|el_next_pos| {
                        dfs(*my_next_pos, *el_next_pos, time - 1, conn, flows, opened, cache)
                    }).max().unwrap()
                }).max().unwrap();
                max_flow = max_flow.max(both_go);
            }

            cache.insert((my_pos, el_pos, time, opened), max_flow);
            max_flow
        }

        let mut cache: HashMap<(usize, usize, i32, u64), i32> = HashMap::new();
        let max_flow = dfs(positions["AA"], positions["AA"], 26, &conn_num, &flows_num, 0 as u64, &mut cache);
        max_flow.to_string()
    }

    fn get_test_data(&self) -> String {
        let a = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

        a.to_string()
        //"".to_string()
    }

    fn get_day_number(&self) -> i32 {
        return 16;
    }
}

impl Day16 {
    fn parse(input: &str) -> (Vec<&str>, HashMap<&str, usize>, Vec<Vec<usize>>, Vec<i32>) {
        let mut valves: Vec<&str> = Vec::new();
        let mut flows: HashMap<&str, i32> = HashMap::new();
        let mut conn: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut positions: HashMap<&str, usize> = HashMap::new();
        input.lines().for_each(|line| {
            let (first, conn_str): (&str, &str);
            if line.contains("valves") {
                let (sfirst, sconn_str) = line.split_once("; tunnels lead to valves ").unwrap();
                first = sfirst;
                conn_str = sconn_str;
            } else {
                let (sfirst, sconn_str) = line.split_once("; tunnel leads to valve ").unwrap();
                first = sfirst;
                conn_str = sconn_str;
            }
            let (valve_str, flow_str) = first.split_once(" has flow rate=").unwrap();
            let valve = &valve_str[6..];
            let flow = flow_str.parse().unwrap();
            flows.insert(valve, flow);
            conn_str.split(", ").for_each(|c| {
                conn.entry(valve).or_insert(Vec::new()).push(c);
            });
            positions.insert(valve, valves.len());
            valves.push(valve);
        });
        let conn_num: Vec<Vec<usize>> = valves.iter().map(|v| {
            conn.get(v).unwrap().iter().map(|c| {
                positions.get(c).unwrap().clone()
            }).collect()
        }).collect();
        let flows_num: Vec<i32> = valves.iter().map(|v| {
            flows.get(v).unwrap().clone()
        }).collect();
        (valves, positions, conn_num, flows_num)
    }
}
