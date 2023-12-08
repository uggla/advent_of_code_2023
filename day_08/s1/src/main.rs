use std::collections::HashMap;

use nom::{bytes::complete::tag, character::complete::multispace1, multi::separated_list1, *};

fn read_input(input: Option<&str>) -> String {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };

    input.to_string()
}

fn parse(input: &str) -> IResult<&str, Data> {
    let (input, directions) = nom::character::complete::alphanumeric1(input)?;
    let (input, _) = multispace1(input)?;
    let (input, destinations) = separated_list1(tag("\n"), parse_line)(input)?;

    let destinations = destinations
        .iter()
        .map(|o| {
            let source = Location(o.0.to_string());
            let dest = o.1.chunks(2).fold(
                Destination::from(("".to_string(), "".to_string())),
                |_acc, x| {
                    let (left, right) = (x[0].to_string(), x[1].to_string());
                    Destination::from((left, right))
                },
            );
            (source, dest)
        })
        .collect::<Vec<_>>()
        .iter()
        .map(|o| (o.0.clone(), o.1.clone()))
        .collect();

    let data = Data {
        directions: directions.chars().map(Direction::new).collect(),
        destinations,
    };

    // let data= hands.into_iter().collect();

    Ok((input, data))
}

fn parse_line(input: &str) -> IResult<&str, (String, Vec<&str>)> {
    let (input, source) = nom::character::complete::alphanumeric1(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, destinations) =
        separated_list1(tag(", "), nom::character::complete::alphanumeric1)(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, (source.to_string(), destinations)))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn new(direction: char) -> Self {
        match direction {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Destination {
    left: Location,
    right: Location,
}

impl From<(String, String)> for Destination {
    fn from((left, right): (String, String)) -> Self {
        Self {
            left: Location(left),
            right: Location(right),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Location(String);

#[derive(Debug, PartialEq, Eq)]
struct Data {
    directions: Vec<Direction>,
    destinations: HashMap<Location, Destination>,
}

fn run(input: String) -> usize {
    let (_, data) = parse(&input).unwrap();
    dbg!(&data);

    let start = Location("AAA".to_string());
    let end = Location("ZZZ".to_string());

    let mut current = start;
    let mut iteration = 0;
    for d in data.directions.iter().cycle() {
        match d {
            Direction::Left => {
                current = data.destinations.get(&current).unwrap().left.clone();
            }
            Direction::Right => {
                current = data.destinations.get(&current).unwrap().right.clone();
            }
        }

        iteration += 1;
        if current == end {
            break;
        }
    }

    dbg!(iteration)
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
    fn test_run() {
        let input = read_input(Some(indoc!(
            "
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 6);
    }
}
