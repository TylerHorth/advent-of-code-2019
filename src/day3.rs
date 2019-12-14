use aoc_runner_derive::{aoc, aoc_generator};
use nom::IResult;
use nom::branch::alt;
use nom::combinator::{map_res, all_consuming, opt};
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending};
use nom::multi::separated_list;
use std::collections::BTreeMap;
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
    HStart {
        x: i32,
        y: i32,
        dist: i32,
        dir: i32
    },
    HEnd {
        x: i32,
        x_orig: i32,
        y: i32
    },
    VLine {
        x: i32,
        y1: i32,
        y2: i32,
        dist: i32
    },
}

impl Event {
    fn get_x(&self) -> i32 {
        match self {
            &Self::HStart { x, .. } => x,
            &Self::HEnd { x, .. } => x,
            &Self::VLine { x, .. } => x,
        }
    }
}

fn create_events(path: &[Move], h: &mut Vec<Event>, v: &mut Vec<Event>) {
    let mut cur = (0, 0);
    let mut dist = 0;

    for m in path {
        match m {
            &Move::Up(i) => {
                v.push(Event::VLine {
                    x: cur.0,
                    y1: cur.1,
                    y2: cur.1 + i,
                    dist
                });
                cur.1 += i;
                dist += i;
            },
            &Move::Down(i) => {
                v.push(Event::VLine {
                    x: cur.0,
                    y1: cur.1,
                    y2: cur.1 - i,
                    dist
                });
                cur.1 -= i;
                dist += i;
            },
            &Move::Left(i) => {
                cur.0 -= i;
                dist += i;
                h.push(Event::HStart {
                    x: cur.0,
                    y: cur.1,
                    dist,
                    dir: -1
                });
                h.push(Event::HEnd {
                    x: cur.0 + i,
                    x_orig: cur.0,
                    y: cur.1
                });
            },
            &Move::Right(i) => {
                h.push(Event::HStart {
                    x: cur.0,
                    y: cur.1,
                    dist,
                    dir: 1
                });
                h.push(Event::HEnd {
                    x: cur.0 + i,
                    x_orig: cur.0,
                    y: cur.1
                });
                cur.0 += i;
                dist += i;
            }
        }
    }
}

fn distance_to_origin(p: (i32, i32)) -> i32 {
    p.0.abs().saturating_add(p.1.abs())
}

fn min_dist_intersection(events: &[Event], line_dist: bool) -> i32 {
    // (y, x) pairs so they're ordered by the y axis
    let mut active = BTreeMap::new();
    let mut min_dist: i32 = i32::max_value();

    for event in events {
        match event {
            &Event::HStart{x, y, dist, dir} => {
                active.insert((y, x), (dist, dir));
            },
            &Event::HEnd {x: _, x_orig, y} => {
                active.remove(&(y, x_orig)).unwrap();
            },
            &Event::VLine{x, y1, y2, dist: vdist} => {
                let range = (
                    Bound::Included((y1.min(y2), x)),
                    Bound::Included((y2.max(y1), x))
                );

                for (&point, &(dist, dir)) in active.range(range) {
                    if line_dist {
                        let vdist = vdist + (point.0 - y1).abs();
                        let hdist = dist + dir * (x - point.1);

                        let tdist = vdist + hdist;
                        if tdist != 0 {
                            min_dist = min_dist.min(tdist);
                        }
                    } else {
                        let intersection = (x, point.0);
                        let new_dist = distance_to_origin(intersection);
                        if new_dist != 0 {
                            min_dist = min_dist.min(new_dist);
                        }
                    }
                }
            }
        }
    }

    min_dist
}

fn find_closest_intersection((path1, path2): &(Vec<Move>, Vec<Move>), line_dist: bool) -> i32 {
    let mut event_set_1 = Vec::new();
    let mut event_set_2 = Vec::new();

    create_events(&path1, &mut event_set_1, &mut event_set_2);
    create_events(&path2, &mut event_set_2, &mut event_set_1);

    event_set_1.sort_unstable_by_key(Event::get_x);
    event_set_2.sort_unstable_by_key(Event::get_x);

    let dist1 = min_dist_intersection(&event_set_1, line_dist);
    let dist2 = min_dist_intersection(&event_set_2, line_dist);

    dist1.min(dist2)
}

#[aoc(day3, part1)]
fn manhattan_intersection(paths: &(Vec<Move>, Vec<Move>)) -> i32 {
    find_closest_intersection(paths, false)
}

#[aoc(day3, part2)]
fn line_dist_intersection(paths: &(Vec<Move>, Vec<Move>)) -> i32 {
    find_closest_intersection(paths, true)
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
    fn test_line_dist() {
        let test_data = "R8,U5,L5,D3\r\nU7,R6,D4,L4";
        let paths = parse_paths(test_data);

        let min_dist = line_dist_intersection(&paths);

        assert_eq!(min_dist, 30);
    }

    #[test]
    fn test_simple_line_dist() {
        let events = vec![
            Event::HStart { x: 0, y: 0, dist: 9, dir: -1 },
            Event::VLine { x: 2, y1: 5, y2: -3, dist: 4 },
            Event::HEnd { x: 8, x_orig: 0, y: 0 },
        ];

        let dist = min_dist_intersection(&events, true);

        assert_eq!(dist, 16);
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
            Event::HStart{x: 0, y: 0, dist: 0, dir: 1},
            Event::HEnd{x: 8, x_orig: 0, y: 0},
            Event::HStart{x: 3, y: 5, dist: 18, dir: -1},
            Event::HEnd{x: 8, x_orig: 3, y: 5}
        ]);

        assert_eq!(v, vec![
            Event::VLine{x: 8, y1: 0, y2: 5, dist: 8},
            Event::VLine{x: 3, y1: 5, y2: 2, dist: 18}
        ]);

        h.sort_by_key(Event::get_x);
        v.sort_by_key(Event::get_x);

        assert_eq!(h, vec![
            Event::HStart{x: 0, y: 0, dist: 0, dir: 1},
            Event::HStart{x: 3, y: 5, dist: 18, dir: -1},
            Event::HEnd{x: 8, x_orig: 0, y: 0},
            Event::HEnd{x: 8, x_orig: 3, y: 5}
        ]);

        assert_eq!(v, vec![
            Event::VLine{x: 3, y1: 5, y2: 2, dist: 18},
            Event::VLine{x: 8, y1: 0, y2: 5, dist: 8}
        ]);
    }

    #[test]
    fn test_intersection() {
        let events = vec![
            Event::VLine{x: 0, y1: 0, y2: 7, dist: 0},
            Event::HStart{x: 0, y: 0, dist: 0, dir: 1},
            Event::HStart{x: 0, y: 7, dist: 0, dir: 1},
            Event::HStart{x: 2, y: 3, dist: 0, dir: 1},
            Event::HStart{x: 3, y: 5, dist: 0, dir: 1},
            Event::VLine{x: 3, y1: 5, y2: 2, dist: 0},
            Event::VLine{x: 6, y1: 7, y2: 3, dist: 0},
            Event::HEnd{x: 6, x_orig: 2, y: 3},
            Event::HEnd{x: 6, x_orig: 0, y: 7},
            Event::HEnd{x: 8, x_orig: 0, y: 0},
            Event::HEnd{x: 8, x_orig: 3, y: 5},
            Event::VLine{x: 8, y1: 0, y2: 5, dist: 0},
        ];

        assert_eq!(min_dist_intersection(&events, false), 6);
    }
}