// https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html
// This structure can't be printed with either `fmt::Display` or with `fmt::Debug`
struct UnPrintable(i32);

// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`
#[derive(Debug)]
struct DebugPrintable(i32);

pub fn debug_examples() {
    println!("{:?}", DebugPrintable(7));

    //fancy debug print
    println!("{:#?}", DebugPrintable(7));
}
