use std::fs::read_to_string;
use anyhow::Result;
use nom::{
    bytes::complete::tag, character::complete::{i128, line_ending}, multi::many1, sequence::{preceded, terminated, tuple}, IResult
};
use itertools::Itertools;
use rayon::prelude::*;

fn process_line(input: &str) -> IResult<&str, (i128, Vec<i128>)> {
    tuple((
        terminated(i128, tag(":")), 
        many1(
            preceded(
                tag(" "), 
                i128
            )
        )
    ))(input)
}

fn get_all_lines(input: &str) -> IResult<&str, Vec<(i128, Vec<i128>)>> {
    many1(terminated(process_line, line_ending))(input)
}

fn is_valid_combination(result: &i128, nums: &Vec<i128>, combination: Vec<&str>) -> bool {
    assert_eq!(nums.len() - 1, combination.len());
    let mut total = 0;
    combination.
        iter()
        .enumerate()
        .for_each(|(i, op)| {
            if i == 0 {
                let first = nums.get(i).unwrap();
                let second = nums.get(i + 1).unwrap();
                match op {
                    &"+" => {
                        let add = first + second;
                        total += add;
                    },
                    &"*" => {
                        let add = first * second;
                        total += add;
                    },
                    &"||" => {
                        let first_str = first.to_string();
                        let second_str = second.to_string();
                        let combined_str = first_str + &second_str;
                        total = combined_str.parse::<i128>().unwrap();
                    },
                    _ => {},
                }
            } else {
                match op {
                    &"+" => {
                        total += nums.get(i + 1).unwrap();
                    },
                    &"*" => {
                        total *= nums.get(i + 1).unwrap();
                    },
                    &"||" => {
                        let first_str = total.to_string();
                        let second_str = nums.get(i + 1).unwrap().to_string();
                        let combined_str = first_str + &second_str;
                        total = combined_str.parse::<i128>().unwrap();
                    },
                    _ => {},
                }
            }
        });

    // if &total == result {
    //     println!("{:?}\nResult: {}\nTotal: {}\nNums: {:?}\nCombination: {:?}\n\n", &total == result,result, total, nums, combination);
    // }
    &total == result
}

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;
    let (_, lines) = get_all_lines(&input).unwrap();
    let sum: i128 = lines
        .par_iter()
        .filter(|(result, nums)| {
            let mut operators = Vec::new();
            for _ in 0..nums.len() - 1 {
                operators.push("+");
                operators.push("*");
                operators.push("||");
            }
            let combinations = operators.into_iter().combinations(nums.len() - 1).collect::<Vec<_>>();
            let count = combinations.into_iter().filter(|comb| {
                is_valid_combination(&result, &nums, comb.clone())
            }).count();

            count > 0
        })
        .map(|(result, _)| result)
        .sum();
    println!("{sum}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_example_part_two() -> Result<()> {
        let input = read_to_string("example.txt")?;
        let (_, lines) = get_all_lines(&input).unwrap();
        let sum: i128 = lines
            .into_iter()
            .filter(|(result, nums)| {
                let mut operators = Vec::new();
                for _ in 0..nums.len() - 1 {
                    operators.push("+");
                    operators.push("*");
                    operators.push("||");
                }
                let combinations = operators.into_iter().combinations(nums.len() - 1).collect::<Vec<_>>();
                let count = combinations.into_iter().filter(|comb| {
                    is_valid_combination(&result, &nums, comb.clone())
                }).count();

                count > 0
            })
            .map(|(result, _)| result)
            .sum();
        assert_eq!(sum, 11387);
        Ok(())
    }
}

