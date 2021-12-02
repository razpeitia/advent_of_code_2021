use std::fs;
use std::io;
use std::time::Instant;

enum Instruction {
    Forward(i64),
    Down(i64),
    Up(i64),
}

fn part2(instructions: &Vec<Instruction>) -> i64 {
    let mut h : i64 = 0;
    let mut d : i64 = 0;
    let mut a : i64 = 0;

    for instruction in instructions {
        match instruction {
            Instruction::Forward(x) => {
                h += x;
                d += a * x;
            },
            Instruction::Down(x) => a += x,
            Instruction::Up(x) => a -= x,
        }
    }
    h * d
}

fn part1(instructions: &Vec<Instruction>) -> i64 {
    let mut h : i64 = 0;
    let mut d : i64 = 0;
    for instruction in instructions {
        match instruction {
            Instruction::Forward(x) => h += x,
            Instruction::Down(x) => d += x,
            Instruction::Up(x) => d -= x,
        }
    }
    h * d
}

fn main() -> Result<(), io::Error> {
    let instructions : Vec<Instruction> = fs::read_to_string("assets/input.txt")?
        .lines()
        .map(|line| {
            let split = line.split(" ");
            let vec: Vec<&str> = split.collect();
            match vec[0] {
                "forward" => Instruction::Forward(vec[1].parse().unwrap()),
                "down" => Instruction::Down(vec[1].parse().unwrap()),
                "up" => Instruction::Up(vec[1].parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .collect();

    let now = Instant::now();
    println!("{}", part1(&instructions));
    println!("Part 1 took {:?}", now.elapsed());


    let now = Instant::now();
    println!("{}", part2(&instructions));
    println!("Part 2 took {:?}", now.elapsed());
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;
    use crate::Instruction;

    #[test]
    fn test_parts() {
        let instructions = vec![
            Instruction::Forward(5),
            Instruction::Down(5),
            Instruction::Forward(8),
            Instruction::Up(3),
            Instruction::Down(8),
            Instruction::Forward(2),
        ];
        assert_eq!(150, part1(&instructions));
        assert_eq!(900, part2(&instructions));
    }
}