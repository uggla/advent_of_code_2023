#![allow(dead_code)]
use std::collections::HashMap;

fn parse_input(input: Option<&str>) -> Vec<String> {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };
    let output = input
        .strip_suffix('\n')
        .unwrap()
        .split('\n')
        .map(|o| o.to_string())
        .collect::<Vec<String>>();

    output
}

#[derive(Debug)]
struct Cubes {
    red: Vec<u32>,
    green: Vec<u32>,
    blue: Vec<u32>,
}

#[derive(Debug)]
struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}
#[derive(PartialEq, Debug, Eq)]
enum Solution {
    Possible,
    NotPossible,
}

fn is_posible(game_cubes: &Cubes, bag: &Bag) -> Solution {
    if game_cubes.red.iter().all(|x| x <= &bag.red)
        && game_cubes.green.iter().all(|x| x <= &bag.green)
        && game_cubes.blue.iter().all(|x| x <= &bag.blue)
    {
        Solution::Possible
    } else {
        Solution::NotPossible
    }
}

fn get_power(game_cubes: &Cubes) -> u32 {
    game_cubes.red.iter().max().unwrap()
        * game_cubes.green.iter().max().unwrap()
        * game_cubes.blue.iter().max().unwrap()
}

fn parse(input: Vec<String>) -> HashMap<u32, Cubes> {
    let mut data: HashMap<u32, Cubes> = HashMap::new();
    for line in &input {
        let mut cube = Cubes {
            red: Vec::new(),
            green: Vec::new(),
            blue: Vec::new(),
        };
        let line_split: Vec<&str> = line.split(':').collect();
        let id = line_split[0]
            .replace("Game ", "")
            .trim()
            .parse::<u32>()
            .unwrap();
        let color_data = line_split[1].replace(';', ","); // seriously
        let color_data: Vec<&str> = color_data.split(',').collect();
        for item in color_data {
            let color_data_split: Vec<&str> = item.trim().split(' ').collect();
            let value = color_data_split[0].trim().parse::<u32>().unwrap();
            let color = color_data_split[1].trim();
            match color {
                "red" => cube.red.push(value),
                "green" => cube.green.push(value),
                "blue" => cube.blue.push(value),
                _ => panic!("unknown color"),
            }
        }
        data.insert(id, cube);
    }
    dbg!(data)
}

fn run(input: Vec<String>) -> u32 {
    let data = parse(input);

    data.iter()
        .map(|(key, value)| {
            dbg!(key, value);
            get_power(value)
        })
        .sum()
}

fn main() {
    let input = parse_input(None);

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
    fn test_is_posible() {
        let game_cubes1 = Cubes {
            red: Vec::from([10]),
            green: Vec::from([10]),
            blue: Vec::from([10]),
        };
        let game_cubes2 = Cubes {
            red: Vec::from([10, 10, 11]),
            green: Vec::from([10]),
            blue: Vec::from([10]),
        };
        let game_cubes3 = Cubes {
            red: Vec::from([10]),
            green: Vec::from([10]),
            blue: Vec::from([15]),
        };

        let bag = Bag {
            red: 10,
            green: 10,
            blue: 10,
        };
        assert_eq!(is_posible(&game_cubes1, &bag), Solution::Possible);
        assert_eq!(is_posible(&game_cubes2, &bag), Solution::NotPossible);
        assert_eq!(is_posible(&game_cubes3, &bag), Solution::NotPossible);
    }

    #[test]
    fn test_get_power() {
        let game_cubes1 = Cubes {
            red: Vec::from([10]),
            green: Vec::from([10]),
            blue: Vec::from([10]),
        };
        let game_cubes2 = Cubes {
            red: Vec::from([10, 10, 11]),
            green: Vec::from([10]),
            blue: Vec::from([10]),
        };
        let game_cubes3 = Cubes {
            red: Vec::from([10]),
            green: Vec::from([10]),
            blue: Vec::from([15]),
        };
        assert_eq!(get_power(&game_cubes1), 1000);
        assert_eq!(get_power(&game_cubes2), 1100);
        assert_eq!(get_power(&game_cubes3), 1500);
    }

    #[test]
    fn test_run() {
        let input = parse_input(Some(indoc!(
            "
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 2286);
    }
}
