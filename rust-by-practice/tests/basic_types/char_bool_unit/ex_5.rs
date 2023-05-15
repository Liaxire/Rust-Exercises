#![allow(dead_code)]
#![allow(unused_variables)]
pub fn ex_5() -> bool {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());

    println!("Success!");
    true
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}
// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}
