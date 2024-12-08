use anyhow::{Result, anyhow};
use std::fs::read_to_string;
use nom::{
    character::complete::{anychar, char, i32, line_ending, one_of, space0}, combinator::opt, multi::many1, sequence::{preceded, terminated}, IResult
};

fn get_all_lines(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    many1(terminated(many1(one_of("#.^<>v")), line_ending))(input)
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct GameState {
    map: Vec<Vec<char>>,
    position: (usize, usize),
    direction: Direction,
}

impl GameState {
    fn new(input: &str) -> Self {
        let input = read_to_string(input).unwrap();
        let (_, lines) = get_all_lines(&input).unwrap();
        let mut state = GameState {
            map: lines,
            position: (0, 0),
            direction: Direction::Up,
        };

        state.find_position();

        state
    }

    fn change_direction(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
            Direction::Right => self.direction = Direction::Down,
        }
    }

    fn change_square(&mut self, c: char, x: usize, y: usize) {
        if let Some(row) = self.map.get_mut(x) {
            row[y] = c;
        }
    }

    fn find_position(&mut self) {
        self.map.iter().enumerate().for_each(|(i, row)|
            row.into_iter().enumerate().for_each(|(j, c)| 
                match c {
                    &'^' => {
                        self.position = (i, j);
                        self.direction = Direction::Up;
                    }
                    &'<' => {
                        self.position = (i, j);
                        self.direction = Direction::Left;
                    }
                    &'>' => {
                        self.position = (i, j);
                        self.direction = Direction::Right;
                    }
                    &'v' => {
                        self.position = (i, j);
                        self.direction = Direction::Down;
                    }
                    _ => {},
                }
            )
        )
    }

    fn move_guard(&mut self) -> Result<()> {
        let (x, y) = self.position;
        match self.get_guard_next_position() {
            Some(c) => {
                match c {
                    &'#' => self.change_direction(),
                    _ => {
                        match self.direction {
                            Direction::Up => {
                                self.change_square('^', x - 1, y);
                                self.change_square('X', x, y);
                                self.position = (x - 1, y);
                            },
                            Direction::Down => {
                                self.change_square('v', x + 1, y);
                                self.change_square('X', x, y);
                                self.position = (x + 1, y);
                            },
                            Direction::Left => {
                                self.change_square('<', x, y - 1);
                                self.change_square('X', x, y);
                                self.position = (x, y - 1);
                            },
                            Direction::Right => {
                                self.change_square('>', x, y + 1);
                                self.change_square('X', x, y);
                                self.position = (x, y + 1);
                            },
                        }
                    },
                }

                return Ok(());
            },
            None => {
                self.change_square('X', x, y);
                return Err(anyhow!("guard will move off the board"))
            },
        }
    }

    fn get_guard_next_position(&self) -> Option<&char> {
        match self.direction {
            Direction::Up => {
                if self.position.0 == 0 {
                    return None;
                } else {
                    match self.map.get(self.position.0 - 1) {
                        Some(row) => return row.get(self.position.1),
                        None => return None,
                    }
                }
            },
            Direction::Down => {
                match self.map.get(self.position.0 + 1) {
                    Some(row) => return row.get(self.position.1),
                    None => return None,
                }
            },
            Direction::Left => {
                if self.position.1 == 0 {
                    return None;
                } else {
                    match self.map.get(self.position.0) {
                        Some(row) => return row.get(self.position.1 - 1),
                        None => return None,
                    }
                }
            },
            Direction::Right => {
                match self.map.get(self.position.0) {
                    Some(row) => return row.get(self.position.1 + 1),
                    None => return None,
                }
            },
        }
    }

    fn count_xs(&self) -> usize {
        self.map.iter().map(|row|
            row.into_iter().filter(|c| *c == &'X')
                .count()
        ).sum()
    }
}

fn main() -> Result<()> {
    let mut state = GameState::new("input.txt");
    while let Ok(()) = state.move_guard() {}
    // for row in state.map.iter() {
    //     println!("{:?}", row);
    // }
    let count = state.count_xs();

    println!("{count}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let mut state = GameState::new("example.txt");
        while let Ok(()) = state.move_guard() {}
        for row in state.map.iter() {
            println!("{:?}", row);
        }
        let count = state.count_xs();

        assert_eq!(count, 41);
        Ok(())
    }
}
