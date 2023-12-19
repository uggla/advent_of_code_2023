use core::panic;
use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Add,
};

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
        .collect::<BTreeMap<Coord, char>>();

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
    grid: BTreeMap<Coord, char>,
}

impl Data {
    // Get all neighbours clockwise
    #[allow(dead_code)]
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

fn run(input: String) -> usize {
    let (_, mut data) = parse(&input).unwrap();
    dbg!(&data);

    let rrocks = find_motif(&data, 'O');
    let csrocks = find_motif(&data, '#');

    dbg!(&rrocks);
    dbg!(&csrocks);

    for c in rrocks.iter() {
        for y in (0..=c.y).rev() {
            let rcoord = Coord::from((c.x, y));
            let neighbours = data.get_neighbours(rcoord);
            match neighbours.get(3).unwrap() {
                Some((nc, v)) => match v {
                    '.' => {
                        // *c = *nc;
                        *data.grid.get_mut(&rcoord).unwrap() = '.';
                        *data.grid.get_mut(nc).unwrap() = 'O';
                    }
                    '#' => {
                        break;
                    }
                    'O' => {
                        break;
                    }
                    _ => panic!("Non expected char"),
                },
                None => {}
            }
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

    let mut nb_rocks = Vec::new();
    for y in 0..data.length_y {
        let mut rocks_on_that_line = Vec::new();
        for x in 0..data.length_x {
            if data.grid.get(&(Coord::from((x as isize, y as isize)))) == Some(&'O') {
                rocks_on_that_line.push(Coord::from((x as isize, y as isize)));
            }
        }
        nb_rocks.push(rocks_on_that_line);
    }

    dbg!(&nb_rocks);

    let mut load = Vec::new();
    for (i, rocks) in nb_rocks.iter().enumerate() {
        let index = data.length_y - i;
        load.push(index * rocks.len());
    }

    dbg!(load.iter().sum::<usize>())
}

fn find_motif(data: &Data, motif: char) -> BTreeSet<Coord> {
    let mut mcoords = (0..data.length_x)
        .flat_map(|x| {
            (0..data.length_y)
                .filter_map(|y| {
                    if data.grid.get(&(Coord::from((x as isize, y as isize)))) == Some(&motif) {
                        Some(Coord::from((x as isize, y as isize)))
                    } else {
                        None
                    }
                })
                .collect::<Vec<Coord>>()
        })
        .collect::<Vec<Coord>>();
    mcoords.sort_by_key(|v| (v.x, v.y));

    mcoords.into_iter().collect::<BTreeSet<Coord>>()
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
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 136);
    }
}
