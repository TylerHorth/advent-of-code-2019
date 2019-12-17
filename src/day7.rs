use aoc_runner_derive::aoc;
use crate::intcode::computer::Computer;
use itertools::Itertools;
use std::sync::mpsc::channel;
use std::iter::repeat_with;

#[aoc(day7, part1)]
fn max_signal_sequence(program: &str) -> i64 {
    (0..=4).permutations(5)
        .map(|phases| {
            let computers = repeat_with(||
                Computer::load(program).unwrap()
            ).take(5);

            let mut in_out = channel();
            in_out.0.send(phases[0]).unwrap();
            in_out.0.send(0).unwrap();

            for (i, mut computer) in computers.enumerate() {
                computer.set_input(in_out.1);

                in_out = channel();

                if i < 4 {
                    in_out.0.send(phases[i + 1]).unwrap();
                }

                computer.set_output(in_out.0);

                computer.run().unwrap();
            }

            in_out.1.recv().unwrap()
        })
        .max()
        .unwrap()
}
