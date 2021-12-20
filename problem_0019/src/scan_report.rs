enum Type {
    Baecon,
    Scanner
}

struct Point {
    x: i32,
    y: i32,
    z: i32,
    t: Type
}

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
