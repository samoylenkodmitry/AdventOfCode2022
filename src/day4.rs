use crate::day::Day;

pub(crate) struct Day4;

struct Range {
    from: i32,
    to: i32,
}

impl Range {
    fn new(from: i32, to: i32) -> Range {
        Range { from, to }
    }
    fn contains(&self, other: &Range) -> bool {
        self.from <= other.from && self.to >= other.to
    }
    /**
      [....] self
         [....]
    or
         [....] self
      [....]

     */
    fn overlaps(&self, other: &Range) -> bool {
        self.from <= other.from && self.to >= other.from
            || other.from <= self.from && other.to >= self.from
    }
}

impl Day for Day4 {
    fn part1(&self, input: &str) -> i32 {
        Self::parse(input, |two_ranges: &Vec<Range>| -> bool{
            two_ranges[0].contains(&two_ranges[1])
                || two_ranges[1].contains(&two_ranges[0])
        })
    }

    fn part2(&self, input: &str) -> i32 {
        Self::parse(input, |two_ranges: &Vec<Range>| -> bool{
            two_ranges[0].overlaps(&two_ranges[1])
                || two_ranges[1].overlaps(&two_ranges[0])
        })
    }

    fn get_test_data(&self) -> &str {
        "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
    }

    fn get_day_number(&self) -> i32 {
        return 4;
    }
}

impl Day4 {
    fn parse(input: &str, x: fn(&Vec<Range>) -> bool) -> i32 {
        input.lines()
            .map(|line| {
                line.split(",")
                    .map(|s_range| {
                        let from_to = s_range.split("-")
                            .map(|s| s.parse::<i32>().unwrap())
                            .collect::<Vec<i32>>();
                        Range::new(from_to[0], from_to[1])
                    })
                    .collect::<Vec<Range>>()
            })
            .filter(|two_ranges| {
                x(two_ranges)
            })
            .count() as i32
    }
}