use core::panic;
use std::{
    collections::{HashMap, HashSet, VecDeque},
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

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Beam {
    direction: Direction,
    pos: Coord,
}

impl Beam {
    fn new(direction: Direction, pos: Coord) -> Self {
        Self { direction, pos }
    }

    fn advence(
        &mut self,
        data: &Data,
        energized_tiles: &mut HashSet<Coord>,
        seen: &mut HashSet<(Coord, Direction)>,
    ) -> Option<Vec<Beam>> {
        let neighbors = data.get_neighbours(self.pos);
        match self.direction {
            Direction::Right => {
                if let Some(nc) = neighbors[Direction::Right as usize] {
                    match nc.1 {
                        '|' => {
                            self.pos = nc.0;
                            energized_tiles.insert(nc.0);
                            if !seen.contains(&(nc.0, Direction::Right)) {
                                seen.insert((nc.0, Direction::Right));
                                return Some(vec![
                                    Beam::new(Direction::Up, nc.0),
                                    Beam::new(Direction::Down, nc.0),
                                ]);
                            }
                        }
                        '-' => {
                            self.pos = nc.0;
                            energized_tiles.insert(nc.0);
                            if !seen.contains(&(nc.0, Direction::Right)) {
                                seen.insert((nc.0, Direction::Right));
                                return Some(vec![self.clone()]);
                            }
                        }
                        '/' => {
                            self.direction = Direction::Up;
                            self.pos = nc.0;
                            energized_tiles.insert(nc.0);
                            if !seen.contains(&(nc.0, Direction::Right)) {
                                seen.insert((nc.0, Direction::Right));
                                return Some(vec![self.clone()]);
                            }
                        }
                        '\\' => {
                            self.direction = Direction::Down;
                            self.pos = nc.0;
                            energized_tiles.insert(nc.0);
                            if !seen.contains(&(nc.0, Direction::Right)) {
                                seen.insert((nc.0, Direction::Right));
                                return Some(vec![self.clone()]);
                            }
                        }
                        '.' => {
                            self.pos = nc.0;
                            energized_tiles.insert(nc.0);
                            if !seen.contains(&(nc.0, Direction::Right)) {
                                seen.insert((nc.0, Direction::Right));
                                return Some(vec![self.clone()]);
                            }
                        }
                        _ => {
                            panic!("Unexpected char: {}", nc.1)
                        }
                    }
                }
            }
            Direction::Down => {
                if let Some(nc) = neighbors[Direction::Down as usize] {
                    match nc.1 {
                        '-' => {
                            self.pos = nc.0;
                            energized_tiles.insert(nc.0);
                            if !seen.contains(&(nc.0, Direction::Down)) {
                                seen.insert((nc.0, Direction::Down));
                                return Some(vec![
                                    Beam::new(Direction::Right, nc.0),
                                    Beam::new(Direction::Left, nc.0),
                                ]);
                            }
                        }
                        '|' => {
                            self.pos = nc.0;
                            energized_tiles.insert(nc.0);
                            if !seen.contains(&(nc.0, Direction::Down)) {
                                seen.insert((nc.0, Direction::Down));
                                return Some(vec![self.clone()]);
                            }
                        }
                        '/' => {
                            self.direction = Direction::Left;
                            self.pos = nc.0;
                            energized_tiles.insert(nc.0);
                            if !seen.contains(&(nc.0, Direction::Down)) {
                                seen.insert((nc.0, Direction::Down));
                                return Some(vec![self.clone()]);
                            }
                        }
                        '\\' => {
                            self.direction = Direction::Right;
                            self.pos = nc.0;
                            energized_tiles.insert(nc.0);
                            if !seen.contains(&(nc.0, Direction::Down)) {
                                seen.insert((nc.0, Direction::Down));
                                return Some(vec![self.clone()]);
                            }
                        }
                        '.' => {
                            self.pos = nc.0;
                            energized_tiles.insert(nc.0);
                            if !seen.contains(&(nc.0, Direction::Down)) {
                                seen.insert((nc.0, Direction::Down));
                                return Some(vec![self.clone()]);
                            }
                        }
                        _ => {
                            panic!("Unexpected char: {}", nc.1)
                        }
                    }
                }
            }
            Direction::Left => {
                if let Some(nc) = neighbors[Direction::Left as usize] {
                    match nc.1 {
                        '|' => {
                            self.pos = nc.0;
                            energized_tiles.insert(nc.0);
                            if !seen.contains(&(nc.0, Direction::Left)) {
                                seen.insert((nc.0, Direction::Left));
                                return Some(vec![
                                    Beam::new(Direction::Up, nc.0),
                                    Beam::new(Direction::Down, nc.0),
                                ]);
                            }
                        }
                        '-' => {
                            self.pos = nc.0;
                            energized_tiles.insert(nc.0);
                            if !seen.contains(&(nc.0, Direction::Left)) {
                                seen.insert((nc.0, Direction::Left));
                                return Some(vec![self.clone()]);
                            }
                        }
                        '/' => {
                            self.direction = Direction::Down;
                            self.pos = nc.0;
                            energized_tiles.insert(nc.0);
                            if !seen.contains(&(nc.0, Direction::Left)) {
                                seen.insert((nc.0, Direction::Left));
                                return Some(vec![self.clone()]);
                            }
                        }
                        '\\' => {
                            self.direction = Direction::Up;
                            self.pos = nc.0;
                            energized_tiles.insert(nc.0);
                            if !seen.contains(&(nc.0, Direction::Left)) {
                                seen.insert((nc.0, Direction::Left));
                                return Some(vec![self.clone()]);
                            }
                        }
                        '.' => {
                            self.pos = nc.0;
                            energized_tiles.insert(nc.0);
                            if !seen.contains(&(nc.0, Direction::Left)) {
                                seen.insert((nc.0, Direction::Left));
                                return Some(vec![self.clone()]);
                            }
                        }
                        _ => {
                            panic!("Unexpected char: {}", nc.1)
                        }
                    }
                }
            }
            Direction::Up => {
                if let Some(nc) = neighbors[Direction::Up as usize] {
                    match nc.1 {
                        '-' => {
                            self.pos = nc.0;
                            energized_tiles.insert(nc.0);
                            if !seen.contains(&(nc.0, Direction::Up)) {
                                seen.insert((nc.0, Direction::Up));
                                return Some(vec![
                                    Beam::new(Direction::Right, nc.0),
                                    Beam::new(Direction::Left, nc.0),
                                ]);
                            }
                        }
                        '|' => {
                            self.pos = nc.0;
                            energized_tiles.insert(nc.0);
                            if !seen.contains(&(nc.0, Direction::Up)) {
                                seen.insert((nc.0, Direction::Up));
                                return Some(vec![self.clone()]);
                            }
                        }
                        '/' => {
                            self.direction = Direction::Right;
                            self.pos = nc.0;
                            energized_tiles.insert(nc.0);
                            if !seen.contains(&(nc.0, Direction::Up)) {
                                seen.insert((nc.0, Direction::Up));
                                return Some(vec![self.clone()]);
                            }
                        }
                        '\\' => {
                            self.direction = Direction::Left;
                            self.pos = nc.0;
                            energized_tiles.insert(nc.0);
                            if !seen.contains(&(nc.0, Direction::Up)) {
                                seen.insert((nc.0, Direction::Up));
                                return Some(vec![self.clone()]);
                            }
                        }
                        '.' => {
                            self.pos = nc.0;
                            energized_tiles.insert(nc.0);
                            if !seen.contains(&(nc.0, Direction::Up)) {
                                seen.insert((nc.0, Direction::Up));
                                return Some(vec![self.clone()]);
                            }
                        }
                        _ => {
                            panic!("Unexpected char: {}", nc.1)
                        }
                    }
                }
            }
        }
        None
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
    let (_, data) = parse(&input).unwrap();
    // dbg!(&data);

    print_text_map(
        &data
            .grid
            .iter()
            .map(|c| (c.0.x as usize, c.0.y as usize, *c.1))
            .collect::<Vec<(usize, usize, char)>>(),
        data.length_x,
        data.length_y,
    );

    let left_tiles =
        (0..data.length_y).map(|r| Beam::new(Direction::Right, Coord::from((0isize, r as isize))));
    let right_tiles = (0..data.length_y).map(|r| {
        Beam::new(
            Direction::Left,
            Coord::from((data.length_x as isize - 1, r as isize)),
        )
    });
    let up_tiles =
        (0..data.length_x).map(|c| Beam::new(Direction::Down, Coord::from((c as isize, 0isize))));
    let down_tiles = (0..data.length_x).map(|c| {
        Beam::new(
            Direction::Up,
            Coord::from((c as isize, data.length_y as isize - 1)),
        )
    });

    let et: Vec<usize> = left_tiles
        .chain(right_tiles)
        .chain(up_tiles)
        .chain(down_tiles)
        .map(|b| {
            let mut beams: VecDeque<Beam> = VecDeque::new();
            beams.push_back(b.clone());
            find_energized_tiles(beams, &data)
        })
        .collect();

    dbg!(*et.iter().max().unwrap())
}

fn find_energized_tiles(mut beams: VecDeque<Beam>, data: &Data) -> usize {
    let mut energized_tiles: HashSet<Coord> = HashSet::new();
    let mut seen: HashSet<(Coord, Direction)> = HashSet::new();

    // We need to mark the first tile because it is in the grid.
    energized_tiles.insert(beams[0].pos);

    while !beams.is_empty() {
        if let Some(new_beams) =
            beams
                .pop_front()
                .unwrap()
                .advence(data, &mut energized_tiles, &mut seen)
        {
            new_beams.iter().for_each(|b| beams.push_back(b.clone()));
        }
    }

    energized_tiles.len()
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
    use rstest::rstest;

    #[test]
    fn test_fake() {
        assert_eq!(1, 1);
    }

    #[rstest]
    // ...
    // .>.
    // ...
    #[case(Direction::Right, "..\n..\n..\n", None)]
    #[case(Direction::Right, "...\n..|\n...\n", Some(vec![Beam::new(Direction::Up, Coord::from((2,1))),
                                                          Beam::new(Direction::Down, Coord::from((2,1)))]
    ))]
    #[case(Direction::Right,"...\n..-\n...\n", Some(vec![Beam::new(Direction::Right, Coord::from((2,1)))]))]
    #[case(Direction::Right,"...\n../\n...\n", Some(vec![Beam::new(Direction::Up, Coord::from((2,1)))]))]
    #[case(Direction::Right,"...\n..\\n...\n", Some(vec![Beam::new(Direction::Down, Coord::from((2,1)))]))]
    #[case(Direction::Right,"...\n...\n...\n", Some(vec![Beam::new(Direction::Right, Coord::from((2,1)))]))]
    #[case(Direction::Up,".-.\n...\n...\n", Some(vec![Beam::new(Direction::Right, Coord::from((1,0))),
                                                      Beam::new(Direction::Left, Coord::from((1,0)))]
    ))]
    #[case(Direction::Up,".|.\n...\n...\n", Some(vec![Beam::new(Direction::Up, Coord::from((1,0)))]))]
    #[case(Direction::Up,"./.\n...\n...\n", Some(vec![Beam::new(Direction::Right, Coord::from((1,0)))]))]
    #[case(Direction::Up,".\\.\n...\n...\n", Some(vec![Beam::new(Direction::Left, Coord::from((1,0)))]))]
    #[case(Direction::Up,"...\n...\n...\n", Some(vec![Beam::new(Direction::Up, Coord::from((1,0)))]))]
    #[case(Direction::Down,"...\n...\n.-.\n", Some(vec![Beam::new(Direction::Right, Coord::from((1,2))),
                                                        Beam::new(Direction::Left, Coord::from((1,2)))]
    ))]
    #[case(Direction::Down,"...\n...\n.|.\n", Some(vec![Beam::new(Direction::Down, Coord::from((1,2)))]))]
    #[case(Direction::Down,"...\n...\n./.\n", Some(vec![Beam::new(Direction::Left, Coord::from((1,2)))]))]
    #[case(Direction::Down,"...\n...\n.\\.\n", Some(vec![Beam::new(Direction::Right, Coord::from((1,2)))]))]
    #[case(Direction::Down,"...\n...\n...\n", Some(vec![Beam::new(Direction::Down, Coord::from((1,2)))]))]
    #[case(Direction::Left,"...\n|..\n...\n", Some(vec![Beam::new(Direction::Up, Coord::from((0,1))),
                                                         Beam::new(Direction::Down, Coord::from((0,1)))]
    ))]
    #[case(Direction::Left,"...\n-..\n...\n", Some(vec![Beam::new(Direction::Left, Coord::from((0,1)))]))]
    #[case(Direction::Left,"...\n/..\n...\n", Some(vec![Beam::new(Direction::Down, Coord::from((0,1)))]))]
    #[case(Direction::Left,"...\n\\..\n...\n", Some(vec![Beam::new(Direction::Up, Coord::from((0,1)))]))]
    #[case(Direction::Left,"...\n...\n...\n", Some(vec![Beam::new(Direction::Left, Coord::from((0,1)))]))]
    fn test_advance(
        #[case] d: Direction,
        #[case] input: &str,
        #[case] expected: Option<Vec<Beam>>,
    ) {
        // let input = read_input(Some(s));
        let (_, data) = parse(input).unwrap();
        let mut beams: VecDeque<Beam> = VecDeque::new();
        beams.push_back(Beam::new(d, Coord::from((1, 1))));
        let mut energized_tiles: HashSet<Coord> = HashSet::new();
        let mut seen: HashSet<(Coord, Direction)> = HashSet::new();

        let new_beams = beams
            .pop_front()
            .unwrap()
            .advence(&data, &mut energized_tiles, &mut seen);
        assert_eq!(new_beams, expected);
    }

    #[test]
    fn test_run1() {
        let input = read_input(Some(indoc!(
            r"
            .|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 51);
    }
}
