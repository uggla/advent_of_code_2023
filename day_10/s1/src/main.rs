use core::panic;
use std::{collections::BTreeMap, ops::Add};

use nom::{
    bytes::complete::take_until, character::complete::line_ending, multi::many1,
    sequence::terminated, *,
};

fn read_input(input: Option<&str>) -> String {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };

    input.to_string()
}

fn parse(input: &str) -> IResult<&str, Data> {
    let (input, pipes) = many1(parse_line)(input)?;

    let data = pipes
        .iter()
        .enumerate()
        .flat_map(|y| {
            y.1.iter()
                .enumerate()
                .map(|x| (Coord::from((x.0 as isize, y.0 as isize)), *x.1))
                .collect::<Vec<(Coord, char)>>()
        })
        .collect::<BTreeMap<Coord, char>>();

    let data = Data { grid: data };

    Ok((input, data))
}

fn parse_line(input: &str) -> IResult<&str, Vec<char>> {
    let (input, pipe) = terminated(take_until("\n"), line_ending)(input)?;
    let pipe = pipe.to_string().chars().collect();
    Ok((input, pipe))
}

#[derive(Debug, PartialEq, Eq)]
struct Data {
    grid: BTreeMap<Coord, char>,
}

impl Data {
    // Get all neighbours clockwise
    #[allow(dead_code)]
    fn get_neighbours(&self, coord: Coord) -> Vec<(Coord, char)> {
        let neighbours_coords: Vec<Coord> =
            Vec::from([(1, 0).into(), (0, 1).into(), (-1, 0).into(), (0, -1).into()]);

        let neighbours = neighbours_coords
            .iter()
            .filter_map(|nc| self.grid.get(&(coord + *nc)).map(|v| (coord + *nc, *v)))
            .collect::<Vec<(Coord, char)>>();
        neighbours
    }

    fn next_pos(&self, current_pos: Coord, prev_pos: Coord) -> (Coord, char) {
        println!("{:?}-{:?}", prev_pos, current_pos);
        let neighbours = self.get_neighbours_with_pipes(current_pos);
        dbg!(neighbours
            .into_iter()
            .filter(|n| n.0 != prev_pos)
            .nth(0)
            .unwrap())
    }

