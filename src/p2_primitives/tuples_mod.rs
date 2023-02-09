// https://doc.rust-lang.org/stable/rust-by-example/primitives/tuples.html
use std::fmt::{Display, Formatter, write};

// Tuples can be used as function arguments and a return value
fn reverse(pair:(i32, bool)) -> (bool, i32) {
    // in the same way as in python a, b = (xz)
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
}

// Start of custom activity
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix{
    fn fmt(&self, f: &mut Formatter<>) -> std::fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3 )
    }
}

fn transpose(mtrx:Matrix) -> Matrix {
    Matrix(mtrx.0, mtrx.2, mtrx.1, mtrx.3)
}

// End of custom activity


pub fn main(){
     // a tuple with a bunch of diferent types
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64,
        0.1f32, 0.2f64, 'a', true);

    // Extracting values from tuple using indexing
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);


    // Nested tuples are ok
    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), (-2i16, -3i32));

    // Tuples are printable through debug
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // But long Tuples (more than 12 elements) cannot be printed
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error


    let pair =(1i32, true);
    println!("pair is {:?}", pair);

    println!("reversed_pair is {:?}", reverse(pair));

    // One element tuple vs a literal
    println!("One element tuple {:?}", (5i32,));
    println!("One integer {:?}", (5i32));

    // tuples can be distructed to create bindings
    let tuple = (1, "hello", 4.5, true);
    let (a,b,c,d) = tuple;
    println!("{:?},{:?},{:?},{:?}", a, b,c, d);


    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);
    println!("Transposed Matrix:\n{}", transpose(matrix));


}