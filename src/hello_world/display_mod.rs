use std::fmt;

// controlled struct that will derive from Debug.
// will use it to compare it against custom created Display format
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement Diplay for MinMax
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

// Do the same thing to point2D:
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

pub fn display_examples() {
    let minmax = MinMax(0, 14);

    println!("Comparing structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);
}
