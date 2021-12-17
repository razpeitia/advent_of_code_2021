use std::collections::VecDeque;
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

fn to_integer(bytes : &Vec<usize>) -> usize {
    let mut ans : usize = 0;
    for &b in bytes {
        ans <<= 4;
        ans |= b;
    }
    ans
}

fn evaluate2(r#type : usize, len : usize, stack : &mut VecDeque<usize>) {
    match r#type {
        0 => {
            let mut sum : usize = 0;
            for _ in 0..len {
                sum += stack.pop_back().unwrap();
            }
            stack.push_back(sum);
        }
        1 => {
            let mut mul : usize = 1;
            for _ in 0..len {
                mul *= stack.pop_back().unwrap();
            }
            stack.push_back(mul);
        }
        2 => {
            let mut min = stack.pop_back().unwrap();
            for _ in 1..len {
                let x = stack.pop_back().unwrap();
                if x < min {
                    min = x;
                }
            }
            stack.push_back(min);
        }
        3 => {
            let mut max = stack.pop_back().unwrap();
            for _ in 1..len {
                let x = stack.pop_back().unwrap();
                if x > max {
                    max = x;
                }
            }
            stack.push_back(max);
        }
        4 => return,
        5 => {
            let a = stack.pop_back().unwrap();
            let b = stack.pop_back().unwrap();
            let gt : usize = (b > a) as usize;
            stack.push_back(gt);
        }
        6 => {
            let a = stack.pop_back().unwrap();
            let b = stack.pop_back().unwrap();
            let lt : usize = (b < a) as usize;
            stack.push_back(lt);
        }
        7 => {
            let a = stack.pop_back().unwrap();
            let b = stack.pop_back().unwrap();
            let eq : usize = (b == a) as usize;
            stack.push_back(eq);
        }
        _ => unreachable!()
    }
}

fn evaluate(stream : &Vec<u8>, pointer : &mut usize, stack : &mut VecDeque<usize>) {
    let _version = get_value(pointer, 3, stream);
    let r#type = get_value(pointer, 3, stream);

    let mut packages : usize = 0;
    if r#type == 4 {
        let mut bytes: Vec<usize> = Vec::new();
        loop {
            let a = get_value(pointer, 1, stream);
            let value = get_value(pointer, 4, stream);
            bytes.push(value);
            if a == 0 {
                break;
            }
        }
        stack.push_back(to_integer(&bytes));
        return
    } else {
        let len_type = get_value(pointer, 1, stream);
        if len_type == 0 {
            let fixed_len = get_value(pointer, 15, stream);
            let sub_limit = *pointer + fixed_len;
            while *pointer < sub_limit {
                evaluate(stream, pointer, stack);
                packages += 1;
            }
        } else {
            let num_packets = get_value(pointer, 11, stream);
            packages = num_packets;
            for _ in 0..num_packets {
                evaluate(stream, pointer, stack)
            }
        }
    }
    evaluate2(r#type, packages, stack)
}

fn part2(stream : &Vec<u8>) -> usize {
    let mut pointer : usize = 0;
    let mut stack : VecDeque<usize> = VecDeque::new();
    evaluate(stream, &mut pointer, &mut stack);
    stack[0]
}


fn main() -> Result<(), io::Error>  {
    let s = fs::read_to_string("assets/input.txt")?;
    let input  = parse_input(&s);

    let now = Instant::now();
    println!("{:#?}", part1(&input));
    println!("Part 1 took {:?}", now.elapsed());

    let now = Instant::now();
    println!("{:#?}", part2(&input));
    println!("Part 2 took {:?}", now.elapsed());
    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::{parse_input, part1, part2};

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

        let s1 = "C200B40A82".to_string();
        let input1 : Vec<u8> = parse_input(&s1);
        assert_eq!(3, part2(&input1));

        let s1 = "04005AC33890".to_string();
        let input1 : Vec<u8> = parse_input(&s1);
        assert_eq!(54, part2(&input1));

        let s1 = "880086C3E88112".to_string();
        let input1 : Vec<u8> = parse_input(&s1);
        assert_eq!(7, part2(&input1));

        let s1 = "CE00C43D881120".to_string();
        let input1 : Vec<u8> = parse_input(&s1);
        assert_eq!(9, part2(&input1));

        let s1 = "D8005AC2A8F0".to_string();
        let input1 : Vec<u8> = parse_input(&s1);
        assert_eq!(1, part2(&input1));

        let s1 = "F600BC2D8F".to_string();
        let input1 : Vec<u8> = parse_input(&s1);
        assert_eq!(0, part2(&input1));

        let s1 = "9C005AC2F8F0".to_string();
        let input1 : Vec<u8> = parse_input(&s1);
        assert_eq!(0, part2(&input1));

        let s1 = "9C0141080250320F1802104A08".to_string();
        let input1 : Vec<u8> = parse_input(&s1);
        assert_eq!(1, part2(&input1));
    }
}