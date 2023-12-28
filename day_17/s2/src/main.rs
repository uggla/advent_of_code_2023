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

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, Ord, PartialOrd)]
struct Crucible {
    direction: Direction,
    position: Coord,
    nb_moves: usize,
    cost: usize,
}

impl Crucible {
    fn get_next_positions(&self, data: &Data) -> Vec<Crucible> {
        let neighbors = data.get_neighbours(self.position);
        let positions = match self.direction {
            Direction::Right => {
                vec![
                    if let Some((npos, cost)) = neighbors[Direction::Right as usize] {
                        Some(Crucible {
                            direction: Direction::Right,
                            position: npos,
                            nb_moves: self.nb_moves + 1,
                            cost: cost.to_digit(10).unwrap() as usize,
                        })
                    } else {
                        None
                    },
                    if let Some((npos, cost)) = neighbors[Direction::Up as usize] {
                        Some(Crucible {
                            direction: Direction::Up,
                            position: npos,
                            nb_moves: 1,
                            cost: cost.to_digit(10).unwrap() as usize,
                        })
                    } else {
                        None
                    },
                    if let Some((npos, cost)) = neighbors[Direction::Down as usize] {
                        Some(Crucible {
                            direction: Direction::Down,
                            position: npos,
                            nb_moves: 1,
                            cost: cost.to_digit(10).unwrap() as usize,
                        })
                    } else {
                        None
                    },
                ]
            }
            Direction::Down => {
                vec![
                    if let Some((npos, cost)) = neighbors[Direction::Right as usize] {
                        Some(Crucible {
                            direction: Direction::Right,
                            position: npos,
                            nb_moves: 1,
                            cost: cost.to_digit(10).unwrap() as usize,
                        })
                    } else {
                        None
                    },
                    if let Some((npos, cost)) = neighbors[Direction::Left as usize] {
                        Some(Crucible {
                            direction: Direction::Left,
                            position: npos,
                            nb_moves: 1,
                            cost: cost.to_digit(10).unwrap() as usize,
                        })
                    } else {
                        None
                    },
                    if let Some((npos, cost)) = neighbors[Direction::Down as usize] {
                        Some(Crucible {
                            direction: Direction::Down,
                            position: npos,
                            nb_moves: self.nb_moves + 1,
                            cost: cost.to_digit(10).unwrap() as usize,
                        })
                    } else {
                        None
                    },
                ]
            }
            Direction::Left => {
                vec![
                    if let Some((npos, cost)) = neighbors[Direction::Left as usize] {
                        Some(Crucible {
                            direction: Direction::Left,
                            position: npos,
                            nb_moves: self.nb_moves + 1,
                            cost: cost.to_digit(10).unwrap() as usize,
                        })
                    } else {
                        None
                    },
                    if let Some((npos, cost)) = neighbors[Direction::Up as usize] {
                        Some(Crucible {
                            direction: Direction::Up,
                            position: npos,
                            nb_moves: 1,
                            cost: cost.to_digit(10).unwrap() as usize,
                        })
                    } else {
                        None
                    },
                    if let Some((npos, cost)) = neighbors[Direction::Down as usize] {
                        Some(Crucible {
                            direction: Direction::Down,
                            position: npos,
                            nb_moves: 1,
                            cost: cost.to_digit(10).unwrap() as usize,
                        })
                    } else {
                        None
                    },
                ]
            }
            Direction::Up => {
                vec![
                    if let Some((npos, cost)) = neighbors[Direction::Right as usize] {
                        Some(Crucible {
                            direction: Direction::Right,
                            position: npos,
                            nb_moves: 1,
                            cost: cost.to_digit(10).unwrap() as usize,
                        })
                    } else {
                        None
                    },
                    if let Some((npos, cost)) = neighbors[Direction::Left as usize] {
                        Some(Crucible {
                            direction: Direction::Left,
                            position: npos,
                            nb_moves: 1,
                            cost: cost.to_digit(10).unwrap() as usize,
                        })
                    } else {
                        None
                    },
                    if let Some((npos, cost)) = neighbors[Direction::Up as usize] {
                        Some(Crucible {
                            direction: Direction::Up,
                            position: npos,
                            nb_moves: self.nb_moves + 1,
                            cost: cost.to_digit(10).unwrap() as usize,
                        })
                    } else {
                        None
                    },
                ]
            }
        };
        let mut positions: Vec<Crucible> = positions.iter().filter_map(|c| *c).collect();
        // Remove positions that exceed 3 moves in the same direction
        positions.retain(|c| c.nb_moves < 4);
        positions
    }
}

fn run(input: String) -> usize {
    let (_, data) = parse(&input).unwrap();
    // dbg!(&data);

    let mut to_process: VecDeque<Crucible> = VecDeque::new();
    let mut g: DiGraphMap<Crucible, usize> = DiGraphMap::new();

    let start = [Crucible {
        direction: Direction::Right,
        position: Coord::from((0, 0)),
        nb_moves: 0,
        cost: data.grid[&Coord::from((0, 0)) as &Coord]
            .to_digit(10)
            .unwrap() as usize,
    }];

    start.iter().for_each(|c| {
        to_process.push_back(*c);
    });

    let mut seen = HashSet::new();
    while !to_process.is_empty() {
        let crucible = to_process.pop_front().unwrap();
        let new_positions = crucible.get_next_positions(&data);
        if !seen.contains(&crucible) {
            seen.insert(crucible);
            for c in new_positions.iter() {
                g.add_edge(crucible, *c, c.cost);
                if c.position
                    != Coord::from((data.length_x as isize - 1, data.length_y as isize - 1))
                {
                    to_process.push_back(*c);
                }
            }
        }
    }

    // println!("{:?}", Dot::new(&g));

    let path = dijkstra(&g, start[0], None, |e| *e.weight());

    let path: Vec<(&Crucible, &usize)> = path
        .iter()
        .filter(|p| {
            p.0.position == Coord::from((data.length_x as isize - 1, data.length_y as isize - 1))
        })
        .collect();

    dbg!(path.iter().map(|p| *p.1).min().unwrap())
}

#[allow(dead_code)]
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
            2413432311323
            3215453535623
            3255245654254
            3446585845452
            4546657867536
            1438598798454
            4457876987766
            3637877979653
            4654967986887
            4564679986453
            1224686865563
            2546548887735
            4322674655533
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 102);
    }
}
