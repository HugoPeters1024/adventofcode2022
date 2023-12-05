use std::{io::BufRead, collections::HashSet};

use nom::{IResult, character::complete::{digit1, space1}, combinator::map_res, bytes::complete::tag, multi::separated_list1};

#[derive(Clone)]
struct Card {
    card_numbers: Vec<u32>,
    winning_numbers: Vec<u32>,
    copies: u32,
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = parse_u32(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = space1(input)?;
    let (input, card_numbers) = separated_list1(space1, parse_u32)(input)?;
    let (input, _) = space1(input)?;
    let (input, _) =  tag("|")(input)?;
    let (input, _) = space1(input)?;
    let (input, winning_numbers) = separated_list1(space1, parse_u32)(input)?;
    Ok((input, Card {
        card_numbers,
        winning_numbers,
        copies: 1,
    }))
}

fn main() {
    let mut sum = 0;

    let mut cards: Vec<Card> = std::io::stdin().lock().lines().map(|l| parse_card(&l.unwrap()).unwrap().1).collect();

    for card in &cards {
        let winning_set: HashSet<u32> = HashSet::from_iter(card.winning_numbers.clone().into_iter());

        let mut matches = 0;
        for n in &card.card_numbers {
            if winning_set.contains(n) {
                matches += 1;
            }
        }

        if matches >= 1 {
            sum += 1 << (matches-1);
        }
    }

    println!("Part 1: {}", sum);

    for i in 0..cards.len() {
        let card = &cards[i];

        let winning_set: HashSet<u32> = HashSet::from_iter(card.winning_numbers.clone().into_iter());

        let mut matches = 0;
        for n in &card.card_numbers {
            if winning_set.contains(n) {
                matches += 1;
            }
        }

        for di in 1..=matches {
            cards[i+di].copies += cards[i].copies
        }
    }

    println!("Part 2: {}", cards.iter().map(|c| c.copies).sum::<u32>());
}
