mod p1_hello_world;  // https://doc.rust-lang.org/stable/rust-by-example/hello.html
mod p2_primitives;  //https://doc.rust-lang.org/stable/rust-by-example/primitives.html


fn main() {
    // part_1_hello_world();
    part_2_primitives();
}

#[allow(dead_code)]
fn part_1_hello_world() {
    p1_hello_world::hello_world::hello_world();
    p1_hello_world::formatted_print_mod::formated_print();
    p1_hello_world::debug_mod::debug_examples();
    p1_hello_world::display_mod::display_examples();
    p1_hello_world::list_print_mod::print_list();
    p1_hello_world::formatting_mod::formatting_example_city();
    p1_hello_world::formatting_mod::formatting_example_color();
}

fn part_2_primitives(){
    p2_primitives::literals_and_operators::main();
    p2_primitives::tuples_mod::main();
    p2_primitives::arrays_and_slices::main();
}