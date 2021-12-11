use std::fs;

type Grid = Vec<Vec<u8>>;

//fn debug(grid: &Grid) -> String {
//    let size = grid.len();
//    let mut line = String::from("");
//    for y in 0..size {
//        for x in 0..size {
//            line.push(char::from_digit(grid[y][x] as u32, 10).unwrap());
//        }
//        line.push('\n');
//    }
//    line
//}

fn main() {
    let input = fs::read_to_string("input")
                   .unwrap_or("".to_string());

    let mut octopuses: Grid = input
        .split_terminator("\n")
        .map(|line|
            line
                .chars()
                .map(|m| m.to_digit(10).unwrap() as u8)
                .collect()
        )
        .collect();

    println!("Amount of flashes: {}", dumbo_octopus_flashes(&mut octopuses, 100));
}

fn get_points(grid: &Grid, y: usize, x: usize) -> Vec<(i32, i32, u8)> {
    let directions = vec![
        (-1, -1), (-1, 0), (-1, 1), // TL  T  TR
        (0, -1),           (0, 1),  // L      R
        (1, -1),  (1, 0),  (1, 1)   // BL  B  BR
    ];

    let mut found_points = vec![];

    for (dy, dx) in directions {
        let temp = vec![];
        let y = y as i32 + dy;
        let x = x as i32 + dx;
        let y_row = grid.get(y as usize).unwrap_or(&temp);

        if let Some(p) = y_row.get(x as usize) {
            found_points.push((y, x, *p));
        }
    }

    found_points
}

#[test]
fn test_get_points() {
    let example = vec![
        vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
        vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
        vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
        vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
        vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
        vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
        vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
        vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
        vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
        vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6]
    ];

    assert_eq!(
        get_points(&example, 0, 0),
        vec![(0, 1, 4), (1, 0, 2), (1, 1, 7)]
    );
}

fn dumbo_octopus_flashes(octopuses: &mut Grid, steps: usize) -> u64 {
    let size = octopuses.len();
    let mut flashes = 0;

    for _ in 0..steps {
        let mut flash_points = vec![];
        let mut start = 0;

        for y in 0..size {
            for x in 0..size {
                octopuses[y][x] += 1;

                if octopuses[y][x] > 9 {
                    flash_points.push((y, x));
                }
            }
        }

        while start < flash_points.len() {
            let (fy, fx) = flash_points[start];
            let surroundings = get_points(&octopuses, fy, fx);

            for (sy, sx, _) in surroundings {
                let ssy = sy as usize;
                let ssx = sx as usize;
                let point = (ssy, ssx);

                octopuses[ssy][ssx] += 1;

                if octopuses[ssy][ssx] > 9 &&
                    !flash_points.contains(&point) {

                    flash_points.push(point);
                }
            }

            start += 1
        }

        for (fy, fx) in flash_points {
            if octopuses[fy][fx] > 9 {
                flashes += 1;
                octopuses[fy][fx] = 0;
            }
        }
    }

    flashes
}

#[test]
fn test_dumbo_octopus_flashes() {
    let mut small_example = vec![
        vec![1, 1, 1, 1, 1],
        vec![1, 9, 9, 9, 1],
        vec![1, 9, 1, 9, 1],
        vec![1, 9, 9, 9, 1],
        vec![1, 1, 1, 1, 1]
    ];

    assert_eq!(dumbo_octopus_flashes(&mut small_example, 1), 9);
    assert_eq!(small_example, vec![
        vec![3, 4, 5, 4, 3],
        vec![4, 0, 0, 0, 4],
        vec![5, 0, 0, 0, 5],
        vec![4, 0, 0, 0, 4],
        vec![3, 4, 5, 4, 3]
    ]);

    let mut example = vec![
        vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
        vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
        vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
        vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
        vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
        vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
        vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
        vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
        vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
        vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6]
    ];

    assert_eq!(dumbo_octopus_flashes(&mut example, 1), 0);
    assert_eq!(example, vec![
        vec![6, 5, 9, 4, 2, 5, 4, 3, 3, 4],
        vec![3, 8, 5, 6, 9, 6, 5, 8, 2, 2],
        vec![6, 3, 7, 5, 6, 6, 7, 2, 8, 4],
        vec![7, 2, 5, 2, 4, 4, 7, 2, 5, 7],
        vec![7, 4, 6, 8, 4, 9, 6, 5, 8, 9],
        vec![5, 2, 7, 8, 6, 3, 5, 7, 5, 6],
        vec![3, 2, 8, 7, 9, 5, 2, 8, 3, 2],
        vec![7, 9, 9, 3, 9, 9, 2, 2, 4, 5],
        vec![5, 9, 5, 7, 9, 5, 9, 6, 6, 5],
        vec![6, 3, 9, 4, 8, 6, 2, 6, 3, 7]
    ]);
    assert_eq!(dumbo_octopus_flashes(&mut example, 1), 35);
    assert_eq!(example, vec![
        vec![8, 8, 0, 7, 4, 7, 6, 5, 5, 5],
        vec![5, 0, 8, 9, 0, 8, 7, 0, 5, 4],
        vec![8, 5, 9, 7, 8, 8, 9, 6, 0, 8],
        vec![8, 4, 8, 5, 7, 6, 9, 6, 0, 0],
        vec![8, 7, 0, 0, 9, 0, 8, 8, 0, 0],
        vec![6, 6, 0, 0, 0, 8, 8, 9, 8, 9],
        vec![6, 8, 0, 0, 0, 0, 5, 9, 4, 3],
        vec![0, 0, 0, 0, 0, 0, 7, 4, 5, 6],
        vec![9, 0, 0, 0, 0, 0, 0, 8, 7, 6],
        vec![8, 7, 0, 0, 0, 0, 6, 8, 4, 8]
    ]);

    let mut example = vec![
        vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
        vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
        vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
        vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
        vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
        vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
        vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
        vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
        vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
        vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6]
    ];

    assert_eq!(dumbo_octopus_flashes(&mut example, 100), 1656);
}
