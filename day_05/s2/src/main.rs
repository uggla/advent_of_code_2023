use std::collections::BTreeMap;
use std::collections::HashSet;

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
struct Deck {
    id: u32,
    cards: Vec<u32>,
    solutions: HashSet<u32>,
}

fn parse(input: Vec<String>) -> Vec<Deck> {
    let mut deck = Vec::new();
    for line in input {
        let line_split = line.split(':').collect::<Vec<&str>>();
        let id = line_split[0]
            .replace("Card ", "")
            .trim()
            .parse::<u32>()
            .unwrap();
        let data = line_split[1].split('|').collect::<Vec<&str>>();
        let cards = data[0]
            .trim()
            .split_whitespace()
            .map(|x| x.trim().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let solutions = data[1]
            .trim()
            .split_whitespace()
            .map(|x| x.trim().parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        deck.push(Deck {
            id,
            cards,
            solutions,
        })
    }
    dbg!(deck)
}

fn run(input: Vec<String>) -> usize {
    let deck = parse(input);
    let sol = deck
        .iter()
        .map(|d| {
            d.cards.iter().fold((0usize, 0usize), |mut matches, c| {
                //
                if d.solutions.contains(&c) {
                    matches.1 += 1;
                }
                (d.id as usize, matches.1)
            })
        })
        .collect::<BTreeMap<usize, usize>>();

    // Create initial hand by adding all original cards
    let initial_hand = sol
        .keys()
        .map(|id| (*id, 1))
        .collect::<BTreeMap<usize, usize>>();

    let instances = sol
        .iter()
        // .enumerate()
        .fold(initial_hand, |mut hand, (id, card_solutions)| {
            let card_instance = *hand.get(&id).unwrap();

            for copy_id in (id + 1)..(id + 1 + *card_solutions) {
                hand.entry(copy_id).and_modify(|value| {
                    *value += card_instance;
                });
            }
            hand
        });

    dbg!(&sol);
    dbg!(&instances);

    instances.values().sum()
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
    fn test_run() {
        let input = parse_input(Some(indoc!(
            "
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 30);
    }
}
