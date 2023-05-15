#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
pub fn ex_1_1() -> bool {
    let v = {
        let mut x = 1;
        x += 2
    };

    assert_eq!(v, ());

    println!("Success!");
    true
}

pub fn ex_1_2() -> bool {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);

    println!("Success!");
    true
}
