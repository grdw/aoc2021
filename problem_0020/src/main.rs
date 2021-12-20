use std::fs;

type Grid = Vec<Vec<char>>;


fn main() {
    let algorithm = fs::read_to_string("algorithm")
                      .unwrap_or("".to_string());

    let contents = fs::read_to_string("input")
                      .unwrap_or("".to_string());

    let mut grid = grid(&contents);
    for _ in 0..2 {
        enhance(&mut grid, &algorithm);
    }

    display(&grid);
    println!("Part 1: {}", lit_points(&grid));
}

fn grid(contents: &str) -> Grid {
    let rows: Vec<&str> = contents.split_terminator("\n").collect();
    let size = rows.len();
    let mut points = vec![];

    for row in rows {
        let mut n: Vec<char> = row.chars().collect();

        for _ in 0..3 {
            n.push('.');
            n.insert(0, '.');
        }

        points.push(n);
    }

    let row = vec!['.'; size + 6];
    for _ in 0..3 {
        points.push(row.clone());
        points.insert(0, row.clone());
    }

    points
}

#[test]
fn test_grid() {
    let test_image = "#..#.\n\
                      #....\n\
                      ##..#\n\
                      ..#..\n\
                      ..###";

    let grid = grid(&test_image);
    assert_eq!(grid.len(), 11);
    assert_eq!(grid[0].len(), 11);
}

fn binary(grid: &Grid, x: usize, y: usize) -> usize {
    let points = vec![
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),  (0, 0), (0, 1),
        (1, -1),  (1, 0), (1, 1)
    ];

    let mut result = String::new();

    for (dy, dx) in points {
        let x = x as isize + dx;
        let y = y as isize + dy;

        let temp = vec![];
        let row = grid.get(y as usize).unwrap_or(&temp);
        let point = row.get(x as usize).unwrap_or(&'.');

        let binary = match point {
            '.' => '0',
            '#' => '1',
            _ => panic!("Invalid char")
        };

        result.push(binary);
    }

    usize::from_str_radix(&result, 2).unwrap()
}

#[test]
fn test_binary() {
    let test_image = "#..#.\n\
                      #....\n\
                      ##..#\n\
                      ..#..\n\
                      ..###";

    let grid = grid(&test_image);
    display(&grid);

    assert_eq!(binary(&grid, 5, 5), 34);
}

fn enhance(grid: &mut Grid, algorithm: &str) {
    let height = grid.len();
    let width = grid[0].len();
    let mut replacements = vec![];

    for y in 0..height {
        for x in 0..width {
            let alg_index = binary(grid, x, y);
            let c = algorithm.chars().nth(alg_index).unwrap();
            replacements.push((x, y, c));
        }
    }

    for (x, y, c) in replacements {
        grid[y][x] = c;
    }

    // Extend for next iterations
    for y in 0..height {
        grid[y].insert(0, '.');
        grid[y].push('.');
    }

    grid.insert(0, vec!['.'; height + 2]);
    grid.push(vec!['.'; height + 2]);
}

fn display(grid: &Grid) {
    for row in grid {
        let s: String = row.into_iter().collect();
        println!("{}", s);
    }
}

fn lit_points(grid: &Grid) -> u64 {
    let mut count = 0;
    for row in grid {
        for c in row {
            if *c == '#' {
                count += 1;
            }
        }
    }
    count
}

#[test]
fn test_enhance() {
    let test_image = "#..#.\n\
                      #....\n\
                      ##..#\n\
                      ..#..\n\
                      ..###";

    let algorithm = fs::read_to_string("test_algorithm")
                      .unwrap_or("".to_string());

    let mut grid = grid(&test_image);
    for _ in 0..2 {
        enhance(&mut grid, &algorithm);
    }

    assert_eq!(lit_points(&grid), 35);
}
