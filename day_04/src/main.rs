use std::fs;
use std::io;
use std::time::Instant;

type Board = Vec<Vec<u32>>;

// fn print_board(board : &Board) {
//     for row in board {
//         for cell in row {
//             print!("{:2} ", cell);
//         }
//         println!("");
//     }
// }

fn won(board : &Board) -> bool {
    for row in board {
        if row.iter().all(|&v| v == 1) {
            return true;
        }
    }

    for i in 0..5  {
        let mut sum = 0;
        for j in 0..5 {
            sum += board[j][i];
        }
        if sum == 5 {
            return true;
        }
    }
    return false;
}

fn sum_remaining(count_board : &Board, board : &Board) -> u32 {
    let mut sum = 0;
    for i in 0..5 {
        for j in 0..5 {
            if count_board[i][j] == 0 {
                sum += board[i][j];
            }
        }
    }
    sum
}

fn part1(numbers : &Vec<u32>, boards : &Vec<Board>) -> u32 {
    let mut count_boards : Vec<Board> = vec![vec![vec![0; 5]; 5]; boards.len()];
    for number in numbers {
        // Mark on count boards
        for (b, board) in boards.iter().enumerate() {
            for (i, row) in board.iter().enumerate() {
                for (j, cell) in row.iter().enumerate() {
                    if number == cell {
                        count_boards[b][i][j] = 1;
                    }
                }
            }
        }
        // Check if some board won
        for (b, board) in count_boards.iter().enumerate() {
            if won(&board) {
                return *number * sum_remaining(&board, &boards[b])
            }
        }
    }
    unreachable!();
}

fn part2(numbers : &Vec<u32>, boards : &Vec<Board>) -> u32 {
    let mut count_boards : Vec<Board> = vec![vec![vec![0; 5]; 5]; boards.len()];
    let mut won_boards : Vec<u32> = vec![0; boards.len()];
    for number in numbers {
        // Mark on count boards
        for (b, board) in boards.iter().enumerate() {
            for (i, row) in board.iter().enumerate() {
                for (j, cell) in row.iter().enumerate() {
                    if number == cell {
                        count_boards[b][i][j] = 1;
                    }
                }
            }
        }
        // Check if some board won
        for (b, board) in count_boards.iter().enumerate() {
            if won(&board) {
                won_boards[b] = 1;
            }
            if won_boards.iter().all(|&n| n == 1) {
                return number * sum_remaining(&board, &boards[b]);
            }
        }
    }
    unreachable!();
}

fn main() -> Result<(), io::Error>  {
    let f = fs::read_to_string("assets/input.txt")?;
    let lines : Vec<&str> = f.split("\n\n").collect();
    let numbers : Vec<u32> = lines[0].split(",").map(|line| line.parse().unwrap()).collect();
    let boards : Vec<Board> = lines[1..].iter().map(|board| {
        board.trim().split("\n").map(|row| {
            row.trim().split_whitespace().map(|cell| cell.parse().unwrap() ).collect::<Vec<u32>>()
        }).collect::<Board>()
    }).collect::<Vec<Board>>();

    let now = Instant::now();
    println!("{:#?}", part1(&numbers, &boards));
    println!("Part 1 took {:?}", now.elapsed());

    let now = Instant::now();
    println!("{:#?}", part2(&numbers, &boards));
    println!("Part 2 took {:?}", now.elapsed());
    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::part1;
    use crate::part2;
    use crate::Board;

    #[test]
    fn test_parts() {
        let numbers : Vec<u32> = vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1];
        let boards : Vec<Board> = vec![
            vec![
                vec![22, 13, 17, 11,  0],
                vec![ 8,  2, 23,  4, 24],
                vec![21,  9, 14, 16,  7],
                vec![ 6, 10,  3, 18,  5],
                vec![ 1, 12, 20, 15, 19],
            ],

            vec![
                vec![ 3, 15,  0,  2, 22],
                vec![ 9, 18, 13, 17,  5],
                vec![19,  8,  7, 25, 23],
                vec![20, 11, 10, 24,  4],
                vec![14, 21, 16, 12,  6],
            ],

            vec![
                vec![14, 21, 17, 24,  4],
                vec![10, 16, 15,  9, 19],
                vec![18,  8, 23, 26, 20],
                vec![22, 11, 13,  6,  5],
                vec![ 2,  0, 12,  3,  7],
            ],
        ];

        assert_eq!(24 * 188, part1(&numbers, &boards));
        assert_eq!(13 * 148, part2(&numbers, &boards));
    }
}