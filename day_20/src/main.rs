use std::fs;
use std::io;
use std::time::Instant;

type Grid = Vec<Vec<u8>>;
static DIRECTIONS: &'static [(isize, isize)] = &[
    (-1, -1), (-1, 0), (-1, 1),
    (0,  -1), (0,  0), (0,  1),
    ( 1, -1), (1,  0), (1,  1),
];

fn parse_input(s : &String) -> (&str, Grid) {
    let lines : Vec<&str> = s.trim().lines().collect();
    let input = lines[0];
    let grid : Grid = s.trim()
        .split("\n\n")
        .last().unwrap()
        .lines()
        .map(|line| {
            line.chars().map(|c|
                match c {
                    '#' => 1,
                    '.' => 0,
                    _ => unreachable!()
                }).collect()
        }).collect();
    (input, grid)
}

fn get_index(i : usize, j : usize, grid : &Grid, rest : u8) -> u16 {
    let mut index : u16 = 0;
    let n = grid.len() as isize;
    for &(di, dj) in DIRECTIONS {
        let ni = (i as isize) + di;
        let nj = (j as isize) + dj;
        index <<= 1;
        if ni >= 0 && ni < n && nj >= 0 && nj < n {
            let ni = ni as usize;
            let nj = nj as usize;
            index |= grid[ni][nj] as u16;
        } else {
            index |= rest as u16;
        }
    }
    index
}

fn print_grid(grid : &Grid) {
    for row in grid {
        for &c in row {
            if c == 0 {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
    println!();
}

fn add_padding(grid : &mut Grid, rest : u8) {
    let n = grid.len();
    // Add two rows of rest top and bottom
    grid.insert(0, vec![rest; n]);
    grid.insert(grid.len(), vec![rest; n]);

    // Add two cols
    for row in grid.iter_mut() {
        row.insert(0, rest);
        row.insert(row.len(), rest);
    }
}

fn enhance(algo : &str, grid : &mut Grid, rest : &mut u8) {

    add_padding(grid, *rest);
    add_padding(grid, *rest);
    let original_grid = grid.to_vec();
    let n = original_grid.len();

    for i in 0..n {
        for j in 0..n {
            let index= get_index(i, j, &original_grid, *rest);
            let value = (algo.chars().nth(index as usize).unwrap() == '#') as u8;
            grid[i][j] = value;
        }
    }
    if *rest == 0 && algo.chars().nth(0).unwrap() == '#' {
        *rest = 1;
    } else if *rest == 1 && algo.chars().nth(511).unwrap() == '.' {
        *rest = 0;
    }

}

fn count_lits(grid : &Grid) -> usize {
    let mut ans = 0;
    for row in grid {
        for &c in row {
            if c == 1 {
                ans += 1;
            }
        }
    }
    ans
}

fn part1(algo : &str, input : &Grid) -> usize {
    let mut state = input.to_vec();
    let mut rest = 0;
    enhance(algo, &mut state, &mut rest);
    enhance(algo, &mut state, &mut rest);
    count_lits(&state)
}

fn part2(algo : &str, input : &Grid) -> usize {
    let mut state = input.to_vec();
    let mut rest = 0;
    for _ in 0..50 {
        enhance(algo, &mut state, &mut rest);
    }
    count_lits(&state)
}


fn main() -> Result<(), io::Error>  {
    let s = fs::read_to_string("assets/input.txt")?;
    let (algo, input)  = parse_input(&s);

    let now = Instant::now();
    println!("{:#?}", part1(algo, &input));
    println!("Part 1 took {:?}", now.elapsed());

    let now = Instant::now();
    println!("{:#?}", part2(algo, &input));
    println!("Part 2 took {:?}", now.elapsed());

    Ok(())
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::iter::FromIterator;
    use crate::{parse_input, part1, part2};

    #[test]
    fn test_parts() {
        let s1 : String = r#"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"#.to_string();
        let (algo, input) = parse_input(&s1);
        assert_eq!(35, part1(algo, &input));
        assert_eq!(3351, part2(algo, &input));
    }
}