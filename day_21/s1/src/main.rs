use std::{
    collections::{HashMap, HashSet, VecDeque},
    isize,
    ops::Add,
};

use nom::{
    bytes::complete::take_until, character::complete::line_ending, multi::many1,
    sequence::terminated, *,
};
use petgraph::{algo::dijkstra, graphmap::DiGraphMap, visit::EdgeRef};

const MAX_MOVES_IN_SAME_DIRECTION: usize = 3;

fn read_input(input: Option<&str>) -> String {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };

    input.to_string()
}

fn parse(input: &str) -> IResult<&str, Data> {
    let (input, lines) = many1(parse_line)(input)?;

    let data = lines
        .iter()
        .enumerate()
        .flat_map(|y| {
            y.1.iter()
                .enumerate()
                .map(|x| (Coord::from((x.0 as isize, y.0 as isize)), *x.1))
                .collect::<Vec<(Coord, char)>>()
        })
        .collect::<HashMap<Coord, char>>();

    let length_x = lines[0].len();
    let length_y = lines.len();

    let data = Data {
        length_x,
        length_y,
        grid: data,
    };

    Ok((input, data))
}

fn parse_line(input: &str) -> IResult<&str, Vec<char>> {
    let (input, pipe) = terminated(take_until("\n"), line_ending)(input)?;
    let pipe = pipe.to_string().chars().collect();
    Ok((input, pipe))
}

#[derive(Debug, PartialEq, Eq)]
struct Data {
    length_x: usize,
    length_y: usize,
    grid: HashMap<Coord, char>,
}

impl Data {
    // Get all neighbours clockwise
    fn get_neighbours(&self, coord: Coord) -> Vec<Option<(Coord, char)>> {
        let neighbours_coords: Vec<Coord> =
            Vec::from([(1, 0).into(), (0, 1).into(), (-1, 0).into(), (0, -1).into()]);

        let neighbours = neighbours_coords
            .iter()
            .map(|nc| self.grid.get(&(coord + *nc)).map(|v| (coord + *nc, *v)))
            .collect::<Vec<Option<(Coord, char)>>>();
        neighbours
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, Ord, PartialOrd)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
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

fn print_text_map(coordinates: &[(usize, usize, char)], width: usize, height: usize) {
    let mut grid = vec![vec!['.'; width]; height];

    // Place the points on the grid
    for &(x, y, v) in coordinates {
        if x < width && y < height {
            grid[y][x] = v; // Assuming the origin (0,0) is at the top-left corner
        }
    }

    // Print the grid row by row
    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!(); // Newline at the end of each row
    }
}

fn run(input: String) -> usize {
    let (_, mut data) = parse(&input).unwrap();
    dbg!(&data);

    print_text_map(
        &data
            .grid
            .iter()
            .map(|c| (c.0.x as usize, c.0.y as usize, *c.1))
            .collect::<Vec<(usize, usize, char)>>(),
        data.length_x,
        data.length_y,
    );

    let mut stack = Vec::new();
    let start_pos = data.grid.iter().find(|c| c.1 == &'S').unwrap().0;
    stack.push(*start_pos);
    let mut counter = 0;

    counter += 1;
    let new_pos = data
        .get_neighbours(pos)
        .into_iter()
        .filter_map(|x| x)
        .filter_map(|o| if o.1 == '#' { None } else { Some(o) })
        .collect::<Vec<(Coord, char)>>();

    while !stack.is_empty() {
        let pos = stack.pop().unwrap();
        for p in new_pos {
            stack.push(p.0);
        }

        if counter == 2 {
            for c in stack.iter() {
                *data.grid.get_mut(c).unwrap() = 'O';
            }
            break;
        }
    }

    print_text_map(
        &data
            .grid
            .iter()
            .map(|c| (c.0.x as usize, c.0.y as usize, *c.1))
            .collect::<Vec<(usize, usize, char)>>(),
        data.length_x,
        data.length_y,
    );
    todo!();
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
            ...........
            .....###.#.
            .###.##..#.
            ..#.#...#..
            ....#.#....
            .##..S####.
            .##..#...#.
            .......##..
            .##.#.####.
            .##..##.##.
            ...........
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 102);
    }
}
