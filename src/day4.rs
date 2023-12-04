use scan_fmt::scan_fmt_some;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("./day4.txt");

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    winning: HashSet<u32>,
    numbers: HashSet<u32>,
}

fn read_input(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let (card_id, numbers) = line.split_once(": ").unwrap();
            let id = scan_fmt_some!(card_id, "Card {d}", usize).unwrap();

            let (winning, numbers) = numbers.split_once(" | ").unwrap();
            let winning = winning
                .split(" ")
                .filter(|s| !s.is_empty())
                .map(|n| n.parse().unwrap())
                .collect();
            let numbers = numbers
                .split(" ")
                .filter(|s| !s.is_empty())
                .map(|n| n.parse().unwrap())
                .collect();
            Card {
                id,
                winning,
                numbers,
            }
        })
        .collect()
}

pub fn a() {
    let cards = read_input(INPUT);

    let value = cards
        .into_iter()
        .map(|card| {
            let matching_numbers = card.numbers.intersection(&card.winning).count();
            if matching_numbers == 0 {
                0
            } else {
                2u64.pow(matching_numbers as u32 - 1)
            }
        })
        .sum::<u64>();
    println!("Day4a: {}", value);
}

pub fn b() {
    let original_cards = read_input(INPUT);

    fn evaluate_card<'a, 'b>(
        card: &'a Card,
        original_cards: &'a [Card],
        card_cache: &'b mut HashMap<usize, Vec<&'a Card>>,
    ) -> Option<&'b Vec<&'a Card>> {
        if card_cache.contains_key(&card.id) {
            return card_cache.get(&card.id);
        }

        let matching_numbers = card.numbers.intersection(&card.winning).count();
        if matching_numbers == 0 {
            return None;
        }

        let card_index = card.id - 1;

        let matching_indices = (1..=matching_numbers)
            .into_iter()
            .map(|offset| &original_cards[card_index + offset])
            .collect::<Vec<_>>();

        Some(card_cache.entry(card.id).or_insert(matching_indices))
    }

    let mut cards = original_cards.iter().collect::<Vec<_>>();
    let mut card_cache = HashMap::new();
    let mut total_cards = cards.len();

    while let Some(card) = cards.pop() {
        let Some(new_cards) = evaluate_card(&card, &original_cards, &mut card_cache) else {
            continue;
        };

        total_cards += new_cards.len();
        cards.extend(new_cards);
    }

    println!("Day4b: {}", total_cards);
}
