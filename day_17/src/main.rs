use std::collections::VecDeque;
use std::fs;
use std::io;
use std::ops::RangeInclusive;
use std::time::Instant;

type Target = (RangeInclusive<isize>, RangeInclusive<isize>);

fn parse_input(s : &String) -> Target {
    let arr : Vec<&str> = s.trim().strip_prefix("target area: ").split(", ").collect();
    let arrx : Vec<isize> = arr[0].strip_prefix("x=").split("..").map(|x| x.parse().unwrap()).collect();
    let arry : Vec<isize> = arr[1].strip_prefix("y=").split("..").map(|x| x.parse().unwrap()).collect();
    (arrx[0]..=arrx[1], arry[0]..=arry[1])
}

fn part1(target : &Target) -> isize {
    let &(tx, ty) = target;
    let mut vy : isize = 1;
    let mut vx : isize = 0;

    // Best y that falls in the target
    let mut max_py = 0;
    
    0
}


fn main() -> Result<(), io::Error>  {
    let s = fs::read_to_string("assets/input.txt")?;
    let target  = parse_input(&s);

    let now = Instant::now();
    println!("{:#?}", part1(&target));
    println!("Part 1 took {:?}", now.elapsed());

    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::{parse_input, part1};

    #[test]
    fn test_parts() {
        let s1 = "target area: x=20..30, y=-10..-5".to_string();
        let target = parse_input(&s1);
        assert_eq!(45, part1(&target));
    }
}