use aoc_runner_derive::aoc;
use crate::intcode::computer::Computer;

fn execute(program: &str, a: i64, b: i64) -> i64 {
    let mut computer = Computer::load(program).unwrap();

    computer.set(1, a).unwrap();
    computer.set(2, b).unwrap();
    computer.run().unwrap();

    computer.get(0).unwrap()
}

#[aoc(day2, part1)]
fn run_program(program: &str) -> i64 {
    execute(program, 12, 2)
}

#[aoc(day2, part2)]
fn find_values(program: &str) -> i64 {
    for i in 0..=99 {
        for j in 0..=99 {
            if execute(program, i, j) == 19690720 {
                return 100 * i + j
            }
        }
    }

    panic!("No noun-verb combination found");
}
