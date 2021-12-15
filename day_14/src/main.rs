use std::fs;
use std::io;
use std::time::Instant;
use std::collections::HashMap;
use std::collections::VecDeque;


type Instructions = HashMap<(char, char), char>;

fn parse_input(s : &String) -> (&str, Instructions) {
    let arr : Vec<&str> = s.trim().split("\n\n").collect();
    let instructions : HashMap<(char, char), char> = arr[1].lines().map(|line| ((line.chars().nth(0).unwrap(), line.chars().nth(1).unwrap()), line.chars().nth(6).unwrap())).collect();
    (arr[0], instructions)
}

fn part1(template : &str, instructions : &Instructions, steps : usize) -> usize {
    let mut buffer : VecDeque<char> = template.chars().collect();
    for _step in 0..steps {
        let mut new_buffer : VecDeque<char> = VecDeque::new();
        for &c in buffer.iter() {
            if new_buffer.len() < 1 {
                new_buffer.push_back(c);
            }
            else {
                let a = new_buffer.pop_back().unwrap();
                let k = *instructions.get(&(a, c)).unwrap();
                new_buffer.push_back(a);
                new_buffer.push_back(k);
                new_buffer.push_back(c);
            }
        }
        buffer = new_buffer;
    }
    let mut counter : HashMap<char, usize> = HashMap::new();
    for x in buffer {
        *counter.entry(x).or_default() += 1;
    }

    counter.values().max().unwrap() - counter.values().min().unwrap()
}


fn main() -> Result<(), io::Error>  {
    let s = fs::read_to_string("assets/input.txt")?;
    let (template, instructions) = parse_input(&s);

    let now = Instant::now();
    println!("{:#?}", part1(&template, &instructions, 10));
    println!("Part 1 took {:?}", now.elapsed());

    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::{part1, parse_input};

    #[test]
    fn test_parts() {
        let input1 = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
"#.to_string();
        let (template, instructions) = parse_input(&input1);
        assert_eq!(1588, part1(&template, &instructions, 10));
    }
}