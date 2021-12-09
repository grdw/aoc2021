use std::fs;

fn main() {
	let display_string = fs::read_to_string("input")
                            .unwrap_or("".to_string());

    let height_map: Vec<Vec<i32>> = display_string
        .split_terminator("\n")
        .map(|line|
            line
                .chars()
                .map(|m| m.to_digit(10).unwrap() as i32)
                .collect()
        )
        .collect();

    let risk = risk_level(&height_map);
    println!("The risk level is: {:?}", risk);

    let max_basins = max_basins_size(&height_map);
    println!("The max basins size combined equal to: {:?}", max_basins);
}

fn get_point(grid: &Vec<Vec<i32>>,
             y: usize,
             x: usize,
             dir_y: i32,
             dir_x: i32) -> i32 {

    let temp = vec![];
    let y = y as i32 + dir_y;
    let x = x as i32 + dir_x;
    let y_row = grid.get(y as usize).unwrap_or(&temp);
    *y_row.get(x as usize).unwrap_or(&10)
}

fn risk_level(heightmap: &Vec<Vec<i32>>) -> i32 {
    low_points(heightmap).iter().map(|(n, _, _)| n + 1).sum()
}

fn low_points(heightmap: &Vec<Vec<i32>>) -> Vec<(i32, usize, usize)> {
    let grid_height = heightmap.len();
    let grid_width = heightmap[0].len();
    let directions = vec![(1, 0), (0, -1), (0, 1), (-1, 0)];
    let mut low_points = vec![];

    for y in 0..grid_height {
        for x in 0..grid_width {
            let current_min = heightmap[y as usize][x as usize];
            let surrounded: i32 = directions
                .iter()
                .map(|(dy, dx)| get_point(&heightmap, y, x, *dy, *dx))
                .min()
                .unwrap();

            if current_min < surrounded {
                low_points.push((current_min, x, y));
            }
        }
    }

    low_points
}

#[test]
fn test_risk_level() {
    let heightmap = vec![
        vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
        vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
        vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
        vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
        vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8]
    ];

    assert_eq!(risk_level(&heightmap), 15);

    let heightmap = vec![
        vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
        vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
        vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
        vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
        vec![9, 8, 9, 9, 9, 6, 6, 6, 7, 8]
    ];

    assert_eq!(risk_level(&heightmap), 9);

    let heightmap = vec![
        vec![1,1,3],
        vec![2,4,1]
    ];

    assert_eq!(risk_level(&heightmap), 2);
}

fn max_basins_size(heightmap: &Vec<Vec<i32>>) -> i32 {
    let lps = low_points(heightmap);
    let directions = vec![(1, 0), (0, -1), (0, 1), (-1, 0)];

    let mut basin_sizes: Vec<usize> = lps.iter().map(|(_, x, y)| {
        let mut points = vec![(*y as i32, *x as i32)];
        let mut start = true;
        let mut min = 0;
        let mut max;

        while start {
            max = points.len();

            let mut match_points: Vec<(i32, i32)> = vec![];

            for i in min..max {
                let (py, px) = points[i];
                let mut findable: Vec<(i32, i32)> = directions
                    .iter()
                    .filter(|(dy, dx)| {
                        let point = get_point(&heightmap, py as usize, px as usize, *dy, *dx);
                        let found_already = (0..points.len() - 1)
                            .any(|i| points[i].0 == py + dy && points[i].1 == px + dx);

                        let found_already_m = (0..match_points.len())
                            .any(|i| match_points[i].0 == py + dy && match_points[i].1 == px + dx);

                        point < 9 && !found_already && !found_already_m
                    })
                    .map(|(dy, dx)| (dy + py, dx + px))
                    .collect();

                if findable.len() > 0 {
                    min = max;
                    match_points.append(&mut findable);
                }
            }

            if match_points.is_empty() {
                start = false
            } else {
                points.append(&mut match_points);
            }
        }

        points.len()
    }).collect();

    basin_sizes.sort_by(|a, b| b.cmp(a));
    basin_sizes[0..3].iter().fold(1, |acc, a| acc * a) as i32
}

#[test]
fn test_max_basins_size() {
    let heightmap = vec![
        vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
        vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
        vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
        vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
        vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8]
    ];

    assert_eq!(max_basins_size(&heightmap), 1134);
}
