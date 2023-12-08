use std::collections::HashMap;

use nom::{character::complete::multispace1, multi::separated_list1, *};

fn read_input(input: Option<&str>) -> String {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };

    input.to_string()
}

fn get_card_value(card: char) -> u32 {
    match card {
        //A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => panic!("Unexpected card value"),
    }
}

fn get_hand_value_and_card_values(hand: &str) -> (u32, Vec<u32>) {
    let mut hand_value = 0;
    let mut card_values = Vec::new();

    let mut card_occurences: HashMap<char, u32> = HashMap::new();

    for card in hand.chars() {
        // Create a map to count card types and define hand value
        let count = card_occurences.entry(card).or_insert(0);
        *count += 1;
        // Create a vector of card values
        card_values.push(get_card_value(card));
    }

    // Do we have jokers ?
    match card_occurences.get(&'J') {
        Some(nb_occurences) => match nb_occurences {
            5 => {
                card_occurences.insert('A', 5);
                card_occurences.remove(&'J').unwrap();
            }
            4 => {
                find_best_hand(&mut card_occurences, 4);
            }
            3 => {
                find_best_hand(&mut card_occurences, 3);
            }
            2 => find_best_hand(&mut card_occurences, 2),
            1 => find_best_hand(&mut card_occurences, 1),
            _ => panic!("Unexpected number of jokers"),
        },
        None => {
            dbg!("no jokers");
        }
    };

    for count in card_occurences.values() {
        match count {
            5 => hand_value += 500,
            4 => hand_value += 400,
            3 => hand_value += 300,
            2 => hand_value += 20,
            1 => {}
            _ => panic!("Unexpected card count"),
        }
    }

    dbg!(&card_occurences);
    dbg!((hand_value, card_values))
}

fn find_best_hand(card_occurences: &mut HashMap<char, u32>, occurences: u32) {
    let mut other_cards = card_occurences
        .iter()
        .map(|(k, v)| (*k, *v))
        .filter(|(k, _v)| *k != 'J')
        .collect::<Vec<_>>();
    other_cards.sort_by_key(|v| (v.1, get_card_value(v.0)));
    let highest_card = other_cards.pop().unwrap();
    card_occurences
        .entry(highest_card.0)
        .and_modify(|value| *value += occurences);
    card_occurences.remove(&'J').unwrap();
}

fn parse(input: &str) -> IResult<&str, HashMap<String, u32>> {
    let (input, hands) = separated_list1(multispace1, parse_line)(input)?;

    let data: HashMap<String, u32> = hands.into_iter().collect();

    Ok((input, data))
}

fn parse_line(input: &str) -> IResult<&str, (String, u32)> {
    let (input, hand) = nom::character::complete::alphanumeric1(input)?;
    let (input, _) = multispace1(input)?;
    let (input, bid) = nom::character::complete::u32(input)?;
    Ok((input, (hand.to_string(), bid)))
}

fn run(input: String) -> usize {
    let (_, data) = parse(&input).unwrap();
    dbg!(&data);

    let mut data = data
        .into_iter()
        .map(|(hand, bid)| ({ (get_hand_value_and_card_values(&hand), hand, bid) }))
        .collect::<Vec<((u32, Vec<u32>), String, u32)>>();

    // Sort by hand value, then by card values
    data.sort_by_key(|((hand_value, card_values), _, _)| (*hand_value, card_values.clone()));

    data.iter()
        .enumerate()
        .inspect(|(i, (value, hand, bid))| {
            dbg!(i, value, hand, bid);
        })
        // index + 1 because rank starts at 1.
        .map(|(i, (_value, _hand, bid))| (i + 1) * *bid as usize)
        .inspect(|i| {
            dbg!(i);
        })
        .sum::<usize>()
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
    fn test_various_j() {
        let input = read_input(Some(indoc!(
            // KJJJJ 2  10
            // JJJJJ 1  4
            // KQJJJ 3  9
            // K2JJQ 4  8
            // K23AJ 5  5
            "
            KJJJJ 2
            JJJJJ 1
            KQJJJ 3
            K2JJQ 4
            K23AJ 5
            "
        )));
        let answer = run(input);
        assert_eq!(answer, 36);
    }

    #[test]
    fn test_run() {
        let input = read_input(Some(indoc!(
            "
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 5905);
    }
}
