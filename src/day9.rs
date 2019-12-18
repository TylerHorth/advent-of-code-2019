use aoc_runner_derive::aoc;
use crate::intcode::computer::Computer;

#[aoc(day9, part1)]
#[aoc(day9, part2)]
fn boost(program: &str) -> &'static str {
    let mut computer = Computer::load(program).unwrap();
    computer.run().unwrap();

    "BOOST COMPLETE"
}
