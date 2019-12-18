use aoc_runner_derive::aoc;
use crate::intcode::computer::Computer;
use itertools::Itertools;
use std::sync::mpsc::channel;
use std::iter::repeat_with;
use std::thread;
use std::thread::JoinHandle;

const NUM_COMPUTERS: usize = 5;

fn max_signal(program: &str, phases: impl Iterator<Item=i64>) -> i64 {
    phases.permutations(NUM_COMPUTERS)
        .map(|phases| {
            let mut computers = repeat_with(|| Computer::load(program).unwrap())
                .take(NUM_COMPUTERS)
                .collect_vec();

            for i in 0..NUM_COMPUTERS {
                let (sender, receiver) = channel();

                sender.send(phases[i]).unwrap();

                computers[i].set_output(sender);
                computers[(i + 1) % NUM_COMPUTERS].set_input(receiver);
            }

            computers
                .last().unwrap()
                .clone_output().unwrap()
                .send(0).unwrap();

            let inputs = computers.into_iter()
                .map(|mut computer| thread::spawn(move || {
                    computer.run().unwrap();
                    computer.take_input().unwrap()
                }))
                .collect_vec()
                .into_iter()
                .map(JoinHandle::join)
                .collect::<Result<Vec<_>, _>>()
                .unwrap();

            inputs
                .first().unwrap()
                .recv().unwrap()
        })
        .max()
        .unwrap()
}

#[aoc(day7, part1)]
fn max_signal_sequence(program: &str) -> i64 {
    max_signal(program, 0..=4)
}

#[aoc(day7, part2)]
fn max_signal_loop(program: &str) -> i64 {
    max_signal(program, 5..=9)
}
