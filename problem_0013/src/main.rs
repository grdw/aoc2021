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

    println!("Amount of folds after 1 time {:?}", fold_paper(&points, &folds, 1));
}

fn fold_paper(points: &Points, folds: &Folds, times: usize) -> usize {
    let mut current_points = points.clone();
    let mut visible_points = 0;

    for i in 0..times {
        let height = current_points.iter().map(|n| n.1).max().unwrap() + 1;
        let width = current_points.iter().map(|n| n.0).max().unwrap() + 1;
        let (axis, value) = folds[i];

        let mut folds: Points = vec![];
        let mut unfolds: Points = vec![];

        for (x, y) in &current_points {
            let val = if axis == "y" { y } else { x };

            if val > &value {
                folds.push((*x, *y));
            } else if val < &value {
                unfolds.push((*x, *y));
            }
        }

        for i in 0..folds.len() {
            let (x, y) = folds[i];
            let folded_point = if axis == "y" {
                (x, height - y - 1)
            } else {
                (width - x - 1, y)
            };

            if !unfolds.contains(&folded_point) {
                unfolds.push(folded_point)
            }
        }

        current_points = unfolds.clone();
        visible_points = unfolds.len();
    }

    visible_points
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

    assert_eq!(fold_paper(&points, &folds, 1), 17);
    assert_eq!(fold_paper(&points, &folds, 2), 16);
}
