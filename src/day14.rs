//#[macro_use]
//use glium::Surface;
//use glium::glutin;

use crate::day::Day;
pub(crate) struct Day14;

//extern crate glium;

impl Day for Day14 {
    fn part1(&self, input: &str) -> String {
        parse_and_simulate(input, 0, false).to_string()
    }

    fn part2(&self, input: &str) -> String {
        parse_and_simulate(input, 500, true).to_string()
    }

    fn get_test_data(&self) -> String {
        let a = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9".to_string();

        a
    }

    fn get_day_number(&self) -> i32 {
        return 14;
    }
}

fn parse_and_simulate(input: &str, wall_x_expand: usize, add_bottom: bool) -> i32 {
    let mut paths_vec: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut min_x = 500 as usize;
    let mut max_x = 500;
    let mut min_y = 0 as usize;
    let mut max_y = 0;

    input.lines().for_each(|line| {
        let mut line_vec: Vec<(usize, usize)> = Vec::new();
        line.split(" -> ").for_each(|pair| {
            let (x, y) = pair.split_once(",").unwrap();
            let (x, y): (usize, usize) = (x.parse().unwrap(), y.parse().unwrap());
            let x = x + wall_x_expand;

            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);
            line_vec.push((x, y));
        });
        paths_vec.push(line_vec);
    });
    if add_bottom {
        max_y += 2;
    }
    max_x += wall_x_expand;
    let sz_x = (max_x - min_x + 1) as usize;
    let sz_y = (max_y - min_y + 1) as usize;
    let mut xy: Vec<Vec<char>> = vec![vec!['.'; sz_x]; sz_y];
    for path in paths_vec {
        let mut prev = path[0];
        for i in 1..path.len() {
            let (x1, y1) = prev;
            let (x2, y2) = path[i];
            if x1 == x2 {
                for y in y1.min(y2)..=y1.max(y2) {
                    xy[(y - min_y) as usize][(x1 - min_x) as usize] = '#';
                }
            } else {
                for x in x1.min(x2)..=x1.max(x2) {
                    xy[(y1 - min_y) as usize][(x - min_x) as usize] = '#';
                }
            }
            prev = path[i];
        }
    }
    if add_bottom {
        for i in 0..sz_x {
            xy[sz_y - 1][i] = '#';
        }
    }
    xy[0 - min_y][500 - min_x + wall_x_expand] = '+';
    let mut sand_count = 0;
    let mut simulation_going = true;
    while simulation_going {
        // create a new sand unit
        let mut sand = (500 - min_x + wall_x_expand, 0 - min_y);
        if xy[sand.1][sand.0] == 'o' {
            // filled up
            break;
        }
        // simulate sand falling down
        let mut sand_falling = true;
        while sand_falling {
            let (x, y) = sand;
            if xy[y + 1][x] == '.' {
                // sand falls down
                sand = (x, y + 1);
            } else if xy[y + 1][x] == '#' || xy[y + 1][x] == 'o' {
                // can't fall down
                // check down-left
                if x == 0 {
                    // fall doun-left is out of bounds
                    simulation_going = false;
                    break;
                }
                if xy[y + 1][x - 1] == '#' || xy[y + 1][x - 1] == 'o' {
                    // can't fall down-left
                    // check down-right
                    if x == xy[0].len() - 1 {
                        // fall down-right is out of bounds
                        simulation_going = false;
                        break;
                    }
                    if xy[y + 1][x + 1] == '#' || xy[y + 1][x + 1] == 'o' {
                        // can't fall down-right
                        // stop falling
                        sand_falling = false;
                        xy[y][x] = 'o';
                        sand_count += 1;
                    } else {
                        // can fall down-right
                        // fall down-right
                        sand = (x + 1, y + 1);
                    }
                } else {
                    // can fall down-left
                    // fall down-left
                    sand = (x - 1, y + 1);
                }
            } else {
                panic!("Unknown character {}", xy[y + 1][x]);
            }

            // check if sand is out of bounds
            if sand.1 >= xy.len() || sand.0 >= xy[sand.1].len() {
                // out of bounds
                // stop falling
                sand_falling = false;
                simulation_going = false;
            }
        }
        print_map(&mut xy);
    }
    draw_map(&mut xy);
    sand_count
}

fn print_map(xy: &mut Vec<Vec<char>>) {
    if true {
        return;
    }
    //print the map
    for y in 0..xy.len() {
        for x in 0..xy[y].len() {
            print!("{}", xy[y][x]);
        }
        println!();
    }
    
}

fn draw_map(matrix: &mut Vec<Vec<char>>) {
    // someday I'll learn how to use a library for this
    
//    use glium::DisplayBuild;
//    use glium::glutin;
//    use glium::Surface;
//    // Create a new OpenGL window using glium
//    let display = glium::glutin::WindowBuilder::new()
//        .build_glium()
//        .unwrap();
//
//    // Iterate over each character in the matrix and draw it to the window
//    for (i, row) in matrix.iter().enumerate() {
//        for (j, character) in row.chars().enumerate() {
//            let target = display.draw();
//            target.clear_color(0.0, 0.0, 0.0, 1.0);
//            target.text(character,
//                        glium::text::FontTexture::new(&display,
//                                                      glium::text::FontAtlas::new(&display,
//                                                                                  glium::text::FontConfig::new()
//                                                                                      .color(glium::text::Color::new(1.0, 1.0, 1.0))
//                                                                                      .size(20))).unwrap(),
//                        glium::glutin::dpi::LogicalPosition::new(i as f64, j as f64));
//            target.finish().unwrap();
//        }
//    }
}

