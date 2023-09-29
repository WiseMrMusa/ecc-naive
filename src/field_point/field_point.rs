use std::fmt::{ Display, Result };
use std::ops::Add;
use std::cmp::PartialEq;

#[derive(Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64,
    pub a: i64,
    pub b: i64,
}


impl Point {
    pub fn new(x: &i64, y: &i64, a: &i64, b: &i64) -> Point {
        assert_eq!(y.pow(2), x.pow(3) + a * x + b, "This is not a point on an eleptic curve");
        Point { x: *x, y: *y, a: *a, b: *b }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "y^2 = x^3 + {}x + {} :: ({}, {})", self.a, self.b, self.x, self.y,)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        assert!(self.a == other.a && self.b == other.b, "Not the same cuve");
        self.x == other.x && self.y == other.y
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if self.a != rhs.a || self.b != rhs.b {
            panic!("Points {} and {} are not on the same curve", self, rhs);
        } else if self.x == 0 {
            rhs
        } else if rhs.y == 0 {
            self
        } else if self != rhs {
            let s = (self.y - rhs.y) / (self.x - rhs.x);
            let x = s.pow(2) - self.x - rhs.x;
            let y = s * (self.x - x) - self.y;
            Point { x, y, a: self.a, b: self.b }
        } else {
            if self.y == 0 {
                Point { x:0, y:0, a: self.a, b: self.b }
            } else {
                let s = (3*self.x.pow(2) + self.a) / (2 * self.y);
                let x = s.pow(2) - 2 * self.x;
                let y = s * (self.x - x) - self.y;
                Point { x, y, a: self.a, b: self.b }
            }
        }
    }
}
