use ecc_naive::field_point::field_point;
use field_point::{ ECC, PointTrait};

fn main() {

    let ep = ECC::new(&5, &7);
    let _ep1 = ep.eleptic_point(&2, &5);
    let _ep2 = ep.eleptic_point(&2, &5);

    println!("{}", _ep1);
    println!("{}", _ep2);
}
