use std::fs;

#[derive(Debug, Eq, Hash, Clone, PartialEq)]
enum Type {
    Baecon,
    Scanner
}

#[derive(Debug, Eq, Hash, Clone, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
    z: i32,
    t: Type
}

const SCAN_RANGE: i32 = 1000;
const MAX_ROTATIONS: usize = 5;

pub struct ScanReport {
    header: String,
    baecons: Vec<Point>
}

impl ScanReport {
    pub fn from_str(input: &str) -> ScanReport {
        let mut lines: Vec<&str> = input
            .split_terminator("\n")
            .collect();

        let header = lines.remove(0);
        let baecons = lines.iter().map(|line| {
                let xyz: Vec<i32> = line
                    .split_terminator(",")
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect();

                Point { x: xyz[0], y: xyz[1], z: xyz[2], t: Type::Baecon }
            })
            .collect();

        ScanReport { header: header.to_string(), baecons: baecons }
    }

    pub fn location(&self, first: &ScanReport) -> Point {
        for p in &first.baecons[0..5] {
            println!("FIRST BAECON: {},{},{}", p.x, p.y, p.z);
        }
        for p in &self.baecons[0..5] {
            println!("CURRENT BAECON: {},{},{}", p.x, p.y, p.z);
        }
        Point { t: Type::Scanner, x: 0, y: 0, z: 0 }
    }
}

#[test]
fn test_scan_report() {
    let input = "--- scanner 0 ---\n\
                 534,645,-322\n\
                 388,-625,-587\n\
                 -374,865,677\n\
                 699,588,504\n\
                 -319,800,690\n\
                 449,-685,-738\n\
                 884,581,405\n\
                 131,93,-1\n\
                 -235,-774,713\n\
                 -312,-878,705\n\
                 -329,-787,824\n\
                 -715,888,-428\n\
                 7,-85,64\n\
                 -382,720,536\n\
                 603,-680,489\n\
                 702,605,-358\n\
                 542,566,-374\n\
                 -385,-335,-417\n\
                 708,-707,515\n\
                 -325,-363,-531\n\
                 -349,-384,-424\n\
                 -605,815,-439\n\
                 777,605,400\n\
                 748,-688,524\n\
                 431,-699,-732\n\
                 -689,893,-402";

    let scan_report = ScanReport::from_str(input);
    assert_eq!(scan_report.baecons.len(), 26);
}

#[test]
fn test_scanner_location() {
    let contents = fs::read_to_string("test_input")
                      .unwrap_or("".to_string());

    let mut readings: Vec<ScanReport> = contents
        .split_terminator("\n\n")
        .map(|l| ScanReport::from_str(l))
        .collect();

    let first_report = readings.remove(0);
    let second_report = readings.remove(0);

    assert_eq!(
        second_report.location(&first_report),
        Point { t: Type::Scanner, x: 68, y: -1246, z: -43 }
    );
}
