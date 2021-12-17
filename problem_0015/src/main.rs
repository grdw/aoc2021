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

    let graph = to_grid(&display_string, 1);
    let risk = risk_level(&graph, 0, 9999);
    println!("The risk level is: {:?}", risk);

    let graph = to_grid(&display_string, 5);
    let risk = risk_level(&graph, 0, 249_999);
    println!("The risk level is: {:?}", risk);
}

fn to_grid(input: &str, repeat: usize) -> Grid {
    let lines: Vec<&str> = input.split_terminator("\n").collect();
    let mut grid: Grid = vec![];
    let mut id = 0;
    let size = lines.len();
    let total_size = size * repeat;

    for y in 0..total_size {
        let line = lines[y % size];
        let mut points = vec![];

        for x in 0..total_size {
            let cha = line.chars().nth(x % size).unwrap();
            let value = cha.to_digit(10).unwrap() as usize;
            let total = value + (x / size) + (y / size);

            let cost = if total < 10 {
                total
            } else {
                total - 9
            };

            points.push(Node(id, cost));
            id += 1;
        }

        grid.push(points);
    }

    grid
}

#[test]
fn test_to_grid() {
    let display_string = fs::read_to_string("test_input")
                            .unwrap_or("".to_string());

    let g = to_grid(&display_string, 5);
    assert_eq!(g[0][11].1 - 1, g[0][0].1);
    assert_eq!(g[11][0].1 - 1, g[0][0].1);
    assert_eq!(g.len(), 50);
    assert_eq!(g[0].len(), 50);
}

fn to_graph(grid: &Grid) -> Edges {
    let size = grid.len();
    let directions: Vec<(isize, isize)> = vec![
        (-1, 0), (0, -1), (1, 0), (0, 1)
    ];
    let mut edges: Edges = vec![vec![]; size.pow(2)];

    for y in 0..size {
        for x in 0..size {
            let current = &grid[y][x];
            let x = x as isize;
            let y = y as isize;

            for (dy, dx) in &directions {
                if let Some(row) = grid.get((y + dy) as usize) {
                    if let Some(cell) = row.get((x + dx) as usize) {
                        edges[current.0].push(Edge(cell.0, cell.1));
                    }
                }
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

fn risk_level(grid: &Grid, start: usize, goal: usize) -> Option<usize> {
    let edges = to_graph(grid);
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

    let grid = to_grid(&display_string, 1);
    let risk = risk_level(&grid, 0, 99);
    assert_eq!(risk.unwrap_or(0), 40);

    let grid = to_grid(&display_string, 5);
    let risk = risk_level(&grid, 0, 2499);
    assert_eq!(risk.unwrap_or(0), 315);
}
