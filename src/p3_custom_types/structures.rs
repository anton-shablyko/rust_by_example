
// There are 3 types of structs out there
// Tuple structs - which a basically named tuples
// Classic? C structs
// Unit structs -  which are fieldless and useful for generics
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// a Unit struct
struct Unit;

// a tuple struct
#[derive(Debug)]
struct Pair (i32, i32);

// a struct with 2 fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Struct can be reused as a field in another struct
struct Rectangle {
    top_left: Point,
    bottom_right: Point
}

pub fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // print/debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };
    // access the fields of the point
    println!("Point's coordinates - x: {}, y: {}", point.x, point.y);

    // Make a new point by updating existing point
    let bottom_right = Point { x: 5.2, ..point };  // {x: 5.2, y: 0.4 }

    //Destructure the point using a `let` structure
    let Point { x: left_edge, y: top_edge } = point;
    println!("{:?}", point);
    println!("{}", left_edge);
    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 2);
    println!("{:?}", pair);

    // Access the fileds of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    //Destructure a tuple struct
    let Pair(integer_a, integer_b) = pair;
    println!("pair contains {:?} and {:?}", integer_a, integer_b);
}