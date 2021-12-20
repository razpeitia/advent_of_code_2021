use std::collections::HashSet;
use std::iter::FromIterator;
use std::fs;
use std::io;
use std::time::Instant;

type Points = Vec<(i64, i64)>;
static DIRECTIONS: &'static [(i64, i64)] = &[
    (-1, -1), (-1, 0), (-1, 1),
    (0,  -1), (0,  0), (0,  1),
    ( 1, -1), (1,  0), (1,  1),
];

fn parse_input(s : &String) -> (&str, Points) {
    let lines : Vec<&str> = s.trim().lines().collect();
    let input = lines[0];
    let points : Points = s.trim()
        .split("\n\n")
        .last().unwrap()
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().filter_map(move |(j, c)|
                match c {
                    '#' => Some((i as i64, j as i64)),
                    '.' => None,
                    _ => unreachable!()
                })
        }).collect();
    (input, points)
}

fn get_index(i : i64, j : i64, point_index : &HashSet<(i64, i64)>, min_x : i64, max_x : i64, min_y : i64, max_y : i64, rest : u16) -> u16 {
    let mut index : u16 = 0;
    for &(di, dj) in DIRECTIONS {
        let ni = i + di;
        let nj = j + dj;
        if (nj >= min_x && nj <= max_x) && (ni >= min_y && ni <= max_y) {
            let contains = point_index.contains(&(ni, nj)) as u16;
            index <<= 1;
            index |= contains;
        } else {
            index <<= 1;
            index |= rest;
        }
    }
    index
}

fn get_value(i : i64, j : i64, point_index : &HashSet<(i64, i64)>, algo : &str, min_x : i64, max_x : i64, min_y : i64, max_y : i64, rest : u16) -> bool {
    let index = get_index(i, j, point_index, min_x, max_x, min_y, max_y, rest);
    algo.chars().nth(index as usize).unwrap() == '#'
}

fn enhance(algo : &str, input : &Points, rest : u16) -> (u16, Points) {
    let point_index : HashSet<(i64, i64)> = HashSet::from_iter(input.iter().cloned());
    let mut points : Points = Vec::new();
    let min_x = input.iter().map(|p| p.0).min().unwrap() - 1;
    let max_x = input.iter().map(|p| p.0).max().unwrap() + 1;
    let min_y = input.iter().map(|p| p.1).min().unwrap() - 1;
    let max_y = input.iter().map(|p| p.1).max().unwrap() + 1;
    for i in min_y..=max_y {
        for j in min_x..=max_x {
            let value = get_value(i, j, &point_index, algo, min_x+1, max_x-1, min_y+1, max_y-1, rest);
            if value {
                points.push((i, j));
            }
        }
    }
    let mut new_rest = rest;
    if rest == 0 && algo.chars().nth(0).unwrap() == '#' {
        new_rest = 1;
    } else if rest == 1 && algo.chars().nth(511).unwrap() == '.' {
        new_rest = 0
    }
    (new_rest, points)
}

fn part1(algo : &str, input : &Points) -> usize {
    let (rest, new_points) = enhance(algo, input, 0);
    let (_, new_points) = enhance(algo, &new_points, rest);
    new_points.len()
}


fn main() -> Result<(), io::Error>  {
    let s = fs::read_to_string("assets/input.txt")?;
    let (algo, input)  = parse_input(&s);

    let now = Instant::now();
    println!("{:#?}", part1(algo, &input));
    println!("Part 1 took {:?}", now.elapsed());

    Ok(())
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::iter::FromIterator;
    use crate::{parse_input, part1, get_index};

    #[test]
    fn test_parts() {
        let s1 : String = r#"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"#.to_string();
        let (algo, input) = parse_input(&s1);
        assert_eq!("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#", algo);
        assert_eq!(vec![(0,0),(0,3),(1,0),(2,0),(2,1),(2,4),(3,2),(4,2),(4,3),(4,4)], input);
        let point_index : HashSet<(i64, i64)> = HashSet::from_iter(vec![(1,0),(2,1)]);
        assert_eq!(34, get_index(1, 1, &point_index, 0, 2, 0, 2, 0));
        assert_eq!(35, part1(algo, &input));
    }
}