// Me: Are you really going to just use the implementation from the docs?
// https://doc.rust-lang.org/std/collections/binary_heap/index.html
// Also me: Just watch me

use std::fs;
use std::io;
use std::time::Instant;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Edge {
    node: usize,
    cost: usize,
}

fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal { return Some(cost); }

        // Important as we may have already found a better way
        if cost > dist[position] { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State { cost: cost + edge.cost, position: edge.node };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

static DIRECTIONS: &'static [(isize, isize)] = &[
              (1,  0),
    (0,  -1),          (0,  1),
              (-1, 0),
];


fn parse_grid(grid : &Vec<Vec<usize>>) -> Vec<Vec<Edge>> {
    let n = grid.len();
    let mut graph : Vec<Vec<Edge>> = Vec::new();
    for i in 0..n {
        for j in 0..n {
            let mut edges : Vec<Edge> = Vec::new();
            for &(di, dj) in DIRECTIONS {
                let ni = (i as isize) + di;
                let nj = (j as isize) + dj;
                if ni >= 0 && ni < (n as isize) && nj >= 0 && nj < (n as isize) {
                    let ni = ni as usize;
                    let nj = nj as usize;
                    let node = ni * n + nj;
                    let cost = grid[ni][nj];
                    edges.push(Edge {node, cost})
                }
            }
            graph.push(edges);
        }
    }
    graph
}

fn create_grid(s : &String) -> Vec<Vec<usize>> {
    s.trim().lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect()).collect()
}

fn parse_input1(s : &String) -> (usize, Vec<Vec<Edge>>) {
    let grid = create_grid(&s);
    (grid.len(), parse_grid(&grid))
}

fn parse_input2(s : &String) -> (usize, Vec<Vec<Edge>>) {
    let grid  = create_grid(&s);
    let n = grid.len();
    let mut new_grid : Vec<Vec<usize>> = vec![vec![0; n*5]; n*5];
    for i in 0..n {
        for j in 0..n {
            new_grid[i][j] = grid[i][j];
        }
    }

    // rows
    for x in 1..5 {
        for i in 0..n {
            for j in 0..n {
                let v = new_grid[i][j+(x-1)*n];
                new_grid[i][j+x*n] = if v == 9 { 1 } else { v + 1 };
            }
        }
    }

    // cols
    for x in 1..5 {
        for i in 0..n {
            for j in 0..n {
                let v = new_grid[i+n*(x-1)][j];
                new_grid[i+n*x][j] = if v == 9 { 1 } else { v + 1 };
            }
        }
    }
    for x in 1..5 {
        for y in 1..5 {
            for i in 0..n {
                for j in 0..n {
                    let v = new_grid[i+n*(y-1)][j+x*n];
                    new_grid[i+n*y][j+n*x] = if v == 9 { 1 } else { v + 1 };
                }
            }
        }
    }
    (new_grid.len(), parse_grid(&new_grid))
}

fn part1(graph : &Vec<Vec<Edge>>, goal : usize) -> usize {
    shortest_path(&graph, 0, goal).unwrap()
}


fn main() -> Result<(), io::Error>  {
    let s = fs::read_to_string("assets/input.txt")?;
    let (n, graph) = parse_input1(&s);

    let now = Instant::now();
    println!("{:#?}", part1(&graph, (n * n) - 1));
    println!("Part 1 took {:?}", now.elapsed());

    let (n, graph) = parse_input2(&s);
    let now = Instant::now();
    println!("{:#?}", part1(&graph, (n * n) - 1));
    println!("Part 2 took {:?}", now.elapsed());

    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::{part1, parse_input1, parse_input2};

    #[test]
    fn test_parts() {
        let s = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#.to_string();
        let (n, graph) = parse_input1(&s);
        assert_eq!(40, part1(&graph, (n * n) - 1));

        let (n, graph) = parse_input2(&s);
        assert_eq!(315, part1(&graph, (n * n) - 1));
    }
}