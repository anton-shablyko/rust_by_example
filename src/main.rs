mod hello_world;  // https://doc.rust-lang.org/stable/rust-by-example/hello.html
mod p2_primitives;  //https://doc.rust-lang.org/stable/rust-by-example/primitives.html


fn main() {
    // part_1_hello_world();
    part_2_primitives();
}

#[allow(dead_code)]
fn part_1_hello_world() {
    hello_world::hello_world::hello_world();
    hello_world::formatted_print_mod::formated_print();
    hello_world::debug_mod::debug_examples();
    hello_world::display_mod::display_examples();
    hello_world::list_print_mod::print_list();
    hello_world::formatting_mod::formatting_example_city();
    hello_world::formatting_mod::formatting_example_color();
}

fn part_2_primitives() {
    p2_primitives::literals_and_operators::main()
}