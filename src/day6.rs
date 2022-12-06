use crate::day::Day;

pub(crate) struct Day6;


impl Day for Day6 {
    fn part1(&self, input: &str) -> String {
        Self::find_unique_of_size(input, 4)
    }

    fn part2(&self, input: &str) -> String {
        Self::find_unique_of_size(input, 14)
    }

    fn get_test_data(&self) -> String {
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()
    }

    fn get_day_number(&self) -> i32 {
        return 6;
    }
}

impl Day6 {
    fn find_unique_of_size(input: &str, sz: usize) -> String {
        let mut ind = 0;
        let mut arr = vec!['.'; sz];
        let mut pos = 0;
        for x in input.chars() {
            arr[ind] = x;

            ind = (1 + ind) % sz;
            pos += 1;
            let mut have_dup = false;
            'outer:
            for i in 0..sz {
                for j in i + 1..sz {
                    if arr[i] == arr[j] || arr[j] == '.' {
                        have_dup = true;
                        break 'outer;
                    }
                }
            }
            if !have_dup {
                break;
            }
        }
        pos.to_string()
    }
}