    // Get all possible neighbours clockwise depending on pipes
    fn get_neighbours_with_pipes(&self, coord: Coord) -> Vec<(Coord, char)> {
        let neighbours_coords: Vec<Coord> =
            Vec::from([(1, 0).into(), (0, 1).into(), (-1, 0).into(), (0, -1).into()]);

        let neighbours = neighbours_coords
            .iter()
            .filter_map(|nc| {
                //
                let current_coord_value = self.grid.get(&coord).unwrap();
                match self.grid.get(&(coord + *nc)) {
                    None => None,
                    Some(v) => {
                        let new_coord = coord + *nc;

                        match current_coord_value {
                            '-' => {
                                match nc {
                                    Coord { x: 1, y: 0 } => {
                                        //
                                        match v {
                                            '-' => Some((new_coord, *v)),
                                            'J' => Some((new_coord, *v)),
                                            '7' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    Coord { x: -1, y: 0 } => {
                                        //
                                        match v {
                                            '-' => Some((new_coord, *v)),
                                            'L' => Some((new_coord, *v)),
                                            'F' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    _ => None,
                                }
                            }
                            '7' => {
                                match nc {
                                    Coord { x: 0, y: 1 } => {
                                        //
                                        match v {
                                            '|' => Some((new_coord, *v)),
                                            'L' => Some((new_coord, *v)),
                                            'J' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    Coord { x: -1, y: 0 } => {
                                        //
                                        match v {
                                            '-' => Some((new_coord, *v)),
                                            'L' => Some((new_coord, *v)),
                                            'F' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    _ => None,
                                }
                            }
                            //
                            '|' => {
                                match nc {
                                    Coord { x: 0, y: 1 } => {
                                        //
                                        match v {
                                            '|' => Some((new_coord, *v)),
                                            'L' => Some((new_coord, *v)),
                                            'J' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    Coord { x: 0, y: -1 } => {
                                        //
                                        match v {
                                            '|' => Some((new_coord, *v)),
                                            'F' => Some((new_coord, *v)),
                                            '7' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    _ => None,
                                }
                            }
                            'J' => {
                                match nc {
                                    Coord { x: -1, y: 0 } => {
                                        //
                                        match v {
                                            '-' => Some((new_coord, *v)),
                                            'L' => Some((new_coord, *v)),
                                            'F' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    Coord { x: 0, y: -1 } => {
                                        //
                                        match v {
                                            '|' => Some((new_coord, *v)),
                                            'F' => Some((new_coord, *v)),
                                            '7' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    _ => None,
                                }
                            }
                            'L' => {
                                match nc {
                                    Coord { x: 1, y: 0 } => {
                                        //
                                        match v {
                                            '-' => Some((new_coord, *v)),
                                            'J' => Some((new_coord, *v)),
                                            '7' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    Coord { x: 0, y: -1 } => {
                                        //
                                        match v {
                                            '|' => Some((new_coord, *v)),
                                            'F' => Some((new_coord, *v)),
                                            '7' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    _ => None,
                                }
                            }
                            'F' => {
                                match nc {
                                    Coord { x: 1, y: 0 } => {
                                        //
                                        match v {
                                            '-' => Some((new_coord, *v)),
                                            'J' => Some((new_coord, *v)),
                                            '7' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    Coord { x: 0, y: 1 } => {
                                        //
                                        match v {
                                            '|' => Some((new_coord, *v)),
                                            'L' => Some((new_coord, *v)),
                                            'J' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    _ => None,
                                }
                            }
                            'S' => {
                                match nc {
                                    Coord { x: 1, y: 0 } => {
                                        //
                                        match v {
                                            '-' => Some((new_coord, *v)),
                                            'J' => Some((new_coord, *v)),
                                            '7' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    Coord { x: 0, y: 1 } => {
                                        //
                                        match v {
                                            '|' => Some((new_coord, *v)),
                                            'L' => Some((new_coord, *v)),
                                            'J' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    Coord { x: -1, y: 0 } => {
                                        //
                                        match v {
                                            '-' => Some((new_coord, *v)),
                                            'L' => Some((new_coord, *v)),
                                            'F' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    Coord { x: 0, y: -1 } => {
                                        //
                                        match v {
                                            '|' => Some((new_coord, *v)),
                                            'F' => Some((new_coord, *v)),
                                            '7' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    _ => panic!("Invalid coord"),
                                }
                            }
                            _ => panic!("Invalid value"),
                        }
                    }
                }
            })
            .collect::<Vec<(Coord, char)>>();
        neighbours
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, PartialOrd, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

impl From<(isize, isize)> for Coord {
    fn from((x, y): (isize, isize)) -> Self {
        Self { x, y }
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;
    fn add(self, rhs: Coord) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn run(input: String) -> isize {
    let (_, data) = parse(&input).unwrap();
    dbg!(&data);

    let start = data.grid.iter().find(|x| x.1 == &'S').unwrap();

    let mut current_pos = (*start.0, *start.1);
    let mut previous_pos = current_pos;
    let mut iterations = 0;
    loop {
        let new_pos = data.next_pos(current_pos.0, previous_pos.0);
        previous_pos = current_pos;
        current_pos = new_pos;

        iterations += 1;

        if current_pos.1 == 'S' {
            break;
        }
    }
    dbg!(&iterations / 2)
}

fn main() {
    let input = read_input(None);

    let answer = run(input);

    println!("Answer: {}", answer);
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use indoc::indoc;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_fake() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_run1() {
        let input = read_input(Some(indoc!(
            "
            -L|F7
            7S-7|
            L|7||
            -L-J|
            L|-JF
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 4);
    }

    #[test]
    fn test_run2() {
        let input = read_input(Some(indoc!(
            "
            7-F7-
            .FJ|7
            SJLL7
            |F--J
            LJ.LJ
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 8);
    }
}
