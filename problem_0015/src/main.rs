use std::fs;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

type Grid = Vec<Vec<Node>>;

#[derive(Debug, Clone)]
struct Node(usize, usize);

#[derive(Debug, Clone)]
struct Edge(usize, usize);
type Edges = Vec<Vec<Edge>>;

fn main() {
    let display_string = fs::read_to_string("input")
                            .unwrap_or("".to_string());

    let risk = risk_level(&display_string, 0, 9999);
    println!("The risk level is: {:?}", risk);
}

fn to_grid(input: &str) -> Grid {
    let mut grid: Grid = vec![];
    let mut id = 0;

    for line in input.split_terminator("\n") {
        let mut points = vec![];

        for cha in line.chars() {
            let value = cha.to_digit(10).unwrap() as usize;
            points.push(Node(id, value));
            id += 1;
        }

        grid.push(points);
    }

    grid
}

fn to_graph(input: &str) -> Edges {
    let grid = to_grid(input);
    let size = grid.len();

    let mut edges: Edges = vec![vec![]; size.pow(2)];

    for y in 0..size {
        for x in 0..size {
            let current = &grid[y][x];

            if let Some(row) = grid.get(y + 1) {
                edges[current.0].push(Edge(
                    row[x].0,
                    row[x].1 as usize
                ));
            }

            if let Some(cell) = grid[y].get(x + 1) {
                edges[current.0].push(Edge(
                    cell.0,
                    cell.1 as usize
                ));
            }
        }
    }

    edges
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn risk_level(input: &str, start: usize, goal: usize) -> Option<usize> {
    let edges = to_graph(input);
    let mut dist: Vec<_> = (0..edges.len()).map(|_| usize::MAX).collect();
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal { return Some(cost); }
        if cost > dist[position] { continue; }

        for edge in &edges[position] {
            let next = State { cost: cost + edge.1, position: edge.0 };

            if next.cost < dist[next.position] {
                heap.push(next);

                dist[next.position] = next.cost;
            }
        }
    }

    None
}


#[test]
fn test_fast_route() {
    let display_string = fs::read_to_string("test_input")
                            .unwrap_or("".to_string());

    let risk = risk_level(&display_string, 0, 99);
    assert_eq!(risk.unwrap_or(0), 40);
}
