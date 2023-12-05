use std::{collections::HashMap, ops::Range};

use nom::{
    bytes::complete::tag, character::complete::multispace1, multi::separated_list1, sequence::pair,
    *,
};

fn read_input(input: Option<&str>) -> String {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };

    input.to_string()
}

#[derive(Debug)]
struct Data {
    seeds: Vec<usize>,
    seed_to_soil: HashMap<Range<usize>, Range<usize>>,
    soil_to_fertilizer: HashMap<Range<usize>, Range<usize>>,
    fertilizer_to_water: HashMap<Range<usize>, Range<usize>>,
    water_to_light: HashMap<Range<usize>, Range<usize>>,
    light_to_temperature: HashMap<Range<usize>, Range<usize>>,
    temperature_to_humidity: HashMap<Range<usize>, Range<usize>>,
    humidity_to_location: HashMap<Range<usize>, Range<usize>>,
}

fn parse_range(input: &str) -> IResult<&str, (Range<usize>, Range<usize>)> {
    let (input, range) = separated_list1(tag(" "), nom::character::complete::u64)(input)?;
    let range_src = range[1] as usize..(range[1] as usize + range[2] as usize);
    let range_dst = range[0] as usize..(range[0] as usize + range[2] as usize);
    Ok((input, (range_src, range_dst)))
}

fn parse_map(input: &str) -> IResult<&str, HashMap<Range<usize>, Range<usize>>> {
    let (input, range_list) = separated_list1(tag("\n"), parse_range)(input)?;
    let hash_map = range_list
        .iter()
        .map(|(src, dst)| (src.clone(), dst.clone()))
        .collect::<HashMap<Range<usize>, Range<usize>>>();
    Ok((input, hash_map))
}

fn parse(input: &str) -> IResult<&str, Data> {
    let (input, seeds) = pair(
        tag("seeds: "),
        separated_list1(multispace1, nom::character::complete::u64),
    )(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("seed-to-soil map:\n")(input)?;
    // let (input, _) = multispace1(input)?;
    let (input, seed_to_soil) = parse_map(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("soil-to-fertilizer map:\n")(input)?;
    let (input, soil_to_fertilizer) = parse_map(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("fertilizer-to-water map:\n")(input)?;
    let (input, fertilizer_to_water) = parse_map(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("water-to-light map:\n")(input)?;
    let (input, water_to_light) = parse_map(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("light-to-temperature map:\n")(input)?;
    let (input, light_to_temperature) = parse_map(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("temperature-to-humidity map:\n")(input)?;
    let (input, temperature_to_humidity) = parse_map(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("humidity-to-location map:\n")(input)?;
    let (input, humidity_to_location) = parse_map(input)?;

    let seeds = seeds.1.iter().map(|x| *x as usize).collect();
    Ok((
        input,
        Data {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        },
    ))
}

fn run(input: String) -> usize {
    let (_, data) = parse(&input).unwrap();
    dbg!(&data);

    let locations = data
        .seeds
        .iter()
        .map(|seed| get_location(&data, *seed))
        .collect::<Vec<usize>>();

    dbg!(&locations);
    *locations.iter().min().unwrap()
}

fn get_location(data: &Data, src: usize) -> usize {
    let mut dst = get_map_location(&data.seed_to_soil, src);
    dst = get_map_location(&data.soil_to_fertilizer, dst);
    dst = get_map_location(&data.fertilizer_to_water, dst);
    dst = get_map_location(&data.water_to_light, dst);
    dst = get_map_location(&data.light_to_temperature, dst);
    dst = get_map_location(&data.temperature_to_humidity, dst);
    get_map_location(&data.humidity_to_location, dst)
}

fn get_map_location(map: &HashMap<Range<usize>, Range<usize>>, src: usize) -> usize {
    for range_src in map.keys() {
        let pos = range_src.clone().position(|p| p == src);
        if let Some(pos) = pos {
            return map.get(range_src).unwrap().clone().nth(pos).unwrap();
        }
    }
    src
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
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 35);
    }
}
