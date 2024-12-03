use anyhow::Result;
use std::fs::read_to_string;
use nom::{
    bytes::complete::{tag, take_until}, character::complete::{anychar, i32, line_ending, space0}, combinator::opt, multi::{many0, many1}, sequence::{delimited, preceded, terminated, tuple}, IResult
};

fn parse_line(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    // let (input, _) = take_until(
    //     delimited(
    //         tag("mul("),
    //         tuple((i32, preceded(tag(","), i32))),
    //         tag(")")
    //     )
    // )(input);
    many1(
        preceded(
            many0(anychar),
            delimited(
                tag("mul("),
                tuple((i32, preceded(tag(","), i32))),
                tag(")")
            )
        )
    )(input)
}

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let tests = [
            ("mul(1,2)", vec![(1, 2)]),
            ("icaestnhmul(1,2)", vec![(1, 2)]),
        ];
        tests.into_iter().for_each(|(input, output)| {
            let (_, r) = parse_line(input).unwrap();
            assert_eq!(output, r);
        });
    }

    #[test]
    fn test_example() -> Result<()> {
        let input = read_to_string("example.txt")?;
        let (_, muls) = parse_line(&input).unwrap();
        let answer: i32 = muls.into_iter().map(|(num_one, num_two)| num_one * num_two).sum();
        assert_eq!(answer, 161);
        Ok(())
    }
}
