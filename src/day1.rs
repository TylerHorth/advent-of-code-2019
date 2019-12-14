use aoc_runner_derive::{aoc, aoc_generator};
use nom::character::complete::{digit1, line_ending};
use nom::combinator::{map_res, opt, all_consuming};
use nom::IResult;
use nom::multi::many0;

fn parse_mass(input: &str) -> IResult<&str, u32> {
    let (input, num) = map_res(digit1, str::parse)(input)?;
    let (input, _) = opt(line_ending)(input)?;

    Ok((input, num))
}

#[aoc_generator(day1)]
fn parse_masses(input: &str) -> Vec<u32> {
    all_consuming(many0(parse_mass))(input).unwrap().1
}

#[aoc(day1, part1)]
fn calculate_fuel(masses: &[u32]) -> u32 {
    masses.iter()
        .map(|mass| mass / 3 - 2)
        .sum()
}

fn fuel_for_module(mut mass: u32) -> u32 {
    let mut total = 0;
    
    while mass > 0 {
        mass = (mass / 3).saturating_sub(2);
        total += mass;
    }

    total
}

#[aoc(day1, part2)]
fn calculate_fuel_for_fuel(masses: &[u32]) -> u32 {
    masses.iter()
        .cloned()
        .map(fuel_for_module)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_runner() {
        assert_eq!(2, calculate_fuel(&vec![12]));
        assert_eq!(2, calculate_fuel(&vec![14]));
        assert_eq!(3, calculate_fuel(&vec![15]));
        assert_eq!(7, calculate_fuel(&vec![12, 14, 15]));
    }

    #[test]
    fn test_generator() {
        assert_eq!(vec![32, 48], parse_masses("32\n48\n"));
        assert_eq!(vec![32, 48], parse_masses("32\n48"));
        assert_eq!(vec![32, 48], parse_masses("32\r\n48"));
    }
}