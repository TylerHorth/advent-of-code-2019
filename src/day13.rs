use crate::intcode::computer::Computer;
use aoc_runner_derive::aoc;
use crossterm::cursor::{
    Hide, MoveDown, MoveRight, MoveToPreviousLine, RestorePosition, SavePosition, Show,
};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{ExecutableCommand, QueueableCommand};
use itertools::Itertools;
use std::collections::HashMap;
use std::io::{stdout, Write};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn char_for_tile(tile_id: i64) -> char {
    match tile_id {
        0 => ' ',
        1 => '%',
        2 => '#',
        3 => '-',
        4 => 'o',
        _ => unreachable!(),
    }
}

#[aoc(day13, part1)]
fn num_block_tiles(program: &str) -> usize {
    let mut computer = Computer::load(program).unwrap();

    let (sender, receiver) = channel();

    computer.set_output(sender);
    computer.run().unwrap();
    drop(computer);

    let mut tiles = HashMap::new();

    for (x, y, tile_id) in receiver.iter().tuples() {
        tiles.insert((x, y), tile_id);
    }

    tiles.values().filter(|&&tile| tile == 2).count()
}

const WIDTH: u16 = 48;
const HEIGHT: u16 = 25;

#[aoc(day13, part2)]
fn play(program: &str) -> i64 {
    let mut computer = Computer::load(program).unwrap();

    computer.set(0, 2).unwrap();

    let (input, receiver) = channel();
    let (sender, output) = channel();
    let (joy_in, joy_out) = channel();

    computer.set_input(receiver);
    computer.set_output(sender);

    thread::spawn(move || computer.run().unwrap());
    thread::spawn(move || {
        for key in joy_out {
            thread::sleep(Duration::from_millis(16));
            input.send(key).unwrap();
        }
    });

    let mut stdout = stdout();

    stdout.queue(Hide).unwrap();

    for _ in 0..HEIGHT {
        stdout.queue(Print('\n')).unwrap();
    }

    stdout
        .queue(MoveToPreviousLine(HEIGHT))
        .unwrap()
        .queue(SavePosition)
        .unwrap()
        .flush()
        .unwrap();

    let mut score = 0;
    let mut paddle = None;

    for (x, y, tile_id) in output.iter().tuples() {
        stdout.queue(RestorePosition).unwrap();

        if tile_id == 3 && paddle.is_none() {
            paddle = Some(x);
        }

        if tile_id == 4 {
            if let Some(paddle_x) = paddle {
                joy_in.send((x - paddle_x).signum()).unwrap();
                paddle = Some(x);
            } else {
                joy_in.send(0).unwrap();
            }
        }

        if x == -1 && y == 0 {
            score = tile_id;
            stdout
                .queue(MoveRight(WIDTH))
                .unwrap()
                .queue(Clear(ClearType::UntilNewLine))
                .unwrap()
                .queue(Print(format!("score: {}", score)))
                .unwrap();
        } else {
            if x > 0 {
                stdout.queue(MoveRight(x as u16)).unwrap();
            }
            if y > 0 {
                stdout.queue(MoveDown(y as u16)).unwrap();
            }

            stdout.queue(Print(char_for_tile(tile_id))).unwrap();
        }

        stdout.flush().unwrap();
    }

    stdout
        .execute(RestorePosition)
        .unwrap()
        .execute(MoveDown(HEIGHT))
        .unwrap()
        .execute(Show)
        .unwrap();

    score
}
