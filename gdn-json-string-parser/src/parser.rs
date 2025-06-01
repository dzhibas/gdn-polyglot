use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_until},
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace0},
    combinator::{cut, map, opt, recognize},
    error::ParseError,
    multi::{many0, many0_count, separated_list0},
    sequence::{delimited, pair, tuple},
    IResult,
};

/// Took from nom recipes
pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn parse_string(i: &str) -> IResult<&str, &str> {
    let parser_a = delimited(tag("\""), take_until("\""), tag("\""));
    let parser_b = delimited(tag("\'"), take_until("\'"), tag("\'"));
    alt((parser_a, parser_b))(i)
}
