mod scan_report;

use std::fs;
use scan_report::ScanReport;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input")
                      .unwrap_or("".to_string());

    let readings: Vec<ScanReport> = contents
        .split_terminator("\n\n")
        .map(|l| ScanReport::from_str(l))
        .collect();

    println!("How many unique beacons: {}", count_unique_baecons(&readings));
}

fn count_unique_baecons(readings: &Vec<ScanReport>) -> usize {
    let mut points = vec![];
    for scan_report in readings {
        for point in scan_report.rotations() {
            points.push(point);
        }
    }
    points.len()
}

#[test]
fn test_count_unique_baecons() {
    let contents = fs::read_to_string("test_input")
                      .unwrap_or("".to_string());

    let readings: Vec<ScanReport> = contents
        .split_terminator("\n\n")
        .map(|l| ScanReport::from_str(l))
        .collect();

    assert_eq!(count_unique_baecons(&readings), 79);
}
