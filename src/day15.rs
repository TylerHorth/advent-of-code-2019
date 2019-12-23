use crate::intcode::computer::{Computer, RuntimeError};
use aoc_runner_derive::aoc;
use crossterm::style::{Colorize, Print, PrintStyledContent, Styler};
use crossterm::QueueableCommand;
use num::{Bounded, Num};
use pathfinding::directed::astar::astar;
use std::collections::{HashMap, HashSet};
use std::io::{stdout, Write};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn as_int(&self) -> i64 {
        match self {
            Self::North => 1,
            Self::South => 2,
            Self::West => 3,
            Self::East => 4,
        }
    }

    fn invert(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::East => Self::West,
        }
    }

    fn step(&self, (x, y): (i64, i64)) -> (i64, i64) {
        match self {
            Self::North => (x, y + 1),
            Self::South => (x, y - 1),
            Self::West => (x - 1, y),
            Self::East => (x + 1, y),
        }
    }

    const ALL: [Direction; 4] = [Self::North, Self::South, Self::East, Self::West];
}

fn dfs(
    start: (i64, i64),
    input: &Sender<i64>,
    output: &Receiver<i64>,
    map: &mut HashMap<(i64, i64), i64>,
) -> Option<(i64, i64)> {
    let mut found = None;

    for dir in &Direction::ALL {
        let next = dir.step(start);
        if map.contains_key(&next) {
            continue;
        }

        input.send(dir.as_int()).unwrap();

        let result = output.recv().unwrap();
        map.insert(next, result);

        if result == 2 {
            found = Some(next);
        }

        if result == 1 {
            if let Some(target) = dfs(next, input, output, map) {
                found = Some(target);
            }
        }

        if result != 0 {
            input.send(dir.invert().as_int()).unwrap();
            assert_eq!(1, output.recv().unwrap());
        }
    }

    found
}

fn get_bounds<'a, T>(positions: impl Iterator<Item = &'a (T, T)>) -> (T, T, T, T)
where
    T: 'a + Num + Bounded + PartialOrd + Clone,
{
    let mut bounds = (
        T::max_value(),
        T::max_value(),
        T::min_value(),
        T::min_value(),
    );

    for (x, y) in positions {
        if x < &bounds.0 {
            bounds.0 = x.clone()
        }
        if y < &bounds.1 {
            bounds.1 = y.clone()
        }
        if x > &bounds.2 {
            bounds.2 = x.clone()
        }
        if y > &bounds.3 {
            bounds.3 = y.clone()
        }
    }

    bounds
}

fn start_computer(program: &str) -> (Sender<i64>, Receiver<i64>) {
    let mut computer = Computer::load(program).unwrap();

    let (input, receiver) = channel();
    let (sender, output) = channel();

    computer.set_input(receiver);
    computer.set_output(sender);

    // Run computer, ignore error from closing input stream
    thread::spawn(move || assert_eq!(RuntimeError::InputError, computer.run().unwrap_err()));

    (input, output)
}

#[aoc(day15, part1)]
fn shortest_path(program: &str) -> usize {
    let (input, output) = start_computer(program);
    let map = &mut HashMap::new();
    let found = dfs((0, 0), &input, &output, map).unwrap();

    let (path, _) = astar(
        &(0, 0),
        |&point| {
            Direction::ALL
                .iter()
                .map(move |dir| dir.step(point))
                .filter(|point| map.get(&point).filter(|&&tile| tile != 0).is_some())
                .map(|point| (point, 1))
        },
        |&(x, y)| (x - found.0).abs() + (y - found.1).abs(),
        |point| point == &found,
    )
    .unwrap();

    let (min_x, min_y, max_x, max_y) = get_bounds(map.keys());

    let mut stdout = stdout();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let command = if x == 0 && y == 0 {
                PrintStyledContent("o".on_cyan().bold().white())
            } else if x == found.0 && y == found.1 {
                PrintStyledContent("X".on_cyan().bold().white())
            } else if path.contains(&(x, y)) {
                PrintStyledContent(".".on_cyan().bold().white())
            } else {
                match map.get(&(x, y)) {
                    Some(0) => PrintStyledContent("#".dark_grey()),
                    Some(1) => PrintStyledContent(".".dark_grey()),
                    _ => PrintStyledContent(" ".white()),
                }
            };

            stdout.queue(command).unwrap();
        }
        stdout.queue(Print("\n")).unwrap();
    }

    stdout.flush().unwrap();

    path.len() - 1
}

#[aoc(day15, part2)]
fn minutes_to_fill(program: &str) -> u32 {
    let (input, output) = start_computer(program);
    let map = &mut HashMap::new();
    let found = dfs((0, 0), &input, &output, map).unwrap();

    let mut filled = HashSet::new();
    let mut minutes = 0;

    filled.insert(found);

    loop {
        let mut to_fill = HashSet::new();
        for &air in &filled {
            for dir in &Direction::ALL {
                let neighbour = dir.step(air);

                if map.get(&neighbour).cloned().unwrap_or(0) == 0 {
                    continue
                }

                if !filled.contains(&neighbour) {
                    to_fill.insert(neighbour);
                }
            }
        }

        if to_fill.is_empty() {
            return minutes;
        }

        filled.extend(to_fill.drain());
        minutes += 1;
    }
}
