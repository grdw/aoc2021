mod scan_report;

use std::fs;
use scan_report::ScanReport;

fn main() {
    let contents = fs::read_to_string("input")
                      .unwrap_or("".to_string());

    let readings: Vec<ScanReport> = contents
        .split_terminator("\n\n")
        .map(|l| ScanReport::from_str(l))
        .collect();
    println!("Hello, world!");
}
