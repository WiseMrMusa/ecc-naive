use ecc_naive::field_point::field_point;
use field_point::Point;

fn main() {
    let p1 = Point::new(&2, &5, &5, &7);
    let p2 = Point::new(&2, &5, &5, &7);
    let _p3 = Point::new(&-1, &-1, &5, &7);
    println!("{}", p1 + p2);
}
