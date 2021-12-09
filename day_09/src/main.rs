use std::fs;
use std::io;
use std::time::Instant;

type Matrix = Vec<Vec<usize>>;

fn is_low(matrix : &Matrix, m : usize, n : usize, i : usize, j : usize) -> bool {
    let x = matrix[i][j];
    // down
    if i+1 < m && x >= matrix[i+1][j] { return false; }
    // up
    if i > 0 && x >= matrix[i-1][j] { return false; }
    // left
    if j > 0 && x >= matrix[i][j-1] { return false; }
    // right
    if j+1 < n && x >= matrix[i][j+1] { return false; }
    true
}

fn part1(matrix : &Matrix) -> usize {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut sum : usize = 0;
    for i in 0..m {
        for j in 0..n {
            if is_low(matrix, m, n, i, j) {
                sum += matrix[i][j] + 1;
            }
        }
    }
    sum
}

fn mark(matrix : &mut Matrix, m : usize, n : usize, i : usize, j : usize, size : &mut usize) {
    if i < m && j < n && matrix[i][j] != 9 {
        matrix[i][j] = 9;
        *size = *size + 1;
        mark(matrix, m, n, i + 1, j, size);
        if i > 0 {
            mark(matrix, m, n, i - 1, j, size)
        }
        mark(matrix, m, n, i, j + 1, size);
        if j > 0 {
            mark(matrix, m, n, i, j - 1, size)
        }
    }
}

fn part2(matrix : &Matrix) -> usize {
    let mut matrix2 = matrix.to_vec();
    let mut sizes : Vec<usize> = Vec::new();
    let m = matrix.len();
    let n = matrix[0].len();
    for i in 0..m {
        for j in 0..n {
            if matrix2[i][j] != 9 {
                let mut size = 0;
                mark(&mut matrix2, m, n, i, j, &mut size);
                sizes.push(size);
            }
        }
    }
    sizes.sort();
    sizes.reverse();
    sizes.iter().take(3).fold(1, |acc, &x| x * acc )
}

fn main() -> Result<(), io::Error>  {
    let f = fs::read_to_string("assets/input.txt")?;
    let matrix : Matrix = f.trim().split("\n").map(|line| line.chars().map(|x| x.to_digit(10).unwrap() as usize).collect()).collect();

    let now = Instant::now();
    println!("{:#?}", part1(&matrix));
    println!("Part 1 took {:?}", now.elapsed());

    let now = Instant::now();
    println!("{:#?}", part2(&matrix));
    println!("Part 1 took {:?}", now.elapsed());

    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::{Matrix, part1, part2};

    #[test]
    fn test_parts() {
        let matrix : Matrix = vec![
            vec![2,1,9,9,9,4,3,2,1,0],
            vec![3,9,8,7,8,9,4,9,2,1],
            vec![9,8,5,6,7,8,9,8,9,2],
            vec![8,7,6,7,8,9,6,7,8,9],
            vec![9,8,9,9,9,6,5,6,7,8],
        ];
        assert_eq!(15, part1(&matrix));
        assert_eq!(1134, part2(&matrix));
    }
}
