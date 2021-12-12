use std::fs;
use std::collections::HashMap;
use std::{thread, time::Duration};

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
                None => {
                    map.insert(nodes[0], vec![nodes[1]]);
                }
            }
        }

        CaveSystem { map: map }
    }

    fn count_paths(&self, start: &str) -> usize {
        let mut to_visit = vec![];
        let mut routes = vec![];

        to_visit.push(vec![start]);

        while let Some(route) = to_visit.pop() {
            let current = route[route.len() - 1];

            if let Some(neighbors) = self.map.get(current) {
                for neighbor in neighbors {
                    let mut new_route = route.clone();

                    new_route.push(*neighbor);
                    to_visit.push(new_route.clone());

                    println!("{:?}", new_route);
                    thread::sleep(Duration::from_millis(1500));

                    if *neighbor == "end" {
                        routes.push(new_route.clone());
                    }
                }
            }
        }

        routes.len()
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
    assert_eq!(system.count_paths("start"), 3);
}

//#[test]
//fn test_passage_pathing_complex_example() {
//    let other_example = vec![
//        "dc-end",
//        "HN-start",
//        "start-kj",
//        "dc-start",
//        "dc-HN",
//        "LN-dc",
//        "HN-end",
//        "kj-sa",
//        "kj-HN",
//        "kj-dc"
//    ];
//
//    let system = CaveSystem::from_vec(&other_example);
//    assert_eq!(system.map.get("start"), Some(&vec!["HN", "kj", "dc"]));
//    assert_eq!(system.map.get("dc"), Some(&vec!["end", "HN", "LN", "kj"]));
//    assert_eq!(system.map.get("end"), None);
//    //assert_eq!(system.count_paths("start"), 19);
//}
