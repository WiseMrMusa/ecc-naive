#[derive(Debug)]
pub struct Point {
    pub x: u64,
    pub y: u64,
    pub a: u64,
    pub b: u64,
}

impl Point {
    pub fn new(x: &u64, y: &u64, a: &u64, b: &u64) -> Point {
        assert_eq!(y.pow(2), x.pow(3) + a*x + 4, "This is not a point on an eleptic curve");
        Point { x: *x, y: *y, a: *a, b: *b }
    }
}

