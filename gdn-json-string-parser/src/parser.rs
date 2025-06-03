use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{anychar, char, multispace0, none_of, one_of},
    combinator::{map, map_opt, map_res, opt, recognize, verify},
    error::ParseError,
    multi::{many0, many1, separated_list0},
    sequence::{delimited, preceded, separated_pair, terminated},
};
use nom_locate::LocatedSpan;
use std::collections::HashMap;

type Span<'a> = LocatedSpan<&'a str, ()>;

#[derive(Debug, PartialEq, Clone)]
pub struct JsonString<'a> {
    pub value: String,
    pub pos: Span<'a>,
    pub col: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum JsonValue<'a> {
    Rest(String),
    Str(JsonString<'a>),
    Array(Vec<JsonValue<'a>>),
    Object(HashMap<String, JsonValue<'a>>),
}

fn ws<'a, O, E: ParseError<Span<'a>>, F: Parser<Span<'a>, Output = O, Error = E>>(
    f: F,
) -> impl Parser<Span<'a>, Output = O, Error = E> {
    delimited(multispace0, f, multispace0)
}

fn u16_hex(input: Span) -> IResult<Span, u16> {
    map_res(take(4usize), |s: Span| {
        u16::from_str_radix(s.fragment(), 16)
    })
    .parse(input)
}

fn unicode_escape(input: Span) -> IResult<Span, char> {
    map_opt(
        alt((
            // Not a surrogate
            map(verify(u16_hex, |cp| !(0xD800..0xE000).contains(cp)), |cp| {
                cp as u32
            }),
            map(
                verify(
                    separated_pair(u16_hex, tag("\\u"), u16_hex),
                    |(high, low)| (0xD800..0xDC00).contains(high) && (0xDC00..0xE000).contains(low),
                ),
                |(high, low)| {
                    let high_ten = (high as u32) - 0xD800;
                    let low_ten = (low as u32) - 0xDC00;
                    (high_ten << 10) + low_ten + 0x10000
                },
            ),
        )),
        // Could be probably replaced with .unwrap() or _unchecked due to the verify checks
        std::char::from_u32,
    )
    .parse(input)
}

fn character(input: Span) -> IResult<Span, char> {
    let (input, c) = none_of("\"")(input)?;
    if c == '\\' {
        alt((
            map_res(anychar, |c| {
                Ok(match c {
                    '"' | '\\' | '/' => c,
                    'b' => '\x08',
                    'f' => '\x0C',
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    _ => return Err(()),
                })
            }),
            preceded(char('u'), unicode_escape),
        ))
        .parse(input)
    } else {
        Ok((input, c))
    }
}
fn string_inside(i: Span) -> IResult<Span, Span> {
    recognize(many0(character)).parse(i)
}

fn string(input: Span) -> IResult<Span, Span> {
    delimited(char('"'), string_inside, char('"')).parse(input)
}

fn array(input: Span) -> IResult<Span, Vec<JsonValue>> {
    delimited(
        char('['),
        ws(separated_list0(ws(char(',')), json_value)),
        char(']'),
    )
    .parse(input)
}

fn object(input: Span) -> IResult<Span, HashMap<String, JsonValue>> {
    map(
        delimited(
            char('{'),
            ws(separated_list0(
                ws(char(',')),
                separated_pair(string, ws(char(':')), json_value),
            )),
            char('}'),
        ),
        |key_values| {
            key_values
                .into_iter()
                .map(|x| (x.0.fragment().to_string(), x.1))
                .collect()
        },
    )
    .parse(input)
}

fn float(input: Span) -> IResult<Span, Span> {
    alt((
        // Case one: .42
        recognize((
            char('.'),
            decimal,
            opt((one_of("eE"), opt(one_of("+-")), decimal)),
        )), // Case two: 42e42 and 42.42e42
        recognize((
            decimal,
            opt(preceded(char('.'), decimal)),
            one_of("eE"),
            opt(one_of("+-")),
            decimal,
        )), // Case three: 42. and 42.42
        recognize((decimal, char('.'), opt(decimal))),
    ))
    .parse(input)
}

fn decimal(input: Span) -> IResult<Span, Span> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))).parse(input)
}

fn parse_rest(i: Span) -> IResult<Span, String> {
    map(
        alt((tag("true"), tag("false"), tag("null"), float, decimal)),
        |s| s.to_string(),
    )
    .parse(i)
}

fn json_value(input: Span) -> IResult<Span, JsonValue> {
    alt((
        map(string, |e| {
            JsonValue::Str(JsonString {
                value: e.fragment().to_string(),
                pos: e,
                col: e.get_column(),
            })
        }),
        map(parse_rest, JsonValue::Rest),
        map(array, JsonValue::Array),
        map(object, JsonValue::Object),
    ))
    .parse(input)
}

pub fn json(i: &str) -> IResult<Span, JsonValue> {
    let span = Span::new(i);
    ws(json_value).parse(span)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_parse_one_line() {
        let input = r#"{"some": 23, "other": null, "something": "something", "double": 213.2, "other": [2343, true, false, "testing", {"other":null, "root": "tree"}], "sub": { "demo": "demo 2" } }"#;
        let (_remaining, _value) = json(input).unwrap();
    }

    #[test]
    fn test_json_parse() {
        let input = r#"{
        "some": 23, 
        "other": null,
         "something":
          "something", 
          "double": 213.2, 
          "other": [2343, true, false, "testing", {"other":null, "root": "tree"}], 
          "sub": { "demo": "demo 2" } 
        }"#;
        let (_remaining, _value) = json(input).unwrap();
    }

    #[test]
    fn test_bigger_json() {
        let source = include_str!("../tests/example1.json");
        let (_, _pr) = json(source).unwrap();
    }
}
