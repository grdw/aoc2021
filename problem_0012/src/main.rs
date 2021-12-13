use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input")
                   .unwrap_or("".to_string());

    let edges: Vec<&str> = input.split_terminator("\n").collect();
    let cave_system = CaveSystem::from_vec(&edges);
    println!(
        "There are {:?} routes",
        cave_system.count_paths()
    );

    println!(
        "There are {:?} routes, if I can visit the first small cave twice",
        cave_system.double_count_paths()
    );
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Cave<'a> {
    Start,
    End,
    Big(&'a str),
    Small(&'a str)
}

fn cave(cave_str: &str) -> Cave {
    if cave_str.chars().all(|c| c.is_uppercase()) {
        Cave::Big(cave_str)
    } else if cave_str == "start"{
        Cave::Start
    } else if cave_str == "end" {
        Cave::End
    } else {
        Cave::Small(cave_str)
    }
}

#[test]
fn cave_test() {
    assert_eq!(cave("B"), Cave::Big("B"));
    assert_eq!(cave("a"), Cave::Small("a"));
    assert_eq!(cave("start"), Cave::Start);
    assert_eq!(cave("end"), Cave::End);
}

struct CaveSystem<'a> {
    map: HashMap<Cave<'a>, Vec<Cave<'a>>>
}

fn double_visit(route: &Vec<&Cave>, neighbor: &Cave) -> bool {
    let mut counts = HashMap::new();

    for c in route {
        match c {
            Cave::Small(_) => {
                let counter = counts.entry(c).or_insert(0);
                *counter += 1;
            },
            _ => continue
        }
    }

    let counter = counts.entry(&neighbor).or_insert(0);
    *counter += 1;

    let two_counts = counts.values().filter(|&&n| n >= 2).count();
    let neighbor_count = counts.get(&neighbor).unwrap() - 1;

    two_counts < 2 && neighbor_count < 2
}

#[test]
fn test_double_visit() {
    let route = vec![&Cave::Start];
    assert!(double_visit(&route, &Cave::Small("c")));

    let route = vec![
        &Cave::Start,
        &Cave::Small("c"),
        &Cave::Small("c"),
        &Cave::Big("D"),
        &Cave::Small("b")
    ];

    assert!(!double_visit(&route, &Cave::Small("c")));
    assert!(!double_visit(&route, &Cave::Small("b")));
    assert!(double_visit(&route, &Cave::Small("a")));

    let route = vec![
        &Cave::Start,
        &Cave::Small("c"),
        &Cave::Big("D"),
        &Cave::Small("b")
    ];

    assert!(double_visit(&route, &Cave::Small("c")));
}

impl CaveSystem<'_> {
    fn from_vec<'a>(input: &'a Vec<&str>) -> CaveSystem<'a> {
        let mut map: HashMap<Cave, Vec<Cave>> = HashMap::new();
        for conn in input {
            let mut nodes: Vec<&str> = conn.split("-").collect();

            if nodes[1] == "start" {
                nodes.swap(1, 0);
            }

            if nodes[0] == "end" {
                nodes.swap(0, 1);
            }

            let (left_cave, right_cave) = (
                cave(nodes[0]),
                cave(nodes[1])
            );

            match map.get_mut(&left_cave) {
                Some(n) => { n.push(right_cave); },
                None => {
                    map.insert(left_cave, vec![right_cave]);
                }
            }

            if right_cave != Cave::End && left_cave != Cave::Start {
                match map.get_mut(&right_cave) {
                    Some(n) => { n.push(left_cave); },
                    None => {
                        map.insert(right_cave, vec![left_cave]);
                    }
                }
            }
        }

        CaveSystem { map: map }
    }

    fn count_paths(&self) -> usize {
        let mut to_visit = vec![];
        let mut routes = 0;

        to_visit.push(vec![&Cave::Start]);

        while let Some(route) = to_visit.pop() {
            let current = route[route.len() - 1];

            if &Cave::End == current {
                routes += 1;
            }

            if let Some(neighbors) = self.map.get(current) {
                for neighbor in neighbors {
                    let condition = match neighbor {
                        &Cave::Big(_) | &Cave::End => true,
                        &Cave::Small(_) => !route.contains(&neighbor),
                        _ => false
                    };

                    if condition {
                        let mut new_route = route.clone();
                        new_route.push(neighbor);
                        to_visit.push(new_route.clone());
                    }
                }
            }
        }

        routes
    }

    fn double_count_paths(&self) -> usize {
        let mut to_visit = vec![];
        let mut routes = 0;

        to_visit.push(vec![&Cave::Start]);

        while let Some(route) = to_visit.pop() {
            let current = route[route.len() - 1];

            if &Cave::End == current {
                routes += 1;
            }

            if let Some(neighbors) = self.map.get(current) {
                for neighbor in neighbors {
                    let condition = match neighbor {
                        &Cave::Big(_) | &Cave::End => true,
                        &Cave::Small(_) => double_visit(&route, neighbor),
                        _ => false
                    };

                    if condition {
                        let mut new_route = route.clone();
                        new_route.push(neighbor);
                        to_visit.push(new_route.clone());
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
    assert_eq!(
        system.map.get(&Cave::Big("A")),
        Some(&vec![Cave::Small("c"), Cave::Small("b"), Cave::End])
    );
    assert_eq!(
        system.map.get(&Cave::Small("c")),
        Some(&vec![Cave::Big("A")])
    );
    assert_eq!(
        system.map.get(&Cave::Small("b")),
        Some(&vec![Cave::Big("A"), Cave::Small("d"), Cave::End])
    );
    assert_eq!(system.map.get(&Cave::End), None);
    assert_eq!(system.count_paths(), 10);
    assert_eq!(system.double_count_paths(), 36);
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
    assert_eq!(
        system.map.get(&Cave::Start),
        Some(&vec![
             Cave::Big("HN"),
             Cave::Small("kj"),
             Cave::Small("dc")
        ])
    );
    assert_eq!(
        system.map.get(&Cave::Small("dc")),
        Some(&vec![
             Cave::End,
             Cave::Big("HN"),
             Cave::Big("LN"),
             Cave::Small("kj")
        ])
    );
    assert_eq!(
        system.map.get(&Cave::End),
        None
    );
    assert_eq!(system.count_paths(), 19);
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
    assert_eq!(system.count_paths(), 226);
}
