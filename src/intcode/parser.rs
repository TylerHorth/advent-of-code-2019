use nom::{Err, IResult};
use nom::character::complete::{digit1, line_ending, char};
use nom::combinator::{all_consuming, map_res, opt, recognize};
use nom::error::ErrorKind;
use nom::multi::separated_list;
use nom::sequence::tuple;

fn int(input: &str) -> IResult<&str, i64> {
    map_res(recognize(tuple((opt(char('-')), digit1))), str::parse)(input)
}

fn ints(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list(char(','), int)(input)
}

pub type ParseError<'a> = Err<(&'a str, ErrorKind)>;

pub fn read_program(program: &str) -> Result<Vec<i64>, ParseError> {
    all_consuming(tuple((ints, opt(line_ending))))(program)
        .map(|(_, (result, _))| result)
}
