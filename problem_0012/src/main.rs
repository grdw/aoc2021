use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input")
                   .unwrap_or("".to_string());

    let edges: Vec<&str> = input.split_terminator("\n").collect();
    let cave_system = CaveSystem::from_vec(&edges);
    println!(
        "There are {:?} routes",
        cave_system.count_paths("start")
    );

    println!(
        "There are {:?} routes, if I can visit the first small cave twice",
        cave_system.double_count_paths("start")
    );
}

struct CaveSystem<'a> {
    map: HashMap<&'a str, Vec<&'a str>>
}

fn big_cave(cave: &str) -> bool {
    cave.chars().all(|c| c.is_uppercase())
}

impl CaveSystem<'_> {
    fn from_vec<'a>(input: &'a Vec<&str>) -> CaveSystem<'a> {
        let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
        for conn in input {
            let mut nodes: Vec<&str> = conn.split("-").collect();

            if nodes[1] == "start" {
                nodes.swap(1, 0);
            }

            if nodes[0] == "end" {
                nodes.swap(0, 1);
            }

            match map.get_mut(nodes[0]) {
                Some(n) => { n.push(nodes[1]); },
                None => {
                    map.insert(nodes[0], vec![nodes[1]]);
                }
            }

            if nodes[1] != "end" && nodes[0] != "start" {
                match map.get_mut(nodes[1]) {
                    Some(n) => { n.push(nodes[0]); },
                    None => {
                        map.insert(nodes[1], vec![nodes[0]]);
                    }
                }
            }
        }

        CaveSystem { map: map }
    }

    fn count_paths(&self, start: &str) -> usize {
        let mut to_visit = vec![];
        let mut routes = 0;

        to_visit.push(vec![start]);

        while let Some(route) = to_visit.pop() {
            let current = route[route.len() - 1];

            if let Some(neighbors) = self.map.get(current) {
                for neighbor in neighbors {
                    if big_cave(neighbor) || !route.contains(&neighbor) {
                        let mut new_route = route.clone();
                        new_route.push(*neighbor);
                        to_visit.push(new_route.clone());
                    }

                    if *neighbor == "end" {
                        routes += 1;
                    }
                }
            }
        }

        routes
    }

    fn double_visit(&self, route: &Vec<&str>, neighbor: &str) -> bool {
        let mut counts = HashMap::new();

        for c in route {
            if big_cave(c) || c == &"start" {
                continue
            }

            let counter = counts.entry(c).or_insert(0);
            *counter += 1;
        }

        let counter = counts.entry(&neighbor).or_insert(0);
        *counter += 1;

        let two_counts = counts.values().filter(|&&n| n >= 2).count();
        two_counts < 2 && route.iter().filter(|&&n| n == neighbor).count() < 2
    }

    fn double_count_paths(&self, start: &str) -> usize {
        let mut to_visit = vec![];
        let mut routes = 0;

        to_visit.push(vec![start]);

        while let Some(route) = to_visit.pop() {
            let current = route[route.len() - 1];

            if let Some(neighbors) = self.map.get(current) {
                for neighbor in neighbors {
                    if big_cave(neighbor) || self.double_visit(&route, neighbor) {
                        let mut new_route = route.clone();
                        new_route.push(*neighbor);
                        to_visit.push(new_route.clone());
                    }

                    if *neighbor == "end" {
                        routes += 1;
                    }
                }
            }
        }

        routes
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
    assert_eq!(system.map.get("A"), Some(&vec!["c", "b", "end"]));
    assert_eq!(system.map.get("c"), Some(&vec!["A"]));
    assert_eq!(system.map.get("b"), Some(&vec!["A", "d", "end"]));
    assert_eq!(system.map.get("end"), None);
    assert_eq!(system.count_paths("start"), 10);
    assert_eq!(system.double_count_paths("start"), 36);
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
    assert_eq!(system.map.get("start"), Some(&vec!["HN", "kj", "dc"]));
    assert_eq!(system.map.get("dc"), Some(&vec!["end", "HN", "LN", "kj"]));
    assert_eq!(system.map.get("end"), None);
    assert_eq!(system.count_paths("start"), 19);
}

#[test]
fn test_passage_pathing_more_complex_example() {
    let complex_example = vec![
        "fs-end",
        "he-DX",
        "fs-he",
        "start-DX",
        "pj-DX",
        "end-zg",
        "zg-sl",
        "zg-pj",
        "pj-he",
        "RW-he",
        "fs-DX",
        "pj-RW",
        "zg-RW",
        "start-pj",
        "he-WI",
        "zg-he",
        "pj-fs",
        "start-RW"
    ];

    let system = CaveSystem::from_vec(&complex_example);
    assert_eq!(system.count_paths("start"), 226);
}
