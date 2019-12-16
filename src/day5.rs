use aoc_runner_derive::aoc;
use crate::intcode::computer::Computer;

#[aoc(day5, part1)]
#[aoc(day5, part2)]
fn run_diagnostics(program: &str) -> &'static str {
    let mut computer = Computer::load(program).unwrap();
    computer.run().unwrap();

    "Diagnostics complete"
}
