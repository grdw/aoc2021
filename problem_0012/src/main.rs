use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;

fn main() {
    let input = fs::read_to_string("input")
                   .unwrap_or("".to_string());

    let edges: Vec<&str> = input.split_terminator("\n").collect();
    println!("{:?}", edges);
}

struct CaveSystem<'a> {
    map: HashMap<&'a str, Vec<&'a str>>
}

impl CaveSystem<'_> {
    fn from_vec<'a>(input: &'a Vec<&str>) -> CaveSystem<'a> {
        let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
        for conn in input {
            let nodes: Vec<&str> = conn.split("-").collect();

            match map.get_mut(nodes[0]) {
                Some(n) => { n.push(nodes[1]); },
                None => { map.insert(nodes[0], vec![nodes[1]]); }
            }

            if nodes[0].chars().all(|n| n.is_uppercase()) {
                match map.get_mut(nodes[1]) {
                    Some(n) => { n.push(nodes[0]); },
                    None => {
                        map.insert(nodes[1], vec![nodes[0]]);
                    }
                }
            }
        }

        println!("{:?}", map);

        CaveSystem { map: map }
    }

    fn count_paths(&self, start: &str) -> u32 {
        //let mut count = 0;
        let mut visited = HashSet::new();
        let mut to_visit = BinaryHeap::new();

        to_visit.push(start);

        while let Some(n) = to_visit.pop() {
            if n.chars().all(|t| t.is_lowercase()) {
                if !visited.insert(n) {
                    continue;
                }
            }

            //count += 1;
            println!("{}", n);

            if let Some(neighbors) = self.map.get(n) {
                for neighbor in neighbors {
                    println!("- {}", neighbor);
                    to_visit.push(*neighbor);
                }
            }
        }

        0
    }
}

#[test]
fn test_passage_pathing_example() {
    let example = vec![
        "start-A",
        "start-b",
        "A-c",
        "A-b",
        "b-d",
        "A-end",
        "b-end"
    ];

    let system = CaveSystem::from_vec(&example);
    assert_eq!(system.count_paths("start"), 10);
}

#[test]
fn test_passage_pathing_complex_example() {
    let other_example = vec![
        "dc-end",
        "HN-start",
        "start-kj",
        "dc-start",
        "dc-HN",
        "LN-dc",
        "HN-end",
        "kj-sa",
        "kj-HN",
        "kj-dc"
    ];

    let system = CaveSystem::from_vec(&other_example);
    assert_eq!(system.count_paths("start"), 19);
}
