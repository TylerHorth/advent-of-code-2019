use aoc_runner_derive::{aoc, aoc_generator};
use std::ops::RangeInclusive;
use nom::IResult;
use nom::combinator::map_res;
use nom::character::complete::digit1;
use nom::bytes::complete::tag;

fn number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

#[aoc_generator(day4)]
fn read_range(input: &str) -> RangeInclusive<u32> {
    let (input, start) = number(input).unwrap();
    let (input, _) = tag::<_,_,()>("-")(input).unwrap();
    let (_, end) = number(input).unwrap();

    start..=end
}

fn is_valid(pass: &u32) -> bool {
    let mut last = pass % 10;
    let mut remaining = pass / 10;
    let mut adjacent = false;
    while remaining != 0 {
        let next = remaining % 10;
        remaining = remaining / 10;

        if next == last {
            adjacent = true;
        }

        if next > last {
            return false;
        }

        last = next;
    }

    adjacent
}

fn is_valid2(pass: &u32) -> bool {
    let mut last = pass % 10;
    let mut remaining = pass / 10;
    let mut chain = 1;
    let mut adjacent = false;
    while remaining != 0 {
        let next = remaining % 10;
        remaining = remaining / 10;

        if next == last {
            chain += 1;
        } else {
            if chain == 2 {
                adjacent = true;
            }
            chain = 1;
        }

        if next > last {
            return false;
        }

        last = next;
    }

    adjacent || chain == 2
}

#[aoc(day4, part1)]
fn count_passwords(range: &RangeInclusive<u32>) -> usize {
    range.clone().filter(is_valid).count()
}

#[aoc(day4, part2)]
fn count_passwords2(range: &RangeInclusive<u32>) -> usize {
    range.clone().filter(is_valid2).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert!(is_valid(&112345));
        assert!(is_valid(&111111));
        assert!(!is_valid(&223450));
        assert!(!is_valid(&123789));
    }

    #[test]
    fn test_is_valid2() {
        assert!(is_valid2(&112345));
        assert!(!is_valid2(&111111));
        assert!(is_valid2(&112233));
        assert!(!is_valid2(&123444));
        assert!(is_valid2(&111122));
        assert!(is_valid2(&112222));
    }
}
