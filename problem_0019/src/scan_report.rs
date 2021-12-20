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
    baecons: Vec<Point>
}

impl ScanReport {
    pub fn from_str(input: &str) -> ScanReport {
        ScanReport { baecons: vec![] }
    }
}
