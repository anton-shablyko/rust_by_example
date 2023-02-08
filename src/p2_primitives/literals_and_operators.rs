
/// literals: `1`, `1.2`, `a`, "abc", `true` and the unit () type
/// like in python: 1_000_000 == 1000000

/// operators:
pub fn main(){
    //int addictions
    println!("1 + 2 = {}", 1u32 + 2);

    //int substractions
    println!("1 - 2 = {}", 1i32 - 2); // datatupe is important! i32 works u32 doesn't

    // boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);


    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    //todo: Start here ...

}