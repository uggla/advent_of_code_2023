use core::panic;
use itertools::Itertools;
use std::{collections::HashMap, ops::Add};

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

#[derive(Debug, Clone, Copy)]
enum TiltDirection {
    East = 0,
    South = 1,
    West = 2,
    North = 3,
}

fn run(input: String) -> usize {
    const CYCLE_NB: usize = 10usize.pow(9);
    let (_, mut data) = parse(&input).unwrap();

    let mut grid_sav: HashMap<Vec<char>, usize> = HashMap::new();
    let mut iteration = CYCLE_NB;

    for i in 0..CYCLE_NB {
        tilt_cycle(&mut data);
        let key = data.grid.values().cloned().collect::<Vec<char>>();
        if iteration == i {
            break;
        }
        if grid_sav.contains_key(&key) && iteration == CYCLE_NB {
            let cycle_length = i - grid_sav.get(&key).unwrap();
            // A repeating cycle with a duration of cycle_length is
            // identified. The objective is to calculate the state of the
            // iteration that corresponds to the same state expected after
            // 10^9 iterations.
            iteration =
                (i + cycle_length + CYCLE_NB % cycle_length - grid_sav.get(&key).unwrap()) - 1;
        } else {
            grid_sav.insert(key, i);
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

    let mut load = Vec::new();
    for (i, rocks) in nb_rocks.iter().enumerate() {
        let index = data.length_y - i;
        load.push(index * rocks.len());
    }
    dbg!(load.iter().sum::<usize>())
}

fn tilt_cycle(data: &mut Data) {
    tilt(&find_motif(&*data, 'O'), data, &TiltDirection::North);
    tilt(&find_motif(&*data, 'O'), data, &TiltDirection::West);
    tilt(&find_motif(&*data, 'O'), data, &TiltDirection::South);
    tilt(&find_motif(&*data, 'O'), data, &TiltDirection::East);
}

fn tilt(rrocks: &[Coord], data: &mut Data, direction: &TiltDirection) {
    let sort_type = match direction {
        TiltDirection::East => east_sort(rrocks),
        TiltDirection::South => south_sort(rrocks),
        TiltDirection::West => west_sort(rrocks),
        TiltDirection::North => north_sort(rrocks),
    };
    for rcoord in sort_type.iter() {
        for t_rcoord in translation_coord(rcoord, data, direction) {
            let neighbours = data.get_neighbours(t_rcoord);
            match neighbours.get(*direction as usize).unwrap() {
                Some((nc, v)) => match v {
                    '.' => {
                        // *c = *nc;
                        *data.grid.get_mut(&t_rcoord).unwrap() = '.';
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
}

fn translation_coord(rcoord: &Coord, data: &mut Data, direction: &TiltDirection) -> Vec<Coord> {
    match direction {
        TiltDirection::East => (rcoord.x..data.length_x as isize)
            .map(|x| Coord::from((x, rcoord.y)))
            .collect(),
        TiltDirection::South => (rcoord.y..data.length_y as isize)
            .map(|y| Coord::from((rcoord.x, y)))
            .collect(),
        TiltDirection::West => ((0..=rcoord.x).rev())
            .map(|x| Coord::from((x, rcoord.y)))
            .collect(),
        TiltDirection::North => ((0..=rcoord.y).rev())
            .map(|y| Coord::from((rcoord.x, y)))
            .collect(),
    }
}

fn west_sort(rrocks: &[Coord]) -> Vec<Coord> {
    rrocks
        .iter()
        .sorted_by(|v1, v2| {
            if v1.x == v2.x {
                v2.y.cmp(&v1.y)
            } else {
                v1.x.cmp(&v2.x)
            }
        })
        .copied()
        .collect::<Vec<Coord>>()
}

fn south_sort(rrocks: &[Coord]) -> Vec<Coord> {
    rrocks
        .iter()
        .sorted_by(|v1, v2| {
            if v1.x == v2.x {
                v2.y.cmp(&v1.y)
            } else {
                v1.x.cmp(&v2.x)
            }
        })
        .copied()
        .collect::<Vec<Coord>>()
}

fn east_sort(rrocks: &[Coord]) -> Vec<Coord> {
    rrocks
        .iter()
        .sorted_by(|v1, v2| {
            if v1.x == v2.x {
                v1.y.cmp(&v2.y)
            } else {
                v2.x.cmp(&v1.x)
            }
        })
        .copied()
        .collect::<Vec<Coord>>()
}

fn north_sort(rrocks: &[Coord]) -> Vec<Coord> {
    rrocks
        .iter()
        .sorted_by(|v1, v2| {
            if v1.x == v2.x {
                v1.y.cmp(&v2.y)
            } else {
                v1.x.cmp(&v2.x)
            }
        })
        .copied()
        .collect::<Vec<Coord>>()
}

fn find_motif(data: &Data, motif: char) -> Vec<Coord> {
    (0..data.length_x)
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
        .collect::<Vec<Coord>>()
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
        assert_eq!(answer, 64);
    }
}
