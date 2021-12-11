use std::fs;

type Grid = Vec<Vec<u8>>;

fn main() {
    let display_string = fs::read_to_string("input")
                            .unwrap_or("".to_string());

    let height_map: Grid = display_string
        .split_terminator("\n")
        .map(|line|
            line
                .chars()
                .map(|m| m.to_digit(10).unwrap() as u8)
                .collect()
        )
        .collect();

    let risk = risk_level(&height_map);
    println!("The risk level is: {:?}", risk);

    let max_basins = max_basins_size(&height_map);
    println!("The max basins size combined equal to: {:?}", max_basins);
}

fn get_points(grid: &Grid, y: usize, x: usize) -> Vec<(i32, i32, u8)> {
    let directions = vec![(1, 0), (0, -1), (0, 1), (-1, 0)];

    directions.iter().map(|(dy, dx)| {
        let temp = vec![];
        let y = y as i32 + dy;
        let x = x as i32 + dx;
        let y_row = grid.get(y as usize).unwrap_or(&temp);

        (*dy, *dx, *y_row.get(x as usize).unwrap_or(&10))
    }).collect()
}

fn risk_level(heightmap: &Grid) -> u32 {
    low_points(heightmap).iter().map(|(_, _, n)| *n as u32 + 1).sum()
}

fn low_points(heightmap: &Grid) -> Vec<(usize, usize, u8)> {
    let grid_height = heightmap.len();
    let grid_width = heightmap[0].len();
    let mut low_points = vec![];

    for y in 0..grid_height {
        for x in 0..grid_width {
            let current_min = heightmap[y][x];
            let surrounded: u8 = *get_points(&heightmap, y, x)
                .iter()
                .map(|(_, _, value)| value )
                .min()
                .unwrap();

            if current_min < surrounded {
                low_points.push((y, x, current_min));
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

fn basin_size(heightmap: &Grid, y: i32, x: i32) -> usize {
    let mut points = vec![(y, x)];
    let mut start = 0;

    while start < points.len() {
        let (py, px) = points[start];
        let mut findable: Vec<(i32, i32)> =
            get_points(&heightmap, py as usize, px as usize)
                .iter()
                .filter(|(_, _, point)| *point < 9)
                .map(|(sy, sx, _)| (sy + py, sx + px))
                .filter(|t| !points.contains(&t))
                .collect();

        points.append(&mut findable);
        start += 1;
    }

    points.len()
}

fn max_basins_size(heightmap: &Grid) -> u32 {
    let mut basin_sizes: Vec<usize> = low_points(heightmap)
        .iter()
        .map(|(y, x, _)| basin_size(heightmap, *y as i32, *x as i32))
        .collect();

    basin_sizes.sort_by(|a, b| b.cmp(a));
    basin_sizes[0..3].iter().fold(1, |acc, a| acc * a) as u32
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
