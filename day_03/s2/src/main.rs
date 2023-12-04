#![allow(dead_code)]
use std::{char, ops::Add};

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

struct Grid {
    length: usize,
    data: Vec<char>,
}

impl Grid {
    fn new(input: Vec<String>) -> Self {
        let length = input[0].len();
        let data: Vec<char> = input.join("").chars().collect();
        Self { length, data }
    }

    fn get(&self, coord: Coord) -> Option<Character> {
        if coord.x < 0 || coord.y < 0 {
            return None;
        }
        // The grid is a square x == y
        if coord.x >= self.length as isize || coord.y >= self.length as isize {
            return None;
        }
        Some(Character {
            value: self.data[coord.y as usize * self.length + coord.x as usize],
            coord,
        })
    }

    // Get all neighbours clockwise
    fn get_neighbours(&self, coord: Coord) -> Vec<Option<Character>> {
        let mut neighbours = Vec::new();
        let neighbours_coords: Vec<Coord> = Vec::from([
            (1, 0).into(),
            (1, 1).into(),
            (0, 1).into(),
            (-1, 1).into(),
            (-1, 0).into(),
            (-1, -1).into(),
            (0, -1).into(),
            (1, -1).into(),
        ]);

        for neighbour_coord in neighbours_coords {
            neighbours.push(self.get(coord + neighbour_coord));
        }

        neighbours
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
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

#[derive(Debug, Eq, PartialEq, Clone)]
struct Number {
    characters: Vec<Vec<Character>>,
}

impl Number {
    fn new() -> Self {
        Self {
            characters: Vec::new(),
        }
    }

    fn filter_part_number(&mut self, grid: &Grid) {
        let mut part_number = Vec::new();
        for number in self.characters.iter() {
            let mut number_part_number = Vec::new();
            for character in number.iter() {
                let neighbours = grid.get_neighbours(character.coord);
                let neighbours: Vec<Character> = neighbours.iter().filter_map(|o| *o).collect();
                let is_character_part_number = neighbours
                    .iter()
                    .any(|n| !(n.value.is_ascii_digit() || n.value == '.'));
                // dbg!(&neighbours, &is_character_part_number);
                number_part_number.push(is_character_part_number);
            }
            if number_part_number.iter().any(|o| *o) {
                part_number.push(number.clone());
            }
        }
        // dbg!(&part_number);
        self.characters = part_number;
    }

    fn get_gears(&self, grid: &Grid, gears: Vec<Character>) -> Vec<usize> {
        let gears_neighbours = gears
            .iter()
            .map(|c| {
                grid.get_neighbours(c.coord)
                    .iter()
                    .filter_map(|o| *o)
                    .collect::<Vec<Character>>()
            })
            .collect::<Vec<Vec<Character>>>();
        // dbg!(&gears_neighbours);

        dbg!(&self.characters);

        let gears_part_contacts = gears_neighbours
            .iter()
            .map(|g| {
                g.iter()
                    .flat_map(|gc| {
                        self.characters
                            .iter()
                            .enumerate()
                            .flat_map(move |(index, n)| {
                                n.iter()
                                    .filter_map(move |nc| {
                                        if nc.coord == gc.coord {
                                            Some(index)
                                        } else {
                                            None
                                        }
                                    })
                                    .collect::<Vec<usize>>()
                            })
                            .collect::<Vec<usize>>()
                    })
                    .collect()
            })
            .collect::<Vec<Vec<usize>>>();

        let gears_parts = gears_part_contacts
            .iter()
            .map(|g| {
                let mut contact_parts = g.clone();
                contact_parts.dedup();
                contact_parts = contact_parts
                    .iter()
                    .map(|p| self.get_number(*p) as usize)
                    .collect::<Vec<usize>>();
                contact_parts
            })
            .collect::<Vec<Vec<usize>>>()
            .into_iter()
            .filter(|g| g.len() > 1) // remove gears not in contact with 2 parts
            .collect::<Vec<Vec<usize>>>();

        let gears_ratio = gears_parts
            .iter()
            .map(|g| g.iter().product::<usize>())
            .collect::<Vec<usize>>();

        dbg!(gears_part_contacts);
        dbg!(gears_parts);
        dbg!(gears_ratio)
    }

    fn get_numbers(&self) -> Vec<u32> {
        self.characters
            .iter()
            .enumerate()
            .map(|(index, _v)| self.get_number(index))
            .collect::<Vec<u32>>()
    }

    fn get_number(&self, index: usize) -> u32 {
        self.characters[index]
            .iter()
            .map(|c| c.value.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<u32>()
            .unwrap()
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Character {
    value: char,
    coord: Coord,
}

fn run(input: Vec<String>) -> u32 {
    let grid = Grid::new(input);
    let mut numbers: Number = Number::new();
    let mut gears: Vec<Character> = Vec::new();
    for y in 0..grid.length {
        let mut characters: Vec<Character> = Vec::new();
        for x in 0..grid.length {
            let coord: Coord = (x as isize, y as isize).into();
            match grid.get(coord) {
                Some(character) if character.value == '*' => {
                    gears.push(character);
                    if !characters.is_empty() {
                        numbers.characters.push(characters.clone());
                    }
                    characters.clear();
                }

                Some(character) if character.value.is_ascii_digit() => {
                    characters.push(Character {
                        value: character.value,
                        coord,
                    });
                }

                _ => {
                    if !characters.is_empty() {
                        numbers.characters.push(characters.clone());
                    }
                    characters.clear();
                }
            }
        }
        // End of line, push remaining characters if we have some.
        if !characters.is_empty() {
            numbers.characters.push(characters.clone());
        }
    }
    // dbg!(&numbers);
    let ratio = numbers.get_gears(&grid, gears);
    ratio.iter().sum::<usize>() as u32
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
    fn test_get() {
        let input = parse_input(Some(indoc!(
            "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
            "
        )));
        dbg!(&input);
        let grid = Grid::new(input);

        assert_eq!(
            grid.get((0, 0).into()),
            Some(Character {
                value: '4',
                coord: (0, 0).into()
            })
        );
        assert_eq!(
            grid.get((2, 0).into()),
            Some(Character {
                value: '7',
                coord: (2, 0).into()
            })
        );
        assert_eq!(
            grid.get((3, 0).into()),
            Some(Character {
                value: '.',
                coord: (3, 0).into()
            })
        );
        assert_eq!(
            grid.get((9, 0).into()),
            Some(Character {
                value: '.',
                coord: (9, 0).into()
            })
        );
        assert_eq!(
            grid.get((1, 9).into()),
            Some(Character {
                value: '6',
                coord: (1, 9).into()
            })
        );
        assert_eq!(
            grid.get((5, 8).into()),
            Some(Character {
                value: '*',
                coord: (5, 8).into()
            })
        );
        assert_eq!(
            grid.get((9, 9).into()),
            Some(Character {
                value: '.',
                coord: (9, 9).into()
            })
        );
        assert_eq!(grid.get((10, 0).into()), None);
        assert_eq!(grid.get((10, 0).into()), None);
        assert_eq!(grid.get((-1, 0).into()), None);
        assert_eq!(grid.get((0, -1).into()), None);
    }

    #[test]
    fn test_add_coords() {
        let a: Coord = (1, 2).into();
        let b: Coord = (3, 4).into();
        assert_eq!(a + b, (4, 6).into());
    }

    #[test]
    fn test_get_neighbours() {
        let input = parse_input(Some(indoc!(
            "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
            "
        )));
        dbg!(&input);
        let grid = Grid::new(input);

        dbg!(grid.get_neighbours((0, 0).into()));
        assert_eq!(
            grid.get_neighbours((0, 0).into()),
            vec![
                Some(Character {
                    value: '6',
                    coord: (1, 0).into()
                }),
                Some(Character {
                    value: '.',
                    coord: (1, 1).into()
                }),
                Some(Character {
                    value: '.',
                    coord: (0, 1).into()
                }),
                None,
                None,
                None,
                None,
                None,
            ]
        );

        dbg!(grid.get_neighbours((2, 2).into()));
        assert_eq!(
            grid.get_neighbours((2, 2).into()),
            vec![
                Some(Character {
                    value: '5',
                    coord: (3, 2).into()
                }),
                Some(Character {
                    value: '.',
                    coord: (3, 3).into()
                }),
                Some(Character {
                    value: '.',
                    coord: (2, 3).into()
                }),
                Some(Character {
                    value: '.',
                    coord: (1, 3).into()
                }),
                Some(Character {
                    value: '.',
                    coord: (1, 2).into()
                }),
                Some(Character {
                    value: '.',
                    coord: (1, 1).into()
                }),
                Some(Character {
                    value: '.',
                    coord: (2, 1).into()
                }),
                Some(Character {
                    value: '*',
                    coord: (3, 1).into()
                }),
            ]
        );

        dbg!(grid.get_neighbours((6, 3).into()));
        assert_eq!(
            grid.get_neighbours((6, 3).into()),
            vec![
                Some(Character {
                    value: '.',
                    coord: (7, 3).into()
                }),
                Some(Character {
                    value: '.',
                    coord: (7, 4).into()
                }),
                Some(Character {
                    value: '.',
                    coord: (6, 4).into()
                }),
                Some(Character {
                    value: '.',
                    coord: (5, 4).into()
                }),
                Some(Character {
                    value: '.',
                    coord: (5, 3).into()
                }),
                Some(Character {
                    value: '.',
                    coord: (5, 2).into()
                }),
                Some(Character {
                    value: '6',
                    coord: (6, 2).into()
                }),
                Some(Character {
                    value: '3',
                    coord: (7, 2).into()
                }),
            ]
        );
    }

    #[test]
    fn test_run() {
        let input = parse_input(Some(indoc!(
            "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 467835);
    }

    #[test]
    fn test_run2() {
        let input = parse_input(Some(indoc!(
            "
            12.......*..
            +.........34
            .......-12..
            ..78........
            ..*....60...
            78.........9
            .5.....23..$
            8...90*12...
            ............
            2.2......12.
            .*.........*
            1.1..503+.56
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 31600);
    }
}
