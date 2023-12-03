use std::collections::HashSet;

use nalgebra::Point2;

const INPUT: &str = include_str!("./day3.txt");

enum Tile {
    Number(u32),
    Empty,
    Symbol(char),
}

struct Input {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct NumberWithLength {
    number: u32,
    length: usize,
    pos: Point2<usize>,
}

impl Input {
    fn get_number_at(&self, pos: Point2<usize>) -> Option<NumberWithLength> {
        let mut chars = Vec::new();
        let mut pos = pos;

        match self.tiles[pos.y * self.width + pos.x] {
            Tile::Number(_) => {}
            _ => {
                return None;
            }
        }

        // Loop towards left until we reach the edge or a non-number tile
        while pos.x > 0 {
            match self.tiles[pos.y * self.width + pos.x - 1] {
                Tile::Number(_) => {
                    pos.x -= 1;
                }
                _ => break,
            }
        }

        while pos.x < self.width {
            match self.tiles[pos.y * self.width + pos.x] {
                Tile::Number(n) => {
                    chars.push(n.to_string());
                    pos.x += 1;
                }
                _ => break,
            }
        }

        if chars.is_empty() {
            return None;
        }

        let number = chars.join("").parse().unwrap();

        Some(NumberWithLength {
            number,
            length: chars.len(),
            pos,
        })
    }
}

fn parse_input(input: &str) -> Input {
    let first_line = input.lines().next().unwrap();
    let width = first_line.len();
    let height = input.lines().count();

    let tiles = input
        .lines()
        .flat_map(|line| line.chars())
        .map(|c| match c {
            '.' => Tile::Empty,
            c if c.is_digit(10) => Tile::Number(c.to_digit(10).unwrap()),
            c => Tile::Symbol(c),
        })
        .collect::<Vec<_>>();

    Input {
        width,
        height,
        tiles,
    }
}

pub fn a() {
    let input = parse_input(INPUT);

    let mut i = 0;
    let mut sum = 0;

    while i < input.tiles.len() {
        let x = i % input.width;
        let y = i / input.width;
        let pos = Point2::new(x, y);

        let Some(num) = input.get_number_at(pos) else {
            i += 1;
            continue;
        };

        // Find all adjacent tiles for each position in the number
        let mut all_adjacent_tiles = HashSet::new();

        for x in pos.x..pos.x + num.length {
            let pos = Point2::new(x, pos.y);
            all_adjacent_tiles.extend(adjacent_tiles(&input, pos));
        }

        // If any adjacent tile is a symbol, add the number to the sum
        let adjacent_symbol = all_adjacent_tiles
            .iter()
            .find(|pos| matches!(input.tiles[pos.y * input.width + pos.x], Tile::Symbol(_)));

        if let Some(_) = adjacent_symbol {
            sum += num.number;
        }

        i += num.length;
    }

    println!("Day3a: {}", sum);
}

pub fn b() {
    let input = parse_input(INPUT);

    let value = input
        .tiles
        .iter()
        .enumerate()
        .filter_map(|(i, tile)| match tile {
            Tile::Symbol('*') => Some(i),
            _ => None,
        })
        .filter_map(|i| {
            let x = i % input.width;
            let y = i / input.width;
            let pos = Point2::new(x, y);

            let adjacent_tiles = adjacent_tiles(&input, pos);
            let adjacent_numbers = adjacent_tiles
                .iter()
                .filter_map(|pos| input.get_number_at(*pos))
                .collect::<HashSet<_>>();
            if adjacent_numbers.len() == 2 {
                Some(
                    adjacent_numbers
                        .into_iter()
                        .map(|n| n.number as u64)
                        .product::<u64>(),
                )
            } else {
                None
            }
        })
        .sum::<u64>();

    println!("Day3b: {}", value);
}

fn adjacent_tiles(input: &Input, pos: Point2<usize>) -> Vec<Point2<usize>> {
    let width = input.width;
    let height = input.height;
    let x = pos.x;
    let y = pos.y;
    let mut adjacent_tiles = Vec::new();

    // Find adjacent tiles for position, including diagonals

    if x > 0 {
        adjacent_tiles.push(Point2::new(x - 1, y));
    }

    if x < width - 1 {
        adjacent_tiles.push(Point2::new(x + 1, y));
    }

    if y > 0 {
        adjacent_tiles.push(Point2::new(x, y - 1));
    }

    if y < height - 1 {
        adjacent_tiles.push(Point2::new(x, y + 1));
    }

    if x > 0 && y > 0 {
        adjacent_tiles.push(Point2::new(x - 1, y - 1));
    }

    if x < width - 1 && y > 0 {
        adjacent_tiles.push(Point2::new(x + 1, y - 1));
    }

    if x > 0 && y < height - 1 {
        adjacent_tiles.push(Point2::new(x - 1, y + 1));
    }

    if x < width - 1 && y < height - 1 {
        adjacent_tiles.push(Point2::new(x + 1, y + 1));
    }

    adjacent_tiles
}
