use std::fs;
use std::io;
use std::time::Instant;

fn part1(numbers : &Vec<u64>) -> u64 {
    let n = numbers.len();
    let mut ans = 0;
    for i in 1..n {
        if numbers[i - 1] < numbers[i] {
            ans += 1;
        }
    }
    ans
}

fn part2(numbers : &Vec<u64>) -> u64 {
    let n = numbers.len();
    let mut sum = numbers[0] + numbers[1] + numbers[2];
    let mut prev_sum = sum;
    let mut ans = 0;
    for i in 3..n {
        sum = sum - numbers[i - 3];
        sum = sum + numbers[i];
        if prev_sum < sum {
            ans += 1;
        }
        prev_sum = sum;
    }
    ans
}

fn main() -> Result<(), io::Error> {
    let numbers : Vec<u64> = fs::read_to_string("assets/input.txt")?
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    
    let now = Instant::now();
    let ans1 = part1(&numbers);
    println!("{}", ans1);
    println!("Part 1 took {:?}", now.elapsed());
    
    let now = Instant::now();
    let ans2 = part2(&numbers);
    println!("{}", ans2);
    println!("Part 2 took {:?}", now.elapsed());
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;

    #[test]
    fn test_part1() {
        let numbers = vec![199,200, 208,210,200,207,240,269,260,263];
        assert_eq!(7, part1(&numbers));
    }

    #[test]
    fn test_part2() {
        let numbers = vec![199,200, 208,210,200,207,240,269,260,263];
        assert_eq!(5, part2(&numbers));
    }
}