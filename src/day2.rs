use aoc_runner_derive::aoc;
use crate::intcode::computer::Computer;

#[aoc(day2, part1)]
fn run_program(program: &str) -> i64 {
    let mut computer = Computer::load(program).unwrap();

    computer.set(1, 12).unwrap();
    computer.set(2, 2).unwrap();
    computer.run().unwrap();

    computer.get(0).unwrap()
}
