use std::fs;
use std::io;
use std::time::Instant;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Graph = HashMap<String, HashSet<String>>;

fn parse_input(s : &String) -> Graph {
    let mut graph : Graph = HashMap::new();
    for line in s.trim().lines() {
        let arr : Vec<&str> = line.split('-').collect();
        let key = arr[0].to_string();
        let value = arr[1].to_string();
        graph.entry(key.clone()).or_insert_with(HashSet::new).insert(value.clone());
        graph.entry(value.clone()).or_insert_with(HashSet::new).insert(key.clone());
    }
    graph
}

fn count_paths(graph : &Graph, queue : &mut VecDeque<String>, visited : &mut HashMap<String, u8>, paths : &mut usize) {
    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();
        if v == "end" {
            *paths += 1;
            return;
        }
        for n in graph.get(&v).unwrap() {
            if n == "start" { continue; }
            let is_small_cave = n.chars().all(|x| x.is_lowercase());
            if is_small_cave && visited[n] >= 1 {
                continue;
            }
            queue.push_back(n.to_string());

            let &visits = visited.get(n).unwrap();
            if is_small_cave {
                visited.insert(n.to_string(), visits + 1);
            }
            count_paths(graph, queue, visited, paths);
            if is_small_cave {
                visited.insert(n.to_string(), visits);
            }
            queue.pop_back();
        }
    }
}

fn count_paths2(graph : &Graph, queue : &mut VecDeque<String>, visited : &mut HashMap<String, u8>, paths : &mut usize) {
    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();
        if v == "end" {
            *paths += 1;
            return;
        }
        for n in graph.get(&v).unwrap() {
            if n == "start" { continue; }
            let is_small_cave = n.chars().all(|x| x.is_lowercase());
            if is_small_cave {
                if visited[n] == 1 && visited.values().any(|&x| x >= 2) {
                    continue;
                }
                if visited[n] >= 2 {
                    continue;
                }
            }


            let &visits = visited.get(n).unwrap();
            if is_small_cave {
                visited.insert(n.to_string(), visits + 1);
            }
            queue.push_back(n.to_string());
            count_paths2(graph, queue, visited, paths);
            if is_small_cave {
                visited.insert(n.to_string(), visits);
            }
            queue.pop_back();
        }
    }
}

fn part1(graph : &Graph) -> usize {
    let mut visited : HashMap<String, u8> = HashMap::new();
    let mut queue : VecDeque<String> = VecDeque::new();
    let mut paths = 0;
    queue.push_back("start".to_string());
    for k in graph.keys() {
        visited.insert(k.to_string(), 0);
    }
    visited.insert("start".to_string(), 1);
    count_paths(graph, &mut queue, &mut visited, &mut paths);
    paths
}

fn part2(graph : &Graph) -> usize {
    let mut visited : HashMap<String, u8> = HashMap::new();
    let mut queue : VecDeque<String> = VecDeque::new();
    let mut paths = 0;
    queue.push_back("start".to_string());
    for k in graph.keys() {
        visited.insert(k.to_string(), 0);
    }
    visited.insert("start".to_string(), 1);
    count_paths2(graph, &mut queue, &mut visited, &mut paths);
    paths
}

fn main() -> Result<(), io::Error>  {
    let s = fs::read_to_string("assets/input.txt")?;
    let graph : Graph = parse_input(&s);

    let now = Instant::now();
    println!("{:#?}", part1(&graph));
    println!("Part 1 took {:?}", now.elapsed());

    let now = Instant::now();
    println!("{:#?}", part2(&graph));
    println!("Part 1 took {:?}", now.elapsed());

    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::{part1, part2, parse_input};

    #[test]
    fn test_parts() {
        let input1  = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#.to_string();
        let graph1 = parse_input(&input1);
        assert_eq!(10, part1(&graph1));
        assert_eq!(36, part2(&graph1));

        let input2  = r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#.to_string();
        let graph2 = parse_input(&input2);
        assert_eq!(19, part1(&graph2));
        assert_eq!(103, part2(&graph2));


        let input3  = r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"#.to_string();
        let graph3 = parse_input(&input3);
        assert_eq!(226, part1(&graph3));
        assert_eq!(3509, part2(&graph3));
    }
}
