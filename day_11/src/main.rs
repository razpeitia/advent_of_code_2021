use std::fs;
use std::io;
use std::time::Instant;
use std::collections::HashSet;

type Grid = Vec<Vec<usize>>;

static DIRECTIONS: &'static [(isize, isize)] = &[
    ( 1, -1), (1,  0), (1,  1),
    (0,  -1),          (0,  1),
    (-1, -1), (-1, 0), (-1, 1),
];

fn flash(grid : &mut Grid, i : usize, j : usize) {
    for &(di, dj) in DIRECTIONS {
        let ni = (i as isize) + di;
        let nj = (j as isize) + dj;
        if ni >= 0 && ni < 10 && nj >= 0 && nj < 10 {
            let ni = ni as usize;
            let nj = nj as usize;
            if grid[ni][nj] == 0 { continue; }
            grid[ni][nj] += 1;
        }
    }
}


fn should_stop(grid : &Grid) -> bool {
    for i in 0..10 {
        for j in 0..10 {
            if grid[i][j] > 9 {
                return false;
            }
        }
    }
    true
}

fn step(grid : &mut Grid) -> usize {
    let mut flashes = 0;

    let mut flashed : HashSet<(usize, usize)> = HashSet::new();
    for i in 0..10 {
        for j in 0..10 {
            grid[i][j] += 1;
        }
    }

    loop {
        for i in 0..10 {
            for j in 0..10 {
                if grid[i][j] > 9 && !flashed.contains(&(i, j)) {
                    grid[i][j] = 0;
                    flashes += 1;
                    flashed.insert((i, j));
                    flash(grid, i, j);
                }
            }
        }

        if should_stop(grid) { break }
    }

    flashes
}

fn part1(grid : &Grid, steps : usize) -> usize {
    let mut new_grid : Grid = grid.to_vec();
    let mut ans : usize = 0;
    for _ in 0..steps {
        ans += step(&mut new_grid);
    }
    ans
}

fn all_sync(grid : &Grid) -> bool {
    for i in 0..10 {
        for j in 0..10 {
            if grid[i][j] != 0 {
                return false;
            }
        }
    }
    true
}

fn part2(grid : &Grid) -> usize {
    let mut new_grid : Grid = grid.to_vec();
    let mut steps : usize = 0;
    loop {
        step(&mut new_grid);
        steps += 1;
        if all_sync(&new_grid) {
            return steps;
        }
    }
}

fn parse_input(s : &String) -> Grid {
    s.trim()
     .lines()
     .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect())
     .collect()
}

fn main() -> Result<(), io::Error>  {
    let s = fs::read_to_string("assets/input.txt")?;
    let grid : Grid = parse_input(&s);

    let now = Instant::now();
    println!("{:#?}", part1(&grid, 100));
    println!("Part 1 took {:?}", now.elapsed());

    let now = Instant::now();
    println!("{:#?}", part2(&grid));
    println!("Part 2 took {:?}", now.elapsed());

    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::{part2, part1, parse_input};

    #[test]
    fn test_parts() {
        let lines : Vec<&str> = vec![
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526"
        ];
        let input : String = lines.join("\n");
        let grid = parse_input(&input);
        assert_eq!(204, part1(&grid, 10));
        assert_eq!(1656, part1(&grid, 100));
        assert_eq!(195, part2(&grid));
    }
}
