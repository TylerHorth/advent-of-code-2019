use aoc_runner_derive::aoc;
use crate::intcode::computer::Computer;
use std::thread;

enum Direction {
    Up,
    Down,
    Left,
    Right
}

use Direction::*;
use std::sync::mpsc::channel;
use std::collections::HashMap;

impl Direction {
    fn turn(self, dir: i64) -> Direction {
        match self {
            Up if dir == 0 => Left,
            Up if dir == 1 => Right,
            Down if dir == 0 => Right,
            Down if dir == 1 => Left,
            Left if dir == 0 => Down,
            Left if dir == 1 => Up,
            Right if dir == 0 => Up,
            Right if dir == 1 => Down,
            _ => unreachable!()
        }
    }

    fn next(&self, (x, y): (i64, i64)) -> (i64, i64) {
        match self  {
            Up => (x, y + 1),
            Down => (x, y - 1),
            Left => (x - 1, y),
            Right => (x + 1, y),
        }
    }
}

fn paint(program: &str, start_color: i64) -> HashMap<(i64, i64), i64> {
    let mut computer = Computer::load(program).unwrap();

    let (input, receiver) = channel();
    let (sender, output) = channel();

    computer.set_input(receiver);
    computer.set_output(sender);

    thread::spawn(move || computer.run().unwrap());

    let mut panels = HashMap::new();

    let mut dir = Direction::Up;
    let mut pos = (0, 0);

    input.send(start_color).unwrap();

    loop {
        let color = match output.recv() {
            Ok(c) => c,
            _ => break
        };

        panels.insert(pos, color);

        let turn = output.recv().ok().unwrap();
        dir = dir.turn(turn);
        pos = dir.next(pos);

        let color = panels
            .get(&pos)
            .cloned()
            .unwrap_or(0);

        if input.send(color).is_err() {
            break
        }
    }

    panels
}

#[aoc(day11, part1)]
fn count(program: &str) -> usize {
    let panels = paint(program, 0);
    panels.len()
}

#[aoc(day11, part2)]
fn show(program: &str) -> &'static str {
    let panels = paint(program, 1);

    let (mut min_x, mut min_y, mut max_x, mut max_y) =
        (std::i64::MAX, std::i64::MAX, std::i64::MIN, std::i64::MIN);

    for pos in panels.keys() {
        min_x = min_x.min(pos.0);
        min_y = min_y.min(pos.1);
        max_x = max_x.max(pos.0);
        max_y = max_y.max(pos.1);
    }

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let color = panels.get(&(x, y)).cloned().unwrap_or(0);
            if color == 0 {
                print!(" ");
            } else {
                print!("#");
            }
        }
        println!();
    }

    "PAINTING COMPLETE"
}
