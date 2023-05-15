use std::mem::size_of_val;
pub fn ex_6() -> bool {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!");
    true
}
