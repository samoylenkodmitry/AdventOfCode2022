use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use rayon::iter::IntoParallelIterator;
use crate::day::Day;
use rayon::prelude::*;

pub(crate) struct Day16;

impl Day for Day16 {
    fn part1(&self, input: &str) -> String {
        let (valves, positions, conn_num, flows_num) = Self::parse(input);
        fn dfs(one: usize, two: usize, time: i32, conn: &Vec<Vec<usize>>, flows: &Vec<i32>, opened: &mut Vec<bool>, cache: &mut HashMap<(usize, usize, i32, Vec<bool>), i32>, min_depth: &mut usize) -> i32 {
            if time <= 0 {
                return 0;
            }
            if opened.iter().all(|v| *v) {
                return 0;
            }
            let my_pos = min(one, two);
            let el_pos = max(one, two);
            if let Some(v) = cache.get(&(my_pos, el_pos, time, (*opened).clone())) {
                return *v;
            }
            let me_do_not_open = opened[my_pos] || flows[my_pos] == 0;
            let el_do_not_open = opened[el_pos] || flows[el_pos] == 0;
            let both_do_not_open = me_do_not_open && el_do_not_open;

            // both open
            let both_open: i32;
            if me_do_not_open || el_do_not_open || el_pos == my_pos {
                both_open = 0;
            } else {
                assert!(!opened[my_pos]);
                assert!(!opened[el_pos]);
                opened[my_pos] = true;
                opened[el_pos] = true;
                both_open = (flows[my_pos] + flows[el_pos]) * (time - 1) + dfs(my_pos, el_pos, time - 1, conn, flows, opened, cache, min_depth);
                opened[my_pos] = false;
                opened[el_pos] = false;
            }
            let both_go = conn[my_pos].iter().map(|s| {
                conn[my_pos].iter().map(|s2| {
                    dfs(*s, *s2, time - 1, conn, flows, opened, cache, min_depth)
                }).max().unwrap()
            }).max().unwrap();
            let mut max_flow = max(both_open, both_go);

            if !both_do_not_open {
                max_flow = max(max_flow, conn[my_pos].iter().map(|s| {
                    if el_do_not_open {
                        assert!(!opened[my_pos]);
                        opened[my_pos] = true;
                        let me_open_and_el_go = flows[my_pos] * (time - 1) + dfs(my_pos, *s, time - 1, conn, flows, opened, cache, min_depth);
                        opened[my_pos] = false;
                        me_open_and_el_go
                    } else if me_do_not_open {
                        assert!(!opened[el_pos]);
                        opened[el_pos] = true;
                        let el_open_and_me_go = flows[el_pos] * (time - 1) + dfs(*s, el_pos, time - 1, conn, flows, opened, cache, min_depth);
                        opened[el_pos] = false;
                        el_open_and_me_go
                    } else {
                        assert!(!opened[my_pos]);
                        assert!(!opened[el_pos]);

                        opened[my_pos] = true;
                        let me_open_and_el_go = flows[my_pos] * (time - 1) + dfs(my_pos, *s, time - 1, conn, flows, opened, cache, min_depth);
                        opened[my_pos] = false;
                        opened[el_pos] = true;
                        let el_open_and_me_go = flows[el_pos] * (time - 1) + dfs(*s, el_pos, time - 1, conn, flows, opened, cache, min_depth);
                        opened[el_pos] = false;
                        max(me_open_and_el_go, el_open_and_me_go)
                    }
                }).max().unwrap());
            }

            cache.insert((my_pos, el_pos, time, (*opened).clone()), max_flow);
            max_flow
        }

        let mut visited = vec![false; valves.len()];
        let mut cache: HashMap<(usize, usize, i32, Vec<bool>), i32> = HashMap::new();
        println!("positions count {}, p*time", positions.len());
        let mut min_depth = usize::MAX;
        let max_flow = dfs(positions["AA"], positions["AA"], 26, &conn_num, &flows_num, &mut visited, &mut cache, &mut min_depth);
        panic!("max_flow: {}", max_flow);
        max_flow.to_string()
    }

    fn part2(&self, input: &str) -> String {
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
        println!("positions count {}, p*time", positions.len());
        let mut min_depth = usize::MAX;
        let max_flow = dfs(positions["AA"], 30, &conn_num, &flows_num, &mut visited, &mut cache, &mut min_depth);
        panic!("max_flow: {}", max_flow);
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
