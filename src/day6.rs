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

fn height<'a>(of: &'a str, map: &'a HashMap<String, String>, dp: &mut HashMap<&'a str, u32>) -> u32 {
    if dp.contains_key(of) {
        dp[of]
    } else {
        let parent = &map[of];
        let height = height(parent, map, dp) + 1;

        dp.insert(of, height);

        height
    }
}

#[aoc(day6, part1)]
fn num_orbits(map: &HashMap<String, String>) -> u32 {
    let dp = &mut HashMap::new();
    dp.insert("COM", 1);

    let mut count = 0;
    for value in map.values() {
        count += height(value, map, dp);
    }

    count
}

#[aoc(day6, part2)]
fn transfers_required(map: &HashMap<String, String>) -> u32 {
    let dp = &mut HashMap::new();
    dp.insert("COM", 1);

    let santa_count = height("SAN", map, dp);

    let mut you: &str = &map["YOU"];
    let mut count = 0;
    while !dp.contains_key(you) {
        you = &map[you];
        count += 1;
    }

    let lca = dp[you];

    santa_count - lca + count - 1
}
