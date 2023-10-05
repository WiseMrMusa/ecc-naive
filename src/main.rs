use ecc_naive::field_point::field_point;
use field_point::{Point, ECC, PointTrait};

fn main() {
    // let p1 = Point::new(&2, &5, &5, &7);
    // let p2 = Point::new(&2, &5, &5, &7);
    // let _p3 = Point::new(&-1, &-1, &5, &7);
    // println!("{}", p1 + p2)

    let ep = ECC::new(&5, &7);
    let _ep1 = ep.eleptic_point(&2, &5);
    let _ep2 = ep.eleptic_point(&2, &5);

    println!("{}", _ep1);
    println!("{}", _ep2);
}
