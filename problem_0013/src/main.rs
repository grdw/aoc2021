use std::fs;

type Points = Vec<(usize, usize)>;
type Folds<'a> = Vec<(&'a str, usize)>;

fn main() {
    let input = fs::read_to_string("input")
                   .unwrap_or("".to_string());

    let folds = fs::read_to_string("folds")
                   .unwrap_or("".to_string());

    let points: Points = input
        .split_terminator("\n")
        .map(|line| {
             let yx: Vec<usize> = line
                .split(",")
                .map(|t| t.parse::<usize>().unwrap())
                .collect();

             (yx[0], yx[1])
        })
        .collect();

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

        let mut folds: Points = vec![];
        let mut unfolds: Points = vec![];

        for i in (0..points.len()).rev() {
            let (x, y) = points[i];
            let val = if axis == "y" { y } else { x };

            if val > value {
                folds.push((x, y));
                points.remove(i);
            } else if val < value {
                unfolds.push((x, y));
            }
        }

        for i in 0..folds.len() {
            let (x, y) = folds[i];
            let folded_point = if axis == "y" {
                (x, height - y)
            } else {
                (width - x, y)
            };

            if !points.contains(&folded_point) {
                points.push(folded_point)
            }
        }
    }
}

#[test]
fn test_transparent_origami() {
    let points = vec![
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
