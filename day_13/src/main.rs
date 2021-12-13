use std::fs;
use std::io;
use std::time::Instant;

type Point = (usize, usize);
type Points = Vec<Point>;
type Instruction = (char, usize);
type Instructions = Vec<Instruction>;

fn parse_input(s : &String) -> (Points, Instructions) {
    let arr : Vec<&str> = s.trim().split("\n\n").collect();
    let points : Vec<(usize, usize)> = arr[0].lines().map(|line| {
       let p : Vec<usize> = line.split(",").map(|x| x.parse().unwrap()).collect();
        (p[0], p[1])
    }).collect();
    let instructions : Vec<(char, usize)> = arr[1].lines().map(|line| {
        let p : Vec<&str> = line.strip_prefix("fold along ").unwrap().split("=").collect();
        (p[0].chars().next().unwrap(), p[1].parse().unwrap())
    }).collect();
    (points, instructions)
}

fn fold_points(points :  &Points, &(axis, value) : &Instruction) -> Points {
    match axis {
        'x' => {
            points.iter().map(|&(x, y)| {
                if x > value {
                    (value - (x - value), y)
                } else {
                    (x, y)
                }
            }).collect()
        },
        'y' => {
            points.iter().map(|&(x, y)| {
                if y > value {
                    (x, value - (y - value))
                } else {
                    (x, y)
                }
            }).collect()
        },
        _ => unreachable!()
    }
}

fn print_points(points : &Points) {
    let rows = points.iter().map(|p| p.1).max().unwrap() + 1;
    let cols = points.iter().map(|p| p.0).max().unwrap() + 1;
    let mut arr : Vec<Vec<u8>> = vec![vec![0; cols]; rows];
    for p in points {
        arr[p.1][p.0] = 1;
    }
    for i in 0..rows {
        for j in 0..cols {
            if arr[i][j] == 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn part1(points : &Points, instructions : &Instructions) -> usize {
    let mut result = fold_points(points, &instructions[0]);
    result.sort();
    result.dedup();
    result.len()
}

fn part2(points : &Points, instructions : &Instructions) {
    let mut result : Points = points.to_vec();
    for instruction in instructions {
        result = fold_points(&result, instruction);
        result.sort();
        result.dedup();
    }
    print_points(&result);
}


fn main() -> Result<(), io::Error>  {
    let s = fs::read_to_string("assets/input.txt")?;
    let (points, instructions) = parse_input(&s);

    let now = Instant::now();
    println!("{:#?}", part1(&points, &instructions));
    println!("Part 1 took {:?}", now.elapsed());

    let now = Instant::now();
    part2(&points, &instructions);
    println!("Part 2 took {:?}", now.elapsed());

    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::{part1, parse_input};

    #[test]
    fn test_parts() {
        let input1 = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#.to_string();
        let (points, instructions) = parse_input(&input1);
        assert_eq!(17, part1(&points, &instructions));
    }
}