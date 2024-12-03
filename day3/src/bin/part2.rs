use anyhow::Result;
use std::fs::read_to_string;
use nom::{
    bytes::complete::{tag, take_until}, character::complete::{anychar, i32}, combinator::{map, map_parser, map_res, opt}, multi::{many0, many1, many_till}, sequence::{delimited, preceded, terminated, tuple}, IResult
};

fn parse_line(input: &str) -> IResult<&str, (Vec<(Vec<char>, (i32, i32))>, Vec<Vec<(Vec<char>, (i32, i32))>>)> {
    tuple((
        map_parser(
            take_until("don't()"),
            |text| 
            many1(
                many_till(
                    anychar,
                    delimited(
                        tag("mul("),
                        tuple((i32, preceded(tag(","), i32))),
                        tag(")")
                    )
                )
            )(text),
        ),
        // many0(
        //         preceded(
        //             many_till(
        //                 anychar,
        //                 tag("do()"),
        //             ),
        //             terminated(
        //                 many1(
        //                     many_till(
        //                         anychar,
        //                         delimited(
        //                             tag("mul("),
        //                             tuple((i32, preceded(tag(","), i32))),
        //                             tag(")")
        //                         )
        //                     )
        //                 ),
        //                 opt(many_till(
        //                     anychar,
        //                     tag("don't()"),
        //                 )),
        //             )
        //         )
        // )
        many1(
        map_parser(
            preceded(
                many_till(
                    anychar,
                    tag("do()"),
                ),
                take_until("don't()")
            ),
            |text| many1(many_till(
                    anychar,
                    delimited(
                        tag("mul("),
                        tuple((i32, preceded(tag(","), i32))),
                        tag(")")
                    )
            ))(text),
        ))
    ))(input)
}

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;
    let (_, (first, rest)) = parse_line(&input).unwrap();
    let mut nums = vec![];
    for x in first {
        nums.push(x.1)
    }
    // nums.push(first.1);
    for x in rest {
        for y in x {
            nums.push(y.1);
        }
    }
    println!("Nums: {:?}", nums);
    let answer: i32 = nums.into_iter().map(|(num_one, num_two)| num_one * num_two).sum();
    println!("{answer}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let tests = [
            // ("mul(1,2)", vec![(1, 2)]),
            // ("icaestnhmul(1,2)", vec![(1, 2)]),
            ("icaestnhmul(1,2)hnnadon't()mul(1,2)nhmul(3,4)iado()snthmul(1,2)nthmul(1,2)snthsdon't()ththmul(3,4)", vec![(1, 2), (1, 2), (1, 2)]),
        ];
        tests.into_iter().for_each(|(input, output)| {
            let (_, (first, rest)) = parse_line(&input).unwrap();
            let mut nums = vec![];
            for x in first {
                nums.push(x.1)
            }
            for x in rest {
                for y in x {
                    nums.push(y.1);
                }
            }
            assert_eq!(output, nums);
        });
    }

    #[test]
    fn test_example() -> Result<()> {
        let input = read_to_string("example2.txt")?;
        let (_, (first, rest)) = parse_line(&input).unwrap();
        let mut nums = vec![];
        for x in first {
            nums.push(x.1)
        }
        for x in rest {
            for y in x {
                nums.push(y.1);
            }
        }
        println!("Nums: {:?}", nums);
        let answer: i32 = nums.into_iter().map(|(num_one, num_two)| num_one * num_two).sum();
        assert_eq!(answer, 48);
        Ok(())
    }
}
