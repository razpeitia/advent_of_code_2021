use std::fs;
use std::io;
use std::time::Instant;

fn part1(numbers : &Vec<i32>) -> i32 { 
    let median = numbers[numbers.len() / 2];
    numbers.iter().map(|x| (x - median).abs() ).sum()
}

fn part2(numbers : &Vec<i32>) -> i32 { 
    let mean : f64 = (numbers.iter().sum::<i32>() as f64) / (numbers.len() as f64);
    let ceil = mean.ceil() as i32;
    let floor = mean.floor() as i32;
    let sum_ceil = numbers.iter().map(|x| { let n = (x - ceil).abs(); (n + 1) * n / 2 } ).sum();
    let sum_floor = numbers.iter().map(|x| { let n = (x - floor).abs(); (n + 1) * n / 2 } ).sum();
    if sum_ceil < sum_floor { sum_ceil } else { sum_floor }
}

fn main() -> Result<(), io::Error>  {
    let f = fs::read_to_string("assets/input.txt")?;
    let mut numbers : Vec<i32> = f.trim().split(",").map(|x| x.parse().unwrap()).collect();
    numbers.sort();

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