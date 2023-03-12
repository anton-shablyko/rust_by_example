// Enum allows you to define a type by enumerating its possible variants

enum WebEvent {
    // Unit like enum
    PageLoad,
    PageUnload,
    // tuple like enum
    KeyPress(char),
    Paste(String),
    // c-like structures
    Click { x: i64, y: i64 },
}

// A function that takes WebEvent ^ as an argument and returns nothing

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page load"),
        WebEvent::PageUnload => println!("page unload"),

        WebEvent::KeyPress(c) => println!("Pressed '{}'.", c),
        WebEvent::Paste(s) => println!("Pasted '{}'.", s),

        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}", x, y);
        }
    }
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Substract,
}

//Create a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

// The most common place you'll see this is in impl blocks using the Self alias.
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Substract => x - y,
        }
    }
}

pub fn main() {
    let pressed = WebEvent::KeyPress('x');
    // to_owned() creates an owned 'String' from a string slice
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}
