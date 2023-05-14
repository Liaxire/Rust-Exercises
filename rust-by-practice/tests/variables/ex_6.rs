pub fn ex_6() -> i32 {
    let mut _x: i32 = 1;
    _x = 7;
    // Shadowing and re-binding
    let _x = _x; 


    let _y = 4;
    // Shadowing
    let _y = "I can also be bound to text!"; 

    println!("Success!");
    1
}