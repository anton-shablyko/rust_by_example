use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    lat: f32,
    lon: f32
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { "N" } else { "S" };
        let lon_c = if self.lon >= 0.0 { "E" } else { "W" };

        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name,
               self.lat.abs(), lat_c,
               self.lon.abs(), lon_c
        )
    }
}

pub fn formatting_example_city(){
    for city in [
        City {name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", city);
    }
}

// Do the same thing but for color
/*
Expected result:
RGB (128, 255, 90) 0x8FF05A
RGB (0, 3, 254) 0x0003FE
RGB (0, 0, 0) 0x000000
*/

struct Color {
    red: u16,
    green: u16,
    blue: u16
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let red_hex = format!("{:X}", self.red);
        let green_hex = format!("{:X}", self.green);
        let blue_hex = format!("{:X}", self.blue);

        write!{f, "RGB ({r}, {g}, {b}) 0x{r_h:0>2}{g_h:0>2}{b_h:0>2}",
               r=self.red, g=self.green, b=self.blue,
               r_h=red_hex, g_h=green_hex, b_h=blue_hex}
    }
}

pub fn formatting_example_color(){
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter(){
        println!("{}", color);
    }
}

