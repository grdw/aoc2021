use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("lines")
                   .unwrap_or("".to_string());

    let lines: Vec<&str> = input
        .split_terminator("\n")
        .collect();

    println!("Amount of overlaps of 2 (no diagonals): {:?}", two_line_overlaps(&lines, false));
    println!("Amount of overlaps of 2 (incl. diagonals): {:?}", two_line_overlaps(&lines, true));
}

#[derive(Debug, Hash, Eq)]
struct Point {
    x: u32,
    y: u32
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point
}

impl Line {
    fn from(input: &str) -> Line {
        let points: Vec<Vec<u32>> = input
            .split(" -> ")
            .map(|coords|
                 coords
                    .split(",")
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect()
            )
            .collect();

        Line {
            p1: Point { x: points[0][0], y: points[0][1] },
            p2: Point { x: points[1][0], y: points[1][1] }
        }
    }

    pub fn is_horizontal(&self) -> bool {
        self.p1.x == self.p2.x
    }

    pub fn is_vertical(&self) -> bool {
        self.p1.y == self.p2.y
    }

    fn list_coords(&self, m1: u32, m2: u32) -> Vec<u32> {
        let mut list = vec![];
        let (min, max, pos) = if m1 > m2 {
            (m2, m1, None)
        } else {
            (m1, m2, Some(0))
        };

        for m in min..=max {
            let position = match pos {
                Some(n) => n,
                None => list.len()
            };

            list.insert(position, m);
        }

        list
    }

    pub fn points(&self) -> Vec<Point> {
        let mut points = vec![];
        let xlist = self.list_coords(self.p1.x, self.p2.x);
        let ylist = self.list_coords(self.p1.y, self.p2.y);
        let mut xlist_iter = xlist.iter();
        let mut ylist_iter = ylist.iter();

        loop {
            let point = match (xlist_iter.next(), ylist_iter.next()) {
                (Some(x), Some(y)) => Point {x: *x, y: *y },
                (Some(x), None) => Point { x: *x, y: self.p1.y },
                (None, Some(y)) => Point { x: self.p1.x, y: *y },
                (None, None) => break
            };

            points.push(point)
        }

        points
    }
}

#[test]
fn test_is_direction() {
    let vertical_line = Line::from("0,9 -> 5,9");
	let horizontal_line = Line::from("2,2 -> 2,1");

    assert!(horizontal_line.is_horizontal());
    assert!(!horizontal_line.is_vertical());
    assert!(vertical_line.is_vertical());
    assert!(!vertical_line.is_horizontal());
}

#[test]
fn test_intermediary_points() {
	let horizontal_line = Line::from("3,3 -> 3,1");
    let points = horizontal_line.points();

    assert_eq!(points[1], Point { x: 3, y: 2 });
}

fn two_line_overlaps(input: &Vec<&str>, incl_diagonals: bool) -> usize {
    let mut point_counts: HashMap<Point, u32> = HashMap::new();

    for line in input {
        let l = Line::from(line);
        let points = l.points();

        if incl_diagonals || l.is_horizontal() || l.is_vertical() {
            for p in points {
                let counter = point_counts.entry(p).or_insert(0);
                *counter += 1;
            }
        }
    }

    point_counts.iter().filter(|(_, &count)| count >= 2).count()
}

#[test]
fn test_overlaps() {
	let lines = vec![
		"0,9 -> 5,9",
		"8,0 -> 0,8",
		"9,4 -> 3,4",
		"2,2 -> 2,1",
		"7,0 -> 7,4",
		"6,4 -> 2,0",
		"0,9 -> 2,9",
		"3,4 -> 1,4",
		"0,0 -> 8,8",
		"5,5 -> 8,2"
    ];

    let t1 = two_line_overlaps(&lines, false);
    let t2 = two_line_overlaps(&lines, true);

    assert_eq!(t1, 5);
    assert_eq!(t2, 12);
}
