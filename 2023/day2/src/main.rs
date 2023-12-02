use std::{collections::HashMap, io::BufRead};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    IResult,
};

#[derive(PartialEq, PartialOrd, Eq, Debug, Ord, Hash, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

struct Game {
    id: u32,
    sets: Vec<Vec<(Color, u32)>>,
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    let (input, color) = alt((
        map(tag("red"), |_| Color::Red),
        map(tag("green"), |_| Color::Green),
        map(tag("blue"), |_| Color::Blue),
    ))(input)?;
    Ok((input, color))
}

fn parse_result(input: &str) -> IResult<&str, (Color, u32)> {
    // Parse a string of the form "10 red"
    let (input, count) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = parse_color(input)?;
    Ok((input, (color, count)))
}

fn parse_set(input: &str) -> IResult<&str, Vec<(Color, u32)>> {
    nom::multi::separated_list1(tag(", "), parse_result)(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, sets) = nom::multi::separated_list1(tag("; "), parse_set)(input)?;
    Ok((input, Game { id, sets }))
}

fn main() {
    let limits = HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);

    let games: Vec<Game> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (_, game) = parse_game(&line).unwrap();
            game
        })
        .collect();

    let mut sum = 0;
    for game in &games {
        let mut valid = true;
        for set in &game.sets {
            for (color, count) in set {
                if *count > limits[color] {
                    valid = false;
                    break;
                }
            }
        }

        if valid {
            sum += game.id;
        }
    }

    println!("Part1 {}", sum);

    let mut sum = 0;
    for game in &games {
        let mut minimums: HashMap<Color, u32> =
            HashMap::from([(Color::Red, 0), (Color::Green, 0), (Color::Blue, 0)]);

        for set in &game.sets {
            for (color, count) in set {
                if *count > minimums[color] {
                    minimums.insert(*color, *count);
                }
            }
        }

        let power = minimums.values().product::<u32>();
        sum += power;
    }

    println!("Part2 {}", sum);
}
