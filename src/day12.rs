use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, line_ending};
use nom::combinator::{map_res, recognize, opt, map};
use nom::multi::separated_list;
use nom::sequence::tuple;
use nom::IResult;
use num::integer::lcm;

fn int(input: &str) -> IResult<&str, i64> {
    map_res(recognize(tuple((opt(char('-')), digit1))), str::parse)(input)
}

fn pos(axis: char) -> impl Fn(&str) -> IResult<&str, i64> {
    move |input| {
        map(tuple((char(axis), char('='), int)), |r| r.2)(input)
    }
}

fn position(input: &str) -> IResult<&str, [i64; 3]> {
    let (input, (_, x, _, y, _, z, _)) = tuple((
        char('<'),
        pos('x'),
        tag(", "),
        pos('y'),
        tag(", "),
        pos('z'),
        char('>'),
    ))(input)?;

    Ok((input, [x, y, z]))
}

#[aoc_generator(day12)]
fn parse_positions(input: &str) -> Vec<[i64; 3]> {
    separated_list(line_ending, position)(input).unwrap().1
}

fn states_from(positions: &Vec<[i64; 3]>) -> [Vec<(i64, i64)>; 3] {
    let mut res = [
        Vec::with_capacity(positions.len()),
        Vec::with_capacity(positions.len()),
        Vec::with_capacity(positions.len())
    ];

    for position in positions {
        for i in 0..3 {
            res[i].push((position[i], 0));
        }
    }

    res
}

fn step(state: &mut Vec<(i64, i64)>) {
    for (i, j) in (0..state.len()).tuple_combinations() {
        if state[i].0 < state[j].0 {
            state[i].1 += 1;
            state[j].1 -= 1;
        }

        if state[i].0 > state[j].0 {
            state[i].1 -= 1;
            state[j].1 += 1;
        }
    }

    for moon in state {
        moon.0 += moon.1;
    }
}

#[aoc(day12, part1)]
fn total_energy(positions: &Vec<[i64; 3]>) -> i64 {
    let mut states = states_from(positions);

    for i in 0..3 {
        for _ in 0..1000 {
            step(&mut states[i]);
        }
    }

    (0..positions.len()).map(|i| {
        let (pot, kin) = states.iter()
            .map(|state| state[i])
            .map(|(a, b)| (a.abs(), b.abs()))
            .fold((0, 0), |(a, b), (c, d)| (a + c, b + d));

        pot * kin
    }).sum()
}

fn calc_period(start: &Vec<(i64, i64)>) -> u64 {
    let mut count = 0;
    let state = &mut start.clone();

    loop {
        step(state);

        count += 1;

        if state == start {
            return count;
        }
    }
}

#[aoc(day12, part2)]
fn find_period(positions: &Vec<[i64; 3]>) -> u64 {
    let states = states_from(positions);

    lcm(
        calc_period(&states[0]),
        lcm(calc_period(&states[1]), calc_period(&states[2])),
    )
}
