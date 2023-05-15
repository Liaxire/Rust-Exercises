// Solve it in two ways
// DON'T let `println!` works
#![allow(unreachable_code)]
pub fn ex_3() -> bool {
    never_return();

    println!("Failed!");
    false
}

fn never_return() -> ! {
    // Implement this function, don't modify the fn signatures
    panic!("Success!")
}