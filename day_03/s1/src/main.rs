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

    fn get(&self, coord: Coord) -> Option<char> {
        if coord.x < 0 || coord.y < 0 {
            return None;
        }
        // The grid is a square x == y
        if coord.x >= self.length as isize || coord.y >= self.length as isize {
            return None;
        }
        Some(self.data[coord.y as usize * self.length + coord.x as usize])
    }

    // Get all neighbours clockwise
    fn get_neighbours(&self, coord: Coord) -> Vec<Option<char>> {
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
                let neighbours: Vec<char> = neighbours.iter().filter_map(|o| *o).collect();
                let is_character_part_number =
                    neighbours.iter().any(|n| !(n.is_digit(10) || *n == '.'));
                // dbg!(&neighbours, &is_character_part_number);
                number_part_number.push(is_character_part_number);
            }
            if number_part_number.iter().any(|o| *o == true) {
                part_number.push(number.clone());
            }
        }
        // dbg!(&part_number);
        self.characters = part_number;
    }

    fn get_part_numbers(&self) -> Vec<u32> {
        self.characters
            .iter()
            .map(|v| {
                v.iter()
                    .map(|c| c.value.to_string())
                    .collect::<Vec<String>>()
                    .join("")
                    .parse::<u32>()
                    .unwrap()
            })
            .collect::<Vec<u32>>()
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
    for y in 0..grid.length {
        let mut characters: Vec<Character> = Vec::new();
        for x in 0..grid.length {
            let coord: Coord = (x as isize, y as isize).into();
            match grid.get(coord) {
                Some(value) if value.is_digit(10) => {
                    characters.push(Character { value, coord });
                }

                _ => {
                    if !characters.is_empty() {
                        numbers.characters.push(characters.clone());
                    }
                    characters.clear();
                }
            }
        }
        if !characters.is_empty() {
            numbers.characters.push(characters.clone());
        }
    }
    dbg!(&numbers);
    numbers.filter_part_number(&grid);
    let part_number = numbers.get_part_numbers();
    dbg!(&part_number);
    part_number.iter().sum()
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

        assert_eq!(grid.get((0, 0).into()), Some('4'));
        assert_eq!(grid.get((2, 0).into()), Some('7'));
        assert_eq!(grid.get((3, 0).into()), Some('.'));
        assert_eq!(grid.get((9, 0).into()), Some('.'));
        assert_eq!(grid.get((1, 9).into()), Some('6'));
        assert_eq!(grid.get((5, 8).into()), Some('*'));
        assert_eq!(grid.get((9, 9).into()), Some('.'));
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
                Some('6'),
                Some('.'),
                Some('.'),
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
                Some('5'),
                Some('.'),
                Some('.'),
                Some('.'),
                Some('.'),
                Some('.'),
                Some('.'),
                Some('*'),
            ]
        );

        dbg!(grid.get_neighbours((6, 3).into()));
        assert_eq!(
            grid.get_neighbours((6, 3).into()),
            vec![
                Some('.'),
                Some('.'),
                Some('.'),
                Some('.'),
                Some('.'),
                Some('.'),
                Some('6'),
                Some('3'),
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
        assert_eq!(answer, 4361);
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
        assert_eq!(answer, 925);
    }
}
