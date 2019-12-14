use aoc_runner_derive::{aoc, aoc_generator};
use nom::IResult;
use nom::branch::alt;
use nom::combinator::{map_res, all_consuming, opt};
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending};
use nom::multi::separated_list;
use std::collections::BTreeSet;
use std::ops::Bound;

#[derive(Debug, Eq, PartialEq)]
enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32)
}

fn number(input: &str) -> IResult<&str, i32> {
    map_res(digit1, str::parse)(input)
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, dir) = alt((
        tag("U"),
        tag("D"),
        tag("L"),
        tag("R")
    ))(input)?;

    let (input, num) = number(input)?;

    let m = match dir {
        "U" => Move::Up(num),
        "D" => Move::Down(num),
        "L" => Move::Left(num),
        "R" => Move::Right(num),
        _ => unreachable!()
    };

    Ok((input, m))
}

fn parse_path(input: &str) -> IResult<&str, Vec<Move>> {
    separated_list(tag(","), parse_move)(input)
}

#[aoc_generator(day3)]
fn parse_paths(input: &str) -> (Vec<Move>, Vec<Move>) {
    let (input, path1) = parse_path(input).unwrap();
    let (input, _)= line_ending::<_, ()>(input).unwrap();
    let (input, path2) = parse_path(input).unwrap();

    all_consuming::<_, _, (), _>(opt(line_ending))(input).unwrap();

    (path1, path2)
}

#[derive(Debug, Eq, PartialEq)]
enum Event {
    HStart(i32, i32),
    HEnd(i32, i32, i32),
    VLine(i32, i32, i32)
}

impl Event {
    fn get_x(&self) -> i32 {
        match self {
            Self::HStart(x, _) => *x,
            Self::HEnd(x, _, _) => *x,
            Self::VLine(x, _, _) => *x,
        }
    }
}

fn create_events(path: &[Move], h: &mut Vec<Event>, v: &mut Vec<Event>) {
    let mut cur = (0, 0);

    for m in path {
        match m {
            Move::Up(i) => {
                v.push(Event::VLine(cur.0, cur.1, cur.1 + *i));
                cur.1 += *i;
            },
            Move::Down(i) => {
                v.push(Event::VLine(cur.0, cur.1, cur.1 - *i));
                cur.1 -= *i;
            },
            Move::Left(i) => {
                cur.0 -= *i;
                h.push(Event::HStart(cur.0, cur.1));
                h.push(Event::HEnd(cur.0 + *i, cur.0, cur.1));
            },
            Move::Right(i) => {
                h.push(Event::HStart(cur.0, cur.1));
                h.push(Event::HEnd(cur.0 + *i, cur.0, cur.1));
                cur.0 += *i;
            }
        }
    }
}

fn distance_to_origin(p: (i32, i32)) -> i32 {
    p.0.abs().saturating_add(p.1.abs())
}

fn min_dist_intersection(events: &[Event]) -> i32 {
    // (y, x) pairs so they're ordered by the y axis
    let mut active = BTreeSet::new();
    let mut dist: i32 = i32::max_value();

    for event in events {
        match event {
            &Event::HStart(x, y) => {
                active.insert((y, x));
            },
            &Event::HEnd(_, x, y) => {
                assert!(active.remove(&(y, x)));
            },
            &Event::VLine(x, y1, y2) => {
                let range = (
                    Bound::Included((y1.min(y2), x)),
                    Bound::Included((y2.max(y1), x))
                );

                for &point in active.range(range) {
                    let intersection = (x, point.0);
                    let new_dist = distance_to_origin(intersection);
                    if new_dist != 0 {
                        dist = dist.min(new_dist);
                    }
                }
            }
        }
    }

    dist
}

#[aoc(day3, part1)]
fn find_closest_intersection((path1, path2): &(Vec<Move>, Vec<Move>)) -> i32 {
    let mut event_set_1 = Vec::new();
    let mut event_set_2 = Vec::new();

    create_events(&path1, &mut event_set_1, &mut event_set_2);
    create_events(&path2, &mut event_set_2, &mut event_set_1);

    event_set_1.sort_unstable_by_key(Event::get_x);
    event_set_2.sort_unstable_by_key(Event::get_x);

    let dist1 = min_dist_intersection(&event_set_1);
    let dist2 = min_dist_intersection(&event_set_2);

    dist1.min(dist2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(distance_to_origin((2, 4)), 6);
        assert_eq!(distance_to_origin((3, 3)), 6);
    }

    #[test]
    fn test_parse() {
        let test_data = "R8,U5,L5,D3\r\nU7,R6,D4,L4";
        let (path1, path2) = parse_paths(test_data);

        assert_eq!(path1, vec![
            Move::Right(8),
            Move::Up(5),
            Move::Left(5),
            Move::Down(3)
        ]);

        assert_eq!(path2, vec![
            Move::Up(7),
            Move::Right(6),
            Move::Down(4),
            Move::Left(4)
        ]);
    }

    #[test]
    fn test_events() {
        let mut h = Vec::new();
        let mut v = Vec::new();
        let path = parse_path("R8,U5,L5,D3").unwrap().1;

        create_events(&path, &mut h, &mut v);

        assert_eq!(h, vec![
            Event::HStart(0, 0),
            Event::HEnd(8, 0, 0),
            Event::HStart(3, 5),
            Event::HEnd(8, 3, 5)
        ]);

        assert_eq!(v, vec![
            Event::VLine(8, 0, 5),
            Event::VLine(3, 5, 2)
        ]);

        h.sort_by_key(Event::get_x);
        v.sort_by_key(Event::get_x);

        assert_eq!(h, vec![
            Event::HStart(0, 0),
            Event::HStart(3, 5),
            Event::HEnd(8, 0, 0),
            Event::HEnd(8, 3, 5)
        ]);

        assert_eq!(v, vec![
            Event::VLine(3, 5, 2),
            Event::VLine(8, 0, 5)
        ]);
    }

    #[test]
    fn test_intersection() {
        let events = vec![
            Event::VLine(0, 0, 7),
            Event::HStart(0, 0),
            Event::HStart(0, 7),
            Event::HStart(2, 3),
            Event::HStart(3, 5),
            Event::VLine(3, 5, 2),
            Event::VLine(6, 7, 3),
            Event::HEnd(6, 2, 3),
            Event::HEnd(6, 0, 7),
            Event::HEnd(8, 0, 0),
            Event::HEnd(8, 3, 5),
            Event::VLine(8, 0, 5),
        ];

        assert_eq!(min_dist_intersection(&events), 6);
    }
}