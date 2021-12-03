use std::fs;
use std::io;
use std::time::Instant;


fn part1(instructions: &Vec<&str>) -> i32 {
    let mut v : Vec<i32> = vec![0; instructions[0].len()];
    for instruction in instructions {
        for (i, c) in instruction.chars().enumerate() {
            if c == '1' {
                v[i] += 1;
            }
        }
    }
    let most : i32 = (instructions.len() / 2) as i32;
    let mut e = 0;
    let mut g = 0;
    for x in v {
        if x >= most {
            e |= 1;
        } else {
            g |= 1;
        }
        e <<= 1;
        g <<= 1;
    }
    g >>= 1;
    e >>= 1;
    e * g
}

fn part2_1(instructions: &Vec<&str>, is_most : bool) -> i32 {
    let mut v : Vec<&str> = instructions.to_vec();
    let m = instructions[0].len();
    let mut i = 0;
    while i < m {
        if v.len() == 1 {
            break;
        }
        let count = v.iter().filter(|x| { x.chars().nth(i).unwrap() == '1' }).count();
        let cmp = if is_most {
            count >= (v.len() - count)
        } else {
            count < (v.len() - count)
        };
        let keep =  if cmp { '1' } else { '0' };
        v.retain(|x| x.chars().nth(i).unwrap() == keep );
        i += 1;
    }
    i32::from_str_radix(v[0], 2).unwrap()
}

fn part2(instructions: &Vec<&str>) -> i32 {
    part2_1(instructions, true) * part2_1(instructions, false)
}

fn main() -> Result<(), io::Error> {
    let f = fs::read_to_string("assets/input.txt")?;
    let instructions : Vec<&str> = f.lines().collect();

    let now = Instant::now();
    println!("{:#?}", part1(&instructions));
    println!("Part 1 took {:?}", now.elapsed());

    let now = Instant::now();
    println!("{:#?}", part2(&instructions));
    println!("Part 2 took {:?}", now.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn test_parts() {
        let instructions : Vec<&str> = vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010",
        ];
        assert_eq!(198, part1(&instructions));
        assert_eq!(230, part2(&instructions));
    }
}