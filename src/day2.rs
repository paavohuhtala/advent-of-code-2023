use std::collections::HashMap;

use scan_fmt::scan_fmt_some;

const INPUT: &str = include_str!("./day2.txt");

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Color {
    Red = 0,
    Green,
    Blue,
}

#[derive(Debug)]
struct Game {
    id: u32,
    cube_sets: Vec<HashMap<Color, u32>>,
}

fn parse_game(line: &str) -> Game {
    let (id, cube_sets) = line.split_once(": ").unwrap();
    let id: u32 = scan_fmt_some!(id, "Game {d}", u32).unwrap();
    let cube_sets = cube_sets
        .split("; ")
        .map(|cube_set| {
            cube_set
                .split(", ")
                .map(|count_and_color| {
                    let (count, color) = count_and_color.split_once(' ').unwrap();
                    let count: u32 = count.parse().unwrap();
                    let color = match color {
                        "red" => Color::Red,
                        "green" => Color::Green,
                        "blue" => Color::Blue,
                        _ => unreachable!(),
                    };
                    (color, count)
                })
                .collect::<HashMap<_, _>>()
        })
        .collect::<Vec<_>>();
    Game { id, cube_sets }
}

pub fn a() {
    let games = INPUT.lines().map(parse_game).collect::<Vec<_>>();

    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;

    let possible_games = games
        .iter()
        .filter(|game| {
            game.cube_sets.iter().all(|cube_set| {
                cube_set.get(&Color::Red).unwrap_or(&0) <= &red_limit
                    && cube_set.get(&Color::Green).unwrap_or(&0) <= &green_limit
                    && cube_set.get(&Color::Blue).unwrap_or(&0) <= &blue_limit
            })
        })
        .map(|game| game.id)
        .sum::<u32>();

    println!("Day2a: {}", possible_games)
}

pub fn b() {
    let games = INPUT.lines().map(parse_game).collect::<Vec<_>>();

    let sum_of_cubes = games
        .iter()
        .map(|game| {
            game.cube_sets
                .iter()
                .fold([0u32; 3], |mut acc, cube_set| {
                    for (color, count) in cube_set {
                        acc[*color as usize] = acc[*color as usize].max(*count);
                    }

                    acc
                })
                .iter()
                .product::<u32>()
        })
        .sum::<u32>();

    println!("Day2b: {}", sum_of_cubes);
}
