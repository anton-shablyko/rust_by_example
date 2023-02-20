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
struct Pair(i32, i32);

// a struct with 2 fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Struct can be reused as a field in another struct
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
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
    let bottom_right = Point { x: 5.2, ..point }; // {x: 5.2, y: 0.4 }

    //Destructure the point using a `let` structure
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;
    println!("{:?}", point);
    println!("{}", left_edge);
    let _rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
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

    let r1: Rectangle = Rectangle {
        top_left: Point { x: 1.0, y: 3.0 },
        bottom_right: Point { x: 3.0, y: 5.0 },
    };

    println!("Area of rectangle is: {} ", rect_area(r1));

    let r2 = square(Point { x: 0.0, y: 0.0 }, 4.0);

    println!("Area of rectangle #2 is {}", rect_area(r2));
    // println!("{:?}", square(Point { x: 0.0, y: 0.0 }, 2.0));
}
// todo: assigment
// 1.  Create a function `rect_area`  which calculates the area of a rectangle
// Use Rectangle struct as an argument
fn rect_area(rect: Rectangle) -> f32 {
    let height = rect.bottom_right.x - rect.top_left.x;
    let width = rect.bottom_right.y - rect.top_left.y;
    return height * width;
}

// 2. Add a function square which takes a point and a f32 arguments and returns a Rectangle
// with its top left corner on the point, and a width and height corresponding to the f32.
fn square(p: Point, h: f32) -> Rectangle {
    let Point {
        x: start_x,
        y: start_y,
    } = p;

    let b_right = Point {
        x: start_x + h,
        y: start_y + h,
    };
    return Rectangle {
        top_left: p,
        bottom_right: b_right,
    };
}
