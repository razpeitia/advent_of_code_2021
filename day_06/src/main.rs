use std::fs;
use std::io;
use std::time::Instant;

fn part1(numbers : &Vec<usize>, days : usize) -> usize {
    let mut state1 : [usize; 9] = [0; 9];
    let mut state2 : [usize; 9] = [0; 9];
    for &number in numbers {
        state1[number] += 1;
    }

    for _d in 1..=days {
        for i in 1..9 {
            state2[i - 1] = state1[i];
        }
        let n = state1[0];
        state2[6] += n;
        state2[8] = n;
        for i in 0..9 {
            state1[i] = state2[i];
        }
    }
    state1.iter().sum()
}

fn main() -> Result<(), io::Error>  {
    let f = fs::read_to_string("assets/input.txt")?;
    let numbers : Vec<usize> = f.split(",").map(|x| x.parse().unwrap()).collect();
    
    let now = Instant::now();
    println!("{:#?}", part1(&numbers, 80));
    println!("Part 1 took {:?}", now.elapsed());

    let now = Instant::now();
    println!("{:#?}", part1(&numbers, 256));
    println!("Part 2 took {:?}", now.elapsed());

    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn test_parts() {
        let numbers = vec![3,4,3,1,2];
        assert_eq!(26, part1(&numbers, 18));
        assert_eq!(5934, part1(&numbers, 80));
    }
}
