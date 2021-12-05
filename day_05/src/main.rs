use std::fs;
use std::io;
use std::time::Instant;

enum SegmentType {
    Horizontal,
    Vertical,
    Diagonal,
}

#[derive(Debug)]
struct Segment {
    x1 : u32,
    y1 : u32,
    x2 : u32,
    y2 : u32,
}

impl Segment {
    fn get_type(&self) -> SegmentType {
        if self.x1 == self.x2 {
            return SegmentType::Vertical;
        }
        else if self.y1 == self.y2 {
            return SegmentType::Horizontal;
        } else {
            return SegmentType::Diagonal;
        }
    }
}

type Board = Vec<Vec<u32>>;

fn board_count(board : &Board) -> u32 {
    let mut sum = 0;
    for row in board {
        for cell in row {
            if *cell >= 2 {
                sum += 1;
            }
        }
    }
    sum
}

fn part1(segments : &Vec<Segment>) -> u32 {
    let max_x : u32 = segments.iter().map(|s| if s.x1 > s.x2 { s.x1 } else { s.x2 } ).max().unwrap() + 1;
    let max_y : u32 = segments.iter().map(|s| if s.y1 > s.y2 { s.y1 } else { s.y2 } ).max().unwrap() + 1;
    let mut board : Board = vec![vec![0u32; max_y as usize]; max_x as usize];
    for segment in segments {
        match segment.get_type() {
            SegmentType::Vertical => {
                let x = segment.x1;
                let (y1, y2) = if segment.y1 < segment.y2 { (segment.y1, segment.y2) } else { (segment.y2, segment.y1) };
                for i in y1..=y2 {
                    board[i as usize][x as usize] += 1;
                }
            },
            SegmentType::Horizontal => {
                let y = segment.y1;
                let (x1, x2) = if segment.x1 < segment.x2 { (segment.x1, segment.x2) } else { (segment.x2, segment.x1) };
                for i in x1..=x2 {
                    board[y as usize][i as usize] += 1;
                }
            },
            _ => {},
        }
    }
    board_count(&board)
}

fn part2(segments : &Vec<Segment>) -> u32 {
    let max_x : u32 = segments.iter().map(|s| if s.x1 > s.x2 { s.x1 } else { s.x2 } ).max().unwrap() + 1;
    let max_y : u32 = segments.iter().map(|s| if s.y1 > s.y2 { s.y1 } else { s.y2 } ).max().unwrap() + 1;
    let mut board : Board = vec![vec![0u32; max_y as usize]; max_x as usize];
    for segment in segments {
        match segment.get_type() {
            SegmentType::Vertical => {
                let x = segment.x1;
                let (y1, y2) = if segment.y1 < segment.y2 { (segment.y1, segment.y2) } else { (segment.y2, segment.y1) };
                for i in y1..=y2 {
                    board[i as usize][x as usize] += 1;
                }
            },
            SegmentType::Horizontal => {
                let y = segment.y1;
                let (x1, x2) = if segment.x1 < segment.x2 { (segment.x1, segment.x2) } else { (segment.x2, segment.x1) };
                for i in x1..=x2 {
                    board[y as usize][i as usize] += 1;
                }
            },
            SegmentType::Diagonal => {
                let mut x = segment.x1;
                let mut y = segment.y1;
                while x != segment.x2 && y != segment.y2 {
                    board[y as usize][x as usize] += 1;
                    if x < segment.x2 {
                        x += 1;
                    } else {
                        x -= 1;
                    }
                    if y < segment.y2 {
                        y += 1;
                    } else {
                        y -= 1;
                    }
                }
                board[y as usize][x as usize] += 1;
            },
        }
    }
    board_count(&board)
}

fn main() -> Result<(), io::Error>  {
    let f = fs::read_to_string("assets/input.txt")?;
    let segments : Vec<Segment> = f.lines().map(|line| {
        let points : Vec<&str> = line.split(" -> ").collect();
        let p1 : Vec<&str> = points[0].split(",").collect();
        let p2 : Vec<&str> = points[1].split(",").collect();
        let x1 : u32 = p1[0].parse().unwrap();
        let y1 : u32 = p1[1].parse().unwrap();
        let x2 : u32 = p2[0].parse().unwrap();
        let y2 : u32 = p2[1].parse().unwrap();
        return Segment{x1, y1, x2, y2};
    }).collect();
    
    let now = Instant::now();
    println!("{:#?}", part1(&segments));
    println!("Part 1 took {:?}", now.elapsed());

    let now = Instant::now();
    println!("{:#?}", part2(&segments));
    println!("Part 2 took {:?}", now.elapsed());
    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;
    use crate::Segment;

    #[test]
    fn test_parts() {
        let segments : Vec<Segment> = vec![
            Segment{x1:0, y1:9, x2:5, y2:9},
            Segment{x1:8, y1:0, x2:0, y2:8},
            Segment{x1:9, y1:4, x2:3, y2:4},
            Segment{x1:2, y1:2, x2:2, y2:1},
            Segment{x1:7, y1:0, x2:7, y2:4},
            Segment{x1:6, y1:4, x2:2, y2:0},
            Segment{x1:0, y1:9, x2:2, y2:9},
            Segment{x1:3, y1:4, x2:1, y2:4},
            Segment{x1:0, y1:0, x2:8, y2:8},
            Segment{x1:5, y1:5, x2:8, y2:2},
        ];
        assert_eq!(5, part1(&segments));
        assert_eq!(12, part2(&segments));

        let segments : Vec<Segment> = vec![
            Segment{x1:0, y1:0, x2:2, y2:2},
        ];
        assert_eq!(0, part2(&segments));
    }
}
