use std::collections::HashMap;
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

fn next_position(p : usize, offset : usize) -> usize {
    let t = p + offset;
    let r = t % 10;
    if r != 0 { r } else { 10 }
}

fn play(cache : &mut HashMap<(usize, usize, usize, usize, usize), (usize, usize)>, p1 : usize, p2 : usize, s1 : usize, s2 : usize, turn : usize) -> (usize, usize) {
    // We already know the outcome of that universe
    if cache.contains_key(&(p1, p2, s1, s2, turn)) {
        return *cache.get(&(p1, p2, s1, s2, turn)).unwrap();
    }
    // Trivial case, some universe won
    if s1 >= 21 || s2 >= 21 {
        let win1 = (s1 >= 21) as usize;
        let win2 = (s2 >= 21) as usize;
        let v = (win1, win2);
        cache.insert((p1, p2, s1, s2, turn), v);
        return v;
    }
    // Splitting the universe
    let mut wins1 : usize = 0;
    let mut wins2 : usize = 0;
    for &i in &[1, 2, 3] {
        for &j in &[1, 2, 3] {
            for &k in &[1, 2, 3] {
                let s = i+j+k;
                if turn == 0 {
                    let p = next_position(p1, s);
                    let ans = play(cache, p, p2, s1+p, s2, 1);
                    wins1 += ans.0;
                    wins2 += ans.1;
                } else {
                    let p = next_position(p2, s);
                    let ans = play(cache, p1, p, s1, s2+p, 0);
                    wins1 += ans.0;
                    wins2 += ans.1;
                }
            }
        }
    }
    cache.insert((p1, p2, s1, s2, turn), (wins1, wins2));
    (wins1, wins2)
}

fn part2(p1 : usize, p2 : usize) -> usize {
    let mut cache : HashMap<(usize, usize, usize, usize, usize), (usize, usize)> = HashMap::new();
    let (w1, w2) = play(&mut cache, p1, p2, 0, 0, 0);
    if w1 > w2 { w1 } else { w2 }
}


fn main() -> Result<(), io::Error>  {
    let s = fs::read_to_string("assets/input.txt")?;
    let (p1, p2)  = parse_input(&s);

    let now = Instant::now();
    println!("{:#?}", part1(p1, p2));
    println!("Part 1 took {:?}", now.elapsed());

    let now = Instant::now();
    println!("{:#?}", part2(p1, p2));
    println!("Part 2 took {:?}", now.elapsed());

    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::{parse_input, part1, part2};

    #[test]
    fn test_parts() {
        let s1 : String = r#"Player 1 starting position: 4
Player 2 starting position: 8"#.to_string();
        let (p1, p2) = parse_input(&s1);
        assert_eq!(739785, part1(p1, p2));
        assert_eq!(444356092776315, part2(p1, p2));
    }
}