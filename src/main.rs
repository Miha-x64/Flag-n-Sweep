extern crate generic_matrix;
extern crate rand;

use mines::*;

mod matrix;

fn main() {
    let field = Field::generate(10, 30, 30, 3, 3, &mut rand::thread_rng());
    println!("{:?}", field);

    println!();

    let session = Session::from_field(&field, 3, 3);
    println!("{:?}", session);
}
