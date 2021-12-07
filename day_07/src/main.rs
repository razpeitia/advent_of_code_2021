use std::fs;
use std::io;
use std::time::Instant;
use std::collections::HashMap;

fn part1(numbers : &Vec<usize>) -> usize { 
    let mut solutions : HashMap<usize, usize> = HashMap::new();
    for &number in numbers {
        if !solutions.contains_key(&number) {
            let solution = numbers.iter().map(|&x| if x > number { x - number } else { number - x } ).sum();
            solutions.insert(number, solution);
        }
    }
    *solutions.iter().min_by(|a, b| a.1.cmp(&b.1)).map(|(_k, v)| v).unwrap()
}

fn part2(numbers : &Vec<usize>) -> usize { 
    let max : usize = *numbers.iter().max().unwrap();
    let mut solutions : HashMap<usize, usize> = HashMap::new();
    for number in 0..=max {
        if !solutions.contains_key(&number) {
            let solution : usize = numbers.iter().map(|&x| { 
                let s = if x > number { x - number } else { number - x };
                (s * (s + 1)) / 2
            } ).sum();
            solutions.insert(number, solution);
        }
    }
    *solutions.iter().min_by(|a, b| a.1.cmp(&b.1)).map(|(_k, v)| v).unwrap()
}

fn main() -> Result<(), io::Error>  {
    let f = fs::read_to_string("assets/input.txt")?;
    let numbers : Vec<usize> = f.trim().split(",").map(|x| x.parse().unwrap()).collect();

    let now = Instant::now();
    println!("{}", part1(&numbers));
    println!("Part 1 took {:?}", now.elapsed());

    let now = Instant::now();
    println!("{}", part2(&numbers));
    println!("Part 2 took {:?}", now.elapsed());

    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn test_parts() {
        let numbers = vec![16,1,2,0,4,2,7,1,2,14];
        assert_eq!(37, part1(&numbers));
        assert_eq!(168, part2(&numbers));
    }
}