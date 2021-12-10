use std::fs;
use std::io;
use std::time::Instant;

fn get_value(c : char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0
    }
}

fn get_value2(stack : &Vec<char>) -> usize {
    let mut ans : usize = 0;
    for &c in stack {
        let v : usize = match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => unreachable!()
        };
        ans *= 5;
        ans += v;
    }
    ans
}

fn score(line : &str) -> usize {
    let mut stack : Vec<char> = Vec::new();
    for c in line.chars() {
        match c {
            c if "([{<".contains(c) => stack.push(c),
            c if ")]}>".contains(c) => {
                // This shouldn't happen
                if stack.len() == 0 { unreachable!() }
                let top = stack.pop().unwrap();
                match (top, c) {
                    ('(', ')') => continue,
                    ('[', ']') => continue,
                    ('{', '}') => continue,
                    ('<', '>') => continue,
                    _ => return get_value(c)
                }
            }
            _ => unreachable!()
        }
    }
    0
}

fn part1(lines : &Vec<&str>) -> usize {
    lines.iter().map(|&line| score(line)).sum()
}

fn score2(line : &str) -> usize {
    let mut stack : Vec<char> = Vec::new();
    for c in line.chars() {
        match c {
            c if "([{<".contains(c) => stack.push(c),
            c if ")]}>".contains(c) => {
                // This shouldn't happen
                if stack.len() == 0 { unreachable!() }
                let top = stack.pop().unwrap();
                match (top, c) {
                    ('(', ')') => continue,
                    ('[', ']') => continue,
                    ('{', '}') => continue,
                    ('<', '>') => continue,
                    _ => return 0
                }
            }
            _ => unreachable!()
        }
    }
    stack.reverse();
    return get_value2(&stack)
}

fn part2(lines : &Vec<&str>) -> usize {
    let mut scores : Vec<usize> = lines.iter().map(|&line| score2(line)).filter(|&score| score > 0).collect();
    scores.sort();
    scores[scores.len() / 2]
}

fn main() -> Result<(), io::Error>  {
    let f = fs::read_to_string("assets/input.txt")?;
    let lines : Vec<&str> = f.trim().lines().collect();

    let now = Instant::now();
    println!("{:#?}", part1(&lines));
    println!("Part 1 took {:?}", now.elapsed());

    let now = Instant::now();
    println!("{:#?}", part2(&lines));
    println!("Part 2 took {:?}", now.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_parts() {
        let lines = vec![
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]"
        ];
        assert_eq!(26397, part1(&lines));
        assert_eq!(288957, part2(&lines));
    }
}
