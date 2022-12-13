use std::cmp::Ordering;

use crate::day::Day;

pub(crate) struct Day13;


impl Day for Day13 {
    fn part1(&self, input: &str) -> String {
        let mut count = 0;
        input.split("\n\n").enumerate().for_each(|(ind, pair_str)| {
            let (first_str, second_str) = pair_str.split_once("\n").unwrap();
            if cmp(first_str, second_str) <= 0 {
                count += ind + 1;
            }
        });
        count.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut packets: Vec<Packet> = input.lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let chrs: Vec<char> = line.chars().collect();
                parse(&chrs[1..chrs.len() - 1].to_vec())
            }).collect();
        packets.push(Packet::List(vec![Packet::Number(2)]));
        packets.push(Packet::List(vec![Packet::Number(6)]));
        packets.sort_by(|a, b| {
            let c = cmp_packets(a, b);
            if c == 0 {
                Ordering::Equal
            } else if c == 1 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
        let one = Self::binary_search(&mut packets, &Packet::List(vec![Packet::Number(2)]));
        let two = Self::binary_search(&mut packets, &Packet::List(vec![Packet::Number(6)]));

        ((one + 1) * (two + 1)).to_string()
    }

    fn get_test_data(&self) -> String {
        let a = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]".to_string();

        a
    }

    fn get_day_number(&self) -> i32 {
        return 13;
    }
}

impl Day13 {
    fn binary_search(packets: &mut Vec<Packet>, p1: &Packet) -> usize {
        let one = packets.binary_search_by(|a| {
            let c = cmp_packets(a, &p1);
            if c == 0 {
                Ordering::Equal
            } else if c == 1 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }).unwrap();
        one
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Packet {
    Empty,
    Number(i32),
    List(Vec<Packet>),
}

fn to_str(p: Packet) -> String {
    match p {
        Packet::Empty => "[]".to_string(),
        Packet::Number(n) => n.to_string(),
        Packet::List(v) => {
            let mut s = "[".to_string();
            v.iter().for_each(|p| {
                s.push_str(&to_str(p.clone()));
                s.push_str(",");
            });
            s.pop();
            s.push_str("]");
            s
        }
    }
}

fn parse(chrs: &Vec<char>) -> Packet {
    let mut list = vec![];
    let mut i = 0;
    while i < chrs.len() {
        let c = chrs[i];
        if c == '[' {
            let mut j = i + 1;
            let mut count = 1;
            while count > 0 {
                if chrs[j] == '[' {
                    count += 1;
                } else if chrs[j] == ']' {
                    count -= 1;
                }
                j += 1;
            }
            if j == i + 1 {
                list.push(Packet::Empty);
            } else {
                list.push(parse(&chrs[i + 1..j - 1].to_vec()));
            }
            i = j;
        } else if c == ',' {
            i += 1;
        } else {
            let mut value = chrs[i].to_digit(10).unwrap() as i32;
            let mut j = i + 1;
            while j < chrs.len() {
                if !chrs[j].is_digit(10) {
                    break;
                }
                value = value * 10 + chrs[j].to_digit(10).unwrap() as i32;
                j += 1;
            }
            list.push(Packet::Number(value));
            i = j;
        }
    }
    Packet::List(list)
}

// p1 > p2 return 1
// p1 < p2 return -1
// p1 == p2 return 0
fn cmp(s1: &str, s2: &str) -> i32 {
    let s1_chrs: Vec<char> = s1.chars().collect();
    let s2_chrs: Vec<char> = s2.chars().collect();
    let s1_packet = parse(&s1_chrs[1..s1_chrs.len() - 1].to_vec());
    let s2_packet = parse(&s2_chrs[1..s2_chrs.len() - 1].to_vec());

    cmp_packets(&s1_packet, &s2_packet)
}


// p1 > p2 return 1
// p1 < p2 return -1
// p1 == p2 return 0
fn cmp_packets(p1: &Packet, p2: &Packet) -> i32 {
    match &p1 {
        Packet::Number(n1) => {
            match &p2 {
                Packet::Empty => { return 1; }
                Packet::Number(n2) => {
                    if *n1 < *n2 {
                        return -1;
                    }
                    if *n1 > *n2 {
                        return 1;
                    }
                    return 0;
                }
                Packet::List(list2) => {
                    if list2.len() == 0 {
                        return 1;
                    }
                    return cmp_packets(
                        &Packet::List(vec![p1.clone()])
                        , p2,
                    );
                }
            }
        }
        Packet::Empty => {
            match &p2 {
                Packet::Empty => {
                    return 0;
                }
                Packet::Number(_) => {
                    return -1;
                }
                Packet::List(_) => {
                    return -1;
                }
            }
        }
        Packet::List(list1) => {
            match &p2 {
                Packet::Empty => { return 1; }
                Packet::Number(_) => {
                    if list1.len() == 0 {
                        return -1;
                    }
                    return cmp_packets(
                        p1,
                        &Packet::List(vec![p2.clone()])
                        ,
                    );
                }
                Packet::List(list2) => {
                    let mut i = 0;
                    while i < list1.len() && i < list2.len() {
                        let cmp = cmp_packets(&list1[i], &list2[i]);
                        if cmp == 0 {
                            i += 1;
                        } else if cmp == 1 {
                            return 1;
                        } else {
                            return -1;
                        }
                    }
                    if i < list1.len() {
                        return 1;
                    }
                    if i < list2.len() {
                        return -1;
                    }
                    return 0;
                }
            }
        }
    }
}
