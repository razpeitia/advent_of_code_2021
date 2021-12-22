use std::fs;
use std::io;
use std::time::Instant;

type Cube = ((isize, isize), (isize, isize), (isize, isize));

fn parse_input(s : &String) -> Vec<(Cube, bool)> {
    s.trim().lines().map(|line| {
        let arr : Vec<&str> = line.split(" ").collect();
        let state = arr[0] == "on";
        let a : Vec<(isize, isize)> = arr[1].split(",").map(|range| {
            let v : Vec<isize> = range[2..].split("..").map(|r| r.parse().unwrap()).collect();
            (v[0], v[1])
        }).collect();
        ((a[0], a[1], a[2]), state)
    }).collect()
}

fn part1(input : &Vec<(Cube, bool)>) -> usize {
    let mut space : Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 101]; 101]; 101];
    for (cube, state) in input {
        let &((x1, x2), (y1, y2), (z1, z2)) = cube;
        if (x1 >= -50 && x1 <= 50) && (x2 >= -50 && x2 <= 50) &&
           (y1 >= -50 && y1 <= 50) && (y2 >= -50 && y2 <= 50) &&
           (z1 >= -50 && z1 <= 50) && (z2 >= -50 && z2 <= 50) {
            println!("x={}..{},y={}..{},z={}..{}", x1, x2, y1, y2, z1, z2);
            let x1 = x1 + 50;
            let x2 = x2 + 50;
            let y1 = y1 + 50;
            let y2 = y2 + 50;
            let z1 = z1 + 50;
            let z2 = z2 + 50;
            for x in x1..=x2 {
                for y in y1..=y2 {
                    for z in z1..=z2 {
                        space[x as usize][y as usize][z as usize] = *state;
                    }
                }
            }
        }
    }
    let mut ans : usize = 0;
    for x in 0..101 {
        for y in 0..101 {
            for z in 0..101 {
                if space[x][y][z] {
                    ans += 1;
                }
            }
        }
    }
    ans
}


fn main() -> Result<(), io::Error>  {
    let s = fs::read_to_string("assets/input.txt")?;
    let input  = parse_input(&s);

    let now = Instant::now();
    println!("{:#?}", part1(&input));
    println!("Part 1 took {:?}", now.elapsed());

    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::{parse_input, part1};

    #[test]
    fn test_parts() {
        let s1 : String = r#"on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10"#.to_string();
        let input = parse_input(&s1);
        assert_eq!(39, part1(&input));

        let s1 : String = r#"on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682"#.to_string();
        let input = parse_input(&s1);
        assert_eq!(590784, part1(&input));
    }
}