use std::io::BufRead;

use nom::{
    character::complete::{digit1, one_of, space1},
    multi::many1,
    IResult,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

const ALL_CARDS: [Card; 13] = [
    Card::J,
    Card::Two,
    Card::Three,
    Card::Four,
    Card::Five,
    Card::Six,
    Card::Seven,
    Card::Eight,
    Card::Nine,
    Card::T,
    Card::Q,
    Card::K,
    Card::A,
];

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u64,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandClass {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, cards) = many1(one_of("AJQKT0123456789"))(input)?;
    let cards = cards
        .iter()
        .map(|c| match c {
            'A' => Card::A,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'T' => Card::T,
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            _ => panic!("Invalid card"),
        })
        .collect();

    let (input, _) = space1(input)?;
    let (input, bid) = digit1(input)?;
    let bid = bid.parse::<u64>().unwrap();

    Ok((input, Hand { cards, bid }))
}

fn hand_class(hand: &Hand) -> HandClass {
    let mut counts = [0; 13];
    for card in hand.cards.iter() {
        counts[*card as usize] += 1;
    }

    let mut counts = counts.iter().enumerate().collect::<Vec<_>>();
    counts.sort_by(|a, b| b.1.cmp(a.1));

    let mut counts = counts
        .iter()
        .map(|(_, count)| **count)
        .filter(|x| *x > 0)
        .collect::<Vec<_>>();
    counts.sort_by(|a, b| b.cmp(a));

    match counts.as_slice() {
        [5] => HandClass::FiveOfAKind,
        [4, 1] => HandClass::FourOfAKind,
        [3, 2] => HandClass::FullHouse,
        [3, 1, 1] => HandClass::ThreeOfAKind,
        [2, 2, 1] => HandClass::TwoPair,
        [2, 1, 1, 1] => HandClass::OnePair,
        [1, 1, 1, 1, 1] => HandClass::HighCard,
        _ => panic!("Draw"),
    }
}

fn cmp_hand<'a, 'b>(lhs: &'a Hand, rhs: &'b Hand) -> std::cmp::Ordering {
    let kind_lhs = hand_class(lhs);
    let kind_rhs = hand_class(rhs);

    if kind_lhs != kind_rhs {
        return kind_lhs.cmp(&kind_rhs);
    }

    for (lhs, rhs) in lhs.cards.iter().zip(rhs.cards.iter()) {
        if lhs != rhs {
            return lhs.cmp(rhs);
        }
    }

    panic!("Invalid hands");
}

fn cmp_hand2<'a, 'b>(lhs: &'a Hand, rhs: &'b Hand) -> std::cmp::Ordering {
    let best_kind_lhs = ALL_CARDS
        .iter()
        .map(|newj| {
            let cards_corrected = lhs
                .cards
                .iter()
                .map(|c| if c == &Card::J { *newj } else { *c })
                .collect();
            Hand {
                cards: cards_corrected,
                bid: lhs.bid,
            }
        })
        .map(|h| hand_class(&h))
        .max();

    let best_kind_rhs = ALL_CARDS
        .iter()
        .map(|newj| {
            let cards_corrected = rhs
                .cards
                .iter()
                .map(|c| if c == &Card::J { *newj } else { *c })
                .collect();
            Hand {
                cards: cards_corrected,
                bid: rhs.bid,
            }
        })
        .map(|h| hand_class(&h))
        .max();

    if best_kind_lhs != best_kind_rhs {
        return best_kind_lhs.cmp(&best_kind_rhs);
    }

    for (lhs, rhs) in lhs.cards.iter().zip(rhs.cards.iter()) {
        if lhs != rhs {
            return lhs.cmp(rhs);
        }
    }

    panic!("Invalid hands");
}

fn main() {
    let mut hands: Vec<Hand> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| parse_hand(&line.unwrap()).unwrap().1)
        .collect();

    hands.sort_by(cmp_hand);

    println!(
        "Part 1: {}",
        hands
            .iter()
            .enumerate()
            .map(|(i, h)| h.bid * (i as u64 + 1))
            .sum::<u64>()
    );

    hands.sort_by(cmp_hand2);

    println!(
        "Part 2: {}",
        hands
            .iter()
            .enumerate()
            .map(|(i, h)| h.bid * (i as u64 + 1))
            .sum::<u64>()
    );
}
