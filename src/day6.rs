use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use nom::IResult;
use nom::character::complete::{char, alphanumeric1, line_ending};
use nom::sequence::separated_pair;
use nom::multi::separated_list;

fn orbit(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(alphanumeric1, char(')'), alphanumeric1)(input)
}

#[aoc_generator(day6)]
fn parse_map(map: &str) -> HashMap<String, String> {
    let (_, orbits) = separated_list(line_ending, orbit)(map).unwrap();

    orbits.into_iter()
        .map(|(a, b)| (b.to_string(), a.to_string()))
        .collect()
}

#[aoc(day6, part1)]
fn count_orbits(map: &HashMap<String, String>) -> u32 {
    let mut count = 0;
    for mut value in map.values() {
        count += 1;
        while value != "COM" {
            count += 1;
            value = &map[value];
        }
    }

    count
}
