use std::fs;
use std::io;
use std::time::Instant;

fn parse_input(s : &String) -> (usize, usize) {
    let arr : Vec<usize> = s.trim().lines().map(|x| x.split(": ").last().unwrap().parse().unwrap()).collect();
    (arr[0], arr[1])
}

fn part1(p1 : usize, p2 : usize) -> usize {
    let mut rolls = 0;
    let mut score1 = 0;
    let mut score2 = 0;
    let mut p1 = p1;
    let mut p2 = p2;
    let mut n = 1;
    loop {
        rolls += 1;
        p1 = (p1 + 3*(n + 1)) % 10;
        if p1 == 0 {
            p1 = 10;
        }
        n += 3;
        score1 += p1;
        if score1 >= 1000 {
            return 3 * rolls * score2;
        }
        rolls += 1;
        p2 = (p2 + 3*(n + 1)) % 10;
        if p2 == 0 {
            p2 = 10;
        }
        n += 3;
        score2 += p2;
        if score2 >= 1000 {
            return 3 * rolls * score1;
        }
    }
}


fn main() -> Result<(), io::Error>  {
    let s = fs::read_to_string("assets/input.txt")?;
    let (p1, p2)  = parse_input(&s);

    let now = Instant::now();
    println!("{:#?}", part1(p1, p2));
    println!("Part 1 took {:?}", now.elapsed());

    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::{parse_input, part1};

    #[test]
    fn test_parts() {
        let s1 : String = r#"Player 1 starting position: 4
Player 2 starting position: 8"#.to_string();
        let (p1, p2) = parse_input(&s1);
        assert_eq!(739785, part1(p1, p2));
    }
}