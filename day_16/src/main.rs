use std::fs;
use std::io;
use std::time::Instant;

fn parse_input(s : &String) -> Vec<u8> {
    s.trim()
     .chars()
     .collect::<Vec<char>>()
     .chunks(2)
     .map(|c| u8::from_str_radix(&c.iter().collect::<String>(), 16).unwrap())
     .collect()
}

fn get_value(pointer : &mut usize, len : usize, stream : &Vec<u8>) -> usize {
    let mut value : usize = 0;
    for p in *pointer..(*pointer+len) {
        value <<= 1;
        let index = p / 8;
        if index < stream.len() {
            let index_value = stream[index];
            let offset = 7 - (p % 8);
            let offset_value = index_value >> offset;
            let bit = (offset_value & 1) as usize;
            value |= bit;
        }
    }
    *pointer += len;
    value
}

fn sum_version(stream : &Vec<u8>, pointer : &mut usize, sum : &mut usize) {
    let version = get_value(pointer, 3, stream);
    let r#type = get_value(pointer, 3, stream);
    *sum += version;
    if r#type == 4 {
        loop {
            let a = get_value(pointer, 1, stream);
            get_value(pointer, 4, stream);
            if a == 0 {
                break;
            }
        }
        return
    } else {
        let len_type = get_value(pointer, 1, stream);
        if len_type == 0 {
            let fixed_len = get_value(pointer, 15, stream);
            let sub_limit = *pointer + fixed_len;
            while *pointer < sub_limit {
                sum_version(stream, pointer, sum);
            }
        } else {
            let num_packets = get_value(pointer, 11, stream);
            for _ in 0..num_packets {
                sum_version(stream, pointer, sum)
            }
        }
    }
}

fn part1(stream : &Vec<u8>) -> usize {
    let mut sum : usize = 0;
    let mut pointer : usize = 0;
    sum_version(stream, &mut pointer, &mut sum);
    sum
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
    use crate::{part1, parse_input};

    #[test]
    fn test_parts() {
        let s1 = "D2FE28".to_string();
        let input1 : Vec<u8> = parse_input(&s1);
        assert_eq!(vec![210, 254, 40], input1);
        assert_eq!(6, part1(&input1));

        let s1 = "38006F45291200".to_string();
        let input1 : Vec<u8> = parse_input(&s1);
        assert_eq!(9, part1(&input1));

        let s1 = "EE00D40C823060".to_string();
        let input1 : Vec<u8> = parse_input(&s1);
        assert_eq!(14, part1(&input1));

        let s1 = "8A004A801A8002F478".to_string();
        let input1 : Vec<u8> = parse_input(&s1);
        assert_eq!(16, part1(&input1));

        let s1 = "620080001611562C8802118E34".to_string();
        let input1 : Vec<u8> = parse_input(&s1);
        assert_eq!(12, part1(&input1));

        let s1 = "C0015000016115A2E0802F182340".to_string();
        let input1 : Vec<u8> = parse_input(&s1);
        assert_eq!(23, part1(&input1));

        let s1 = "A0016C880162017C3686B18A3D4780".to_string();
        let input1 : Vec<u8> = parse_input(&s1);
        assert_eq!(31, part1(&input1));
    }
}