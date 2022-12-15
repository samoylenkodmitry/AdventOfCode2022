use std::collections::HashSet;
use crate::day::Day;

pub(crate) struct Day15;

impl Day for Day15 {
    fn part1(&self, input: &str) -> String {
        let (sensors, dists, min_x, max_x, min_y, test_y, excluded, exp_x, exp_y) = Self::parse(input);

        let sz_x = (max_x - min_x + 1) as usize;
        let mut count: i32 = 0;
        count -= excluded.len() as i32;

        let y = test_y;
        for x in 0..sz_x {
            for i in 0..sensors.len() {
                let dist = ((sensors[i].0 - (x as i32 + min_x))).abs() + ((sensors[i].1 - (y as i32 + min_y))).abs();
                if dist <= dists[i] {
                    count += 1;
                    break;
                }
            }
        }

        count.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (sensors, dists, min_x, max_x, min_y, test_y, excluded, exp_x, exp_y) = Self::parse(input);

        let border: i32 = 4_000_000;
        let mut s_x: u128 = 0;
        let mut s_y: u128 = 0;
        for i in 0..sensors.len() {
            // iterate over borders
            let (sx, sy) = sensors[i];
            let dist = dists[i] + 1;
            let left = (sx - dist, sy);
            let top = (sx, sy - dist);
            let right = (sx + dist, sy);
            let bottom = (sx, sy + dist);
            // from left to top
            let mut x = left.0;
            let mut y = left.1;
            'out:
            while x <= sx {
                if 0 + exp_x <= x && x <= border + exp_x && 0 + exp_y <= y && y <= border + exp_y {
                    let seen = Self::check_all(&sensors, &dists, i, x, y);
                    if !seen {
                        s_x = x as u128 - exp_x as u128;
                        s_y = y as u128 - exp_y as u128;
                        break 'out;
                    }
                }
                x += 1;
                y += 1;
            }
            // from top to right
            let mut x = top.0;
            let mut y = top.1;
            'out:
            while y >= sy {
                if 0 + exp_x <= x && x <= border + exp_x && 0 + exp_y <= y && y <= border + exp_y {
                    let seen = Self::check_all(&sensors, &dists, i, x, y);
                    if !seen {
                        s_x = x as u128 - exp_x as u128;
                        s_y = y as u128 - exp_y as u128;
                        break 'out;
                    }
                }
                x += 1;
                y -= 1;
            }
            // from right to bottom
            let mut x = right.0;
            let mut y = right.1;
            'out:
            while x >= sx {
                if 0 + exp_x <= x && x <= border + exp_x && 0 + exp_y <= y && y <= border + exp_y {
                    let seen = Self::check_all(&sensors, &dists, i, x, y);
                    if !seen {
                        s_x = x as u128 - exp_x as u128;
                        s_y = y as u128 - exp_y as u128;
                        break 'out;
                    }
                }
                x -= 1;
                y -= 1;
            }
            // from bottom to left
            let mut x = bottom.0;
            let mut y = bottom.1;
            'out:
            while y <= sy {
                if 0 + exp_x <= x && x <= border + exp_x && 0 + exp_y <= y && y <= border + exp_y {
                    let seen = Self::check_all(&sensors, &dists, i, x, y);
                    if !seen {
                        s_x = x as u128 - exp_x as u128;
                        s_y = y as u128 - exp_y as u128;
                        break 'out;
                    }
                }
                x -= 1;
                y += 1;
            }
        }
        let fr = s_x * 4_000_000 + s_y;
        fr.to_string()
    }

    fn get_test_data(&self) -> String {
        let a = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3".to_string();

        a.to_string()
    }

    fn get_day_number(&self) -> i32 {
        return 15;
    }
}

impl Day15 {
    fn check_all(sensors: &Vec<(i32, i32)>, dists: &Vec<i32>, i: usize, x: i32, y: i32) -> bool {
        let mut seen = false;
        for j in 0..sensors.len() {
            if j != i {
                let sx = sensors[j].0;
                let sy = sensors[j].1;
                let dist = (sx - x).abs() + (sy - y).abs();
                if dist <= dists[j] {
                    seen = true;
                    break;
                }
            }
        }
        seen
    }

    fn parse(input: &str) -> (Vec<(i32, i32)>, Vec<i32>, i32, i32, i32, usize, HashSet<(usize, usize)>, i32, i32) {
        let mut sensors: Vec<(i32, i32)> = Vec::new();
        let mut dists: Vec<i32> = Vec::new();
        let mut min_x: i32 = 0;
        let mut max_x: i32 = 0;
        let mut min_y: i32 = 0;
        let mut max_y: i32 = 0;
        let mut exp_x: i32 = 1871909;
        let mut exp_y: i32 = 50;
        let test_y = (2_000_000 + exp_y) as usize;
        let mut excluded: HashSet<(usize, usize)> = HashSet::new();

        input.lines().for_each(|line| {
            let (sensor_str, beacon_str) = line.split_once(": closest beacon is at x=").unwrap();
            let (_unused, sensor_str) = sensor_str.split_once(" at x=").unwrap();
            let (sensor_x_str, sensor_y_str) = sensor_str.split_once(", y=").unwrap();
            let (beacon_x_str, beacon_y_str) = beacon_str.split_once(", y=").unwrap();
            let (sensor_x, sensor_y): (i32, i32) = (sensor_x_str.parse().unwrap(), sensor_y_str.parse().unwrap());
            let (beacon_x, beacon_y): (i32, i32) = (beacon_x_str.parse().unwrap(), beacon_y_str.parse().unwrap());
            let sensor_x = sensor_x + exp_x;
            let sensor_y = sensor_y + exp_y;
            let beacon_x = beacon_x + exp_x;
            let beacon_y = beacon_y + exp_y;
            sensors.push((sensor_x, sensor_y));

            min_x = min_x.min(sensor_x);
            max_x = max_x.max(sensor_x);
            min_y = min_y.min(sensor_y);
            max_y = max_y.max(sensor_y);

            min_x = min_x.min(beacon_x);
            max_x = max_x.max(beacon_x);
            min_y = min_y.min(beacon_y);
            max_y = max_y.max(beacon_y);

            let dist = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs();
            if sensor_y == test_y as i32 {
                excluded.insert((sensor_x as usize, sensor_y as usize));
            }
            if beacon_y == test_y as i32 {
                excluded.insert((beacon_x as usize, beacon_y as usize));
            }
            dists.push(dist);
        });
        max_x += exp_x;
        max_y += exp_y;
        (sensors, dists, min_x, max_x, min_y, test_y, excluded, exp_x, exp_y)
    }
}
