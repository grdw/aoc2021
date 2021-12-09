fn get_point(grid: &Vec<Vec<i32>>,
             y: usize,
             x: usize,
             dir_y: i32,
             dir_x: i32) -> Option<i32> {

    let temp = vec![];
    let y = y as i32 + 1 * dir_y;
    let x = x as i32 + 1 * dir_x;
    let y_row = grid.get(y as usize).unwrap_or(&temp);
    y_row.get(x as usize).cloned()
}

fn risk_level(heightmap: &Vec<Vec<i32>>) -> i32 {
    let grid_height = heightmap.len();
    let grid_width = heightmap[0].len();
    let directions = vec![(1, 0), (0, -1), (0, 0), (0, 1), (-1, 0)];
    let mut sum_low_points: i32 = 0;

    for y in 0..grid_height {
        for x in 0..grid_width {
            let min = heightmap[y as usize][x as usize];
            let surrounded_min = directions
                .iter()
                .map(|(dy, dx)| get_point(&heightmap, y, x, *dy, *dx).unwrap_or(min))
                .min()
                .unwrap();

            if surrounded_min == min {
                sum_low_points += min + 1
            }
        }
    }

    sum_low_points
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

    assert_eq!(risk_level(&heightmap), 15)
}
