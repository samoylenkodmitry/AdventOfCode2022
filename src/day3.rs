/*
 */
use std::collections::HashSet;
use crate::day::Day;

pub(crate) struct Day3;

const TEST: &'static str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

impl Day for Day3 {
    /**
    --- Day 3: Rucksack Reorganization ---
    ----------

    One Elf has the important job of loading all of the [rucksacks](https://en.wikipedia.org/wiki/Rucksack) with supplies for the jungle journey. Unfortunately, that Elf didn't quite follow the packing instructions, and so a few items now need to be rearranged.

    Each rucksack has two large *compartments*. All items of a given type are meant to go into exactly one of the two compartments. The Elf that did the packing failed to follow this rule for exactly one item type per rucksack.

    The Elves have made a list of all of the items currently in each rucksack (your puzzle input), but they need your help finding the errors. Every item type is identified by a single lowercase or uppercase letter (that is, `a` and `A` refer to different types of items).

    The list of items for each rucksack is given as characters all on a single line. A given rucksack always has the same number of items in each of its two compartments, so the first half of the characters represent items in the first compartment, while the second half of the characters represent items in the second compartment.

    For example, suppose you have the following list of contents from six rucksacks:

    ```
    vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw

    ```

    * The first rucksack contains the items `vJrwpWtwJgWrhcsFMMfFFhFp`, which means its first compartment contains the items `vJrwpWtwJgWr`, while the second compartment contains the items `hcsFMMfFFhFp`. The only item type that appears in both compartments is lowercase `*p*`.
    * The second rucksack's compartments contain `jqHRNqRjqzjGDLGL` and `rsFMfFZSrLrFZsSL`. The only item type that appears in both compartments is uppercase `*L*`.
    * The third rucksack's compartments contain `PmmdzqPrV` and `vPwwTWBwg`; the only common item type is uppercase `*P*`.
    * The fourth rucksack's compartments only share item type `*v*`.
    * The fifth rucksack's compartments only share item type `*t*`.
    * The sixth rucksack's compartments only share item type `*s*`.

    To help prioritize item rearrangement, every item type can be converted to a *priority*:

    * Lowercase item types `a` through `z` have priorities 1 through 26.
    * Uppercase item types `A` through `Z` have priorities 27 through 52.

    In the above example, the priority of the item type that appears in both compartments of each rucksack is 16 (`p`), 38 (`L`), 42 (`P`), 22 (`v`), 20 (`t`), and 19 (`s`); the sum of these is `*157*`.

    Find the item type that appears in both compartments of each rucksack. *What is the sum of the priorities of those item types?*

    To begin, [get your puzzle input](3/input).

    Answer:

    You can also [Shareon [Twitter](https://twitter.com/intent/tweet?text=%22Rucksack+Reorganization%22+%2D+Day+3+%2D+Advent+of+Code+2022&url=https%3A%2F%2Fadventofcode%2Ecom%2F2022%2Fday%2F3&related=ericwastl&hashtags=AdventOfCode) [Mastodon](javascript:void(0);)] this puzzle.
     */
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

    fn get_day_number(&self) -> i32 {
        return 3;
    }
}