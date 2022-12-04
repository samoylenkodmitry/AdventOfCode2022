use crate::day::Day;

pub(crate) struct Day3;

impl Day for Day3 {
    fn part1(&self, input: &str) -> i32 {
        let mut sum = 0;
        let a = 'a' as i32;
        let A = 'A' as i32 - 26;
        for line in input.lines() {
            let len = line.len();
            let half = len >> 1;
            let mut arr = [false; 128];
            let bstr = line.as_bytes();
            for i in 0..len {
                let c = bstr[i] as i32;
                if i < half {
                    arr[c as usize] = true;
                } else {
                    if arr[c as usize] {
                        sum += 1 + c - if c >= a { a } else { A };
                        break;
                    }
                }
            }
        }
        sum
    }

    fn part2(&self, input: &str) -> i32 {
        let a = 'a' as i32;
        let A = 'A' as i32 - 26;
        let mut sum = 0;
        let lines = input.lines();
        let mut ind = 0;
        let mut arr = [0; 128];
        let mut seen = [false; 128];
        for line in lines {
            seen.fill(false);
            let bstr = line.as_bytes();
            for j in 0..bstr.len() {
                let c = bstr[j] as i32;
                let i = c as usize;
                if !seen[i] {
                    seen[i] = true;
                    arr[i] += 1;
                    if arr[i] == 3 && ((ind + 1) % 3) == 0 {
                        sum += 1 + c - if c >= a { a } else { A };
                    }
                }
            }

            if (ind + 1) % 3 == 0 {
                arr.fill(0);
            }
            ind += 1;
        }
        sum
    }

    fn get_test_data(&self) -> &str {
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
    }

    fn get_day_number(&self) -> i32 {
        return 3;
    }
}