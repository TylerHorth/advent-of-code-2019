use aoc_runner_derive::{aoc, aoc_generator};
use crossterm::cursor::{Hide, MoveUp, Show};
use crossterm::style::{Colorize, Print, PrintStyledContent, Styler};
use crossterm::{ExecutableCommand, QueueableCommand};
use nom::branch::alt;
use nom::character::complete::{char, line_ending};
use nom::multi::{many1, separated_list};
use nom::IResult;
use num::integer::gcd;
use std::collections::HashSet;
use std::f32::consts::PI;
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

fn asteroid(input: &str) -> IResult<&str, bool> {
    let (input, ast) = alt((char('.'), char('#')))(input)?;

    Ok((input, ast == '#'))
}

#[aoc_generator(day10)]
fn parse_map(input: &str) -> Vec<Vec<bool>> {
    separated_list(line_ending, many1(asteroid))(input)
        .unwrap()
        .1
}

fn asteroids_detected(map: &Vec<Vec<bool>>, (x, y): (usize, usize)) -> u32 {
    let mut count = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let dy = i as i32 - y as i32;
            let dx = j as i32 - x as i32;

            if gcd(dx, dy) == 1 {
                let mut y = y as i32;
                let mut x = x as i32;

                loop {
                    y += dy;
                    x += dx;

                    let ast = map.get(y as usize).and_then(|line| line.get(x as usize));

                    match ast {
                        // Asteroid found, increment and stop search
                        Some(true) => {
                            count += 1;
                            break;
                        }
                        // No asteroid found, continue searching in line of sight
                        Some(false) => {
                            continue;
                        }
                        // End of map
                        None => {
                            break;
                        }
                    }
                }
            }
        }
    }

    count
}

fn find_placement(map: &Vec<Vec<bool>>) -> ((usize, usize), u32) {
    let mut max_count = 0;
    let mut pos = (0, 0);

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] {
                let count = asteroids_detected(map, (x, y));

                if count > max_count {
                    max_count = count;
                    pos = (x, y);
                }
            }
        }
    }

    (pos, max_count)
}

#[aoc(day10, part1)]
fn max_asteroids_detected(map: &Vec<Vec<bool>>) -> u32 {
    find_placement(map).1
}

fn angle((x, y): (i32, i32)) -> f32 {
    let a = (x as f32).atan2(-y as f32);

    if a >= 0.0 {
        a
    } else {
        a + 2.0 * PI
    }
}

fn print_map(
    map: &Vec<Vec<bool>>,
    term: &mut impl Write,
    x: i32,
    y: i32,
    laser: &(usize, usize),
    found: &HashSet<(i32, i32)>,
) {
    for j in 0..map.len() {
        for i in 0..map[0].len() {
            if laser.0 == i && laser.1 == j {
                term.queue(PrintStyledContent("X".cyan())).unwrap();
            } else if i == x as usize && j == y as usize {
                term.queue(PrintStyledContent("#".red())).unwrap();
            } else if found.contains(&(i as i32, j as i32)) {
                term.queue(PrintStyledContent("#".green())).unwrap();
            } else if map[j][i] {
                term.queue(PrintStyledContent("#".reset())).unwrap();
            } else {
                term.queue(PrintStyledContent(".".reset())).unwrap();
            }
        }
        term.queue(Print("\n")).unwrap();
    }
    term.flush().unwrap();
}

#[aoc(day10, part2)]
fn find_200th(map: &Vec<Vec<bool>>) -> i32 {
    let laser = find_placement(map).0;

    let mut angles = Vec::new();

    for j in 0..map.len() {
        for i in 0..map[0].len() {
            let y = j as i32 - laser.1 as i32;
            let x = i as i32 - laser.0 as i32;

            if !(laser.0 == x as usize && laser.1 == y as usize) && gcd(x, y) == 1 {
                angles.push((x, y));
            }
        }
    }

    angles.sort_unstable_by(|&a, &b| angle(a).partial_cmp(&angle(b)).unwrap());

    let count = angles.len();
    let mut found = HashSet::new();
    let mut term = stdout();

    term.queue(Hide).unwrap();
    print_map(
        map,
        &mut term,
        laser.0 as i32,
        laser.1 as i32,
        &laser,
        &found,
    );

    for i in 0.. {
        let point = &angles[i % count];

        let mut y = laser.1 as i32;
        let mut x = laser.0 as i32;

        loop {
            y += point.1;
            x += point.0;

            let ast = map.get(y as usize).and_then(|line| line.get(x as usize));

            match ast {
                Some(true) if !found.contains(&(x, y)) => {
                    term.queue(MoveUp(map.len() as u16)).unwrap();
                    print_map(map, &mut term, x, y, &laser, &found);
                    sleep(Duration::from_millis(100));

                    if found.len() == 199 {
                        term.execute(Show).unwrap();
                        return x * 100 + y;
                    } else {
                        found.insert((x, y));
                        break;
                    }
                }
                Some(_) => {
                    continue;
                }
                None => {
                    break;
                }
            }
        }
    }

    unreachable!()
}
