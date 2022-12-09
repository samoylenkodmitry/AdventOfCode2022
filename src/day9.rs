use std::collections::HashSet;
use crate::day::Day;

pub(crate) struct Day9;


impl Day for Day9 {
    fn get_test_data(&self) -> String {
        let a = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2".to_string();
        let b = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20".to_string();
        a
    }

    fn part1(&self, input: &str) -> String {
        Self::simulate(input, 2).len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        Self::simulate(input, 10).len().to_string()
    }

    fn get_day_number(&self) -> i32 {
        return 9;
    }
}

impl Day9 {
    fn move_rope(rope: &mut Vec<(i32, i32)>, count: usize, visited: &mut HashSet<(i32, i32)>) {
        for i in 1..count {
            let mut tx = rope[i].0;
            let mut ty = rope[i].1;
            move_knot(&mut tx, &mut ty, rope[i - 1].0, rope[i - 1].1);
            rope[i] = (tx, ty);
        }
        visited.insert(rope[count - 1]);
        //Self::draw(&rope, false);
    }

    fn simulate(input: &str, count: usize) -> HashSet<(i32, i32)> {
        let mut rope: Vec<(i32, i32)> = Vec::new();
        for _ in 0..count {
            rope.push((0, 0));
        }
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        visited.insert((0, 0));
        input.lines()
            .for_each(|line| {
                let (dir, steps) = line.split_once(" ").unwrap();
                let steps = steps.parse::<i32>().unwrap();
                //println!("{} {}", dir, steps);
                match dir {
                    "R" => {
                        for _ in 0..steps {
                            rope[0].0 += 1;
                            Self::move_rope(&mut rope, count, &mut visited);
                        }
                    }
                    "L" => {
                        for _ in 0..steps {
                            rope[0].0 -= 1;
                            Self::move_rope(&mut rope, count, &mut visited);
                        }
                    }
                    "U" => {
                        for _ in 0..steps {
                            rope[0].1 += 1;
                            Self::move_rope(&mut rope, count, &mut visited);
                        }
                    }
                    "D" => {
                        for _ in 0..steps {
                            rope[0].1 -= 1;
                            Self::move_rope(&mut rope, count, &mut visited);
                        }
                    }
                    _ => panic!("Unknown direction {}", dir)
                }
            });
        //let x: Vec<(i32, i32)> = visited.iter().map(|x| *x).collect();
        //println!("tail");
        //Self::draw(&x, true);
        visited
    }
    fn draw(rope: &Vec<(i32, i32)>, single: bool) {
        let sz = 9;
        for y in (-sz..sz).rev() {
            for x in -sz..sz {
                let mut found = false;
                for i in 0..rope.len() {
                    if rope[i] == (x, y) {
                        if single {
                            print!("*");
                        } else {
                            print!("{}", i);
                        }
                        found = true;
                        break;
                    }
                }
                if !found {
                    if (x, y) == (0, 0) {
                        print!("s");
                    } else {
                        print!(".");
                    }
                }
            }
            println!();
        }
    }
}

fn move_knot(tx: &mut i32, ty: &mut i32, x: i32, y: i32) {
    move_linear(ty, y, tx, x);
    move_linear(tx, x, ty, y);
}

/*

......        ....H.
....H.        ....1.
....1.  -->   ..432.
.432..        .5....
5.....        6.....

....H.  ....H. ....H.  ....H.  ....H.  ....H.  
......  ....1. ....1.  ....1.  ....1.  ....1.  
....1.  ...... ....2.  ...32.  ..432.  ..432.  
.432..  .432.. .43...  .4....  ......  .5....  
5.....  5..... 5.....  5.....  5.....  6.....  
                                         ^
                                         \ diff was >1 for x and y

 */
fn move_linear(t: &mut i32, h: i32, to: &mut i32, o: i32) {
    //  h******t  h-t < 0
    //       <--

    //  t******h  h-t > 0
    //  -->
    let diff = h - *t;
    if diff > 1 {
        *t = h - 1;
        side_motion(to, o);
    } else if diff < -1 {
        *t = h + 1;
        side_motion(to, o);
    }
}

fn side_motion(to: &mut i32, o: i32) {
    if (*to - o).abs() > 1 {
        *to = o + (*to - o).signum();
    } else {
        *to = o;
    }
}

