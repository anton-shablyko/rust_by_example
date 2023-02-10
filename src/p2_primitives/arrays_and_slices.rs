
//https://doc.rust-lang.org/stable/rust-by-example/primitives/array.html
//An array is a collection of objects of the same type


use std::mem;

fn analyze_slice(slice: &[i32]){
    println!("slice {:?}", slice);
    println!("first element of the slice: {}", slice[0]);
    println!("slice has {} elements", slice.len());
}

pub fn main() {
    // Fixed-size array
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // all elements can be instantiated with the same value
    let ys: [i32; 500] = [0; 500];   // [0, 0, ..., 0] 500 times

    // indexing starts at 0
    println!("first element of the array : {}", xs[0]);
    println!("second element of the array : {}", xs[1]);

    // `len` returns a number of elements in the array
    println!("number of elements in array: {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole aray as a slice");
    analyze_slice(&xs);

    // Slices can point to a part of the array
    // They are of the form [start_index .. ending index]
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);
    //  for [1,2,3,4,5]
    // [1 .. 5] == [2, 3, 4, 5]
    // [0 .. 2] == [1,2]

    // Example of empty slice
    let empty_array :[u32;0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // same but more verbose

    // Arrays can be safely accessed using `.get` which returns an
    // `Option`. This can be matched as shown below, or used with `expect()`
    // if you'd like the program exit with the nice message

    for i in 0 .. xs.len() + 1 {  // Oops! The element is out of index
        match xs.get(i) {
            Some(xval) => println!("index{}: value {}", i, xval),
            None => println!("Slow down! Index {} is too far!", i),
        }
    }
}

