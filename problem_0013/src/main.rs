use std::fs;
use std::collections::HashSet;

type Points = HashSet<(usize, usize)>;
type Folds<'a> = Vec<(&'a str, usize)>;

fn main() {
    let input = fs::read_to_string("input")
                   .unwrap_or("".to_string());

    let folds = fs::read_to_string("folds")
                   .unwrap_or("".to_string());

    let mut points: Points = HashSet::new();

    for line in input.split_terminator("\n") {
         let yx: Vec<usize> = line
            .split(",")
            .map(|t| t.parse::<usize>().unwrap())
            .collect();

         points.insert((yx[0], yx[1]));
    }

    let folds: Folds = folds
        .split_terminator("\n")
        .map(|line| {
             let axis_value: Vec<&str> = line.split("=").collect();
             (axis_value[0], axis_value[1].parse::<usize>().unwrap())
        })
        .collect();

    let mut points_clone = points.clone();
    fold_paper(&mut points_clone, &folds, 1);
    println!("Amount of folds after 1 time {:?}",
             points_clone.len());

    let mut points_clone = points.clone();
    fold_paper(&mut points_clone, &folds, folds.len());
    display_paper(&points_clone);
}

fn display_paper(points: &Points) {
    let height = points.iter().map(|n| n.1).max().unwrap() + 1;
    let width = points.iter().map(|n| n.0).max().unwrap() + 1;
    let mut paper = vec![vec!['⬛'; width]; height];

    for (x, y) in points {
        paper[*y][*x] = '⬜';
    }

    println!("");
    for line in paper {
        let s: String = line.into_iter().collect();
        println!("{}", s);
    }
}

fn fold_paper(points: &mut Points, folds: &Folds, times: usize) {
    for i in 0..times {
        let (axis, value) = folds[i];
        let height = points.iter().map(|n| n.1).max().unwrap();
        let width = points.iter().map(|n| n.0).max().unwrap();

        let mut folds: Points = HashSet::new();
        let mut unfolds: Points = HashSet::new();

        for (x, y) in points.iter() {
            let val = if axis == "y" { *y } else { *x };

            if val > value {
                folds.insert((*x, *y));
            } else if val < value {
                unfolds.insert((*x, *y));
            }
        }

        for (x, y) in folds {
            points.remove(&(x, y));

            let folded_point = if axis == "y" {
                (x, height - y)
            } else {
                (width - x, y)
            };

            points.insert(folded_point);
        }
    }
}

#[test]
fn test_transparent_origami() {
    let input = vec![
        (6,10),
        (0,14),
        (9,10),
        (0,3),
        (10,4),
        (4,11),
        (6,0),
        (6,12),
        (4,1),
        (0,13),
        (10,12),
        (3,4),
        (3,0),
        (8,4),
        (1,10),
        (2,14),
        (8,10),
        (9,0),
    ];

    let mut points = HashSet::new();
    for p in input {
        points.insert(p);
    }

    let folds = vec![
        ("y", 7),
        ("x", 5)
    ];

    let mut points_clone = points.clone();
    fold_paper(&mut points_clone, &folds, 1);
    assert_eq!(points_clone.len(), 17);

    let mut points_clone = points.clone();
    fold_paper(&mut points_clone, &folds, 2);
    assert_eq!(points_clone.len(), 16);
}
