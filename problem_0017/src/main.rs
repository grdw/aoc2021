use std::ops::RangeInclusive;

fn main() {
    let x_range = 143..=177;
    let y_range = -106..=-71;

    println!("The max height is: {}", max_probe(x_range, y_range));
}

fn max_probe(
    target_x: RangeInclusive<i16>,
    target_y: RangeInclusive<i16>
) -> i16 {
    let max = *target_x.end();
    let mut m = 0;
    for x in 1..max {
        for y in 1..max {
            let max_p = probe(x, y, &target_x, &target_y);

            if max_p > m {
                m = max_p;
            }
        }
    }

    m
}

#[test]
fn test_max_probe() {
    assert_eq!(max_probe(20..=30, -10..=-5), 45);
}

fn probe(
    mut x_vel: i16,
    mut y_vel: i16,
    target_x: &RangeInclusive<i16>,
    target_y: &RangeInclusive<i16>,
) -> i16 {
    let (mut x, mut y) = (0, 0);
    let mut steps = 0;
    let mut max_y = 0;

    loop {
        x += x_vel;
        y += y_vel;

        if x > 0 && x_vel > 0 {
            x_vel -= 1;
        } else if x < 0 {
            x_vel += 1;
        }

        y_vel -= 1; // gravity

        if y > max_y {
            max_y = y;
        }

        steps += 1;

        if &x > target_x.end() || &y < target_y.start() {
            break 0
        }

        if target_x.contains(&x) && target_y.contains(&y) {
            break max_y
        }
    }
}

#[test]
fn test_probe() {
    let range_x = 20..=30;
    let range_y = -10..=-5;
    assert_eq!(probe(7, 2, &range_x, &range_y), 3);
    assert_eq!(probe(6, 3, &range_x, &range_y), 6);
    assert_eq!(probe(9, 0, &range_x, &range_y), 0);
    assert_eq!(probe(17, -4, &range_x, &range_y), 0);
    assert_eq!(probe(6, 9, &range_x, &range_y), 45);
}
