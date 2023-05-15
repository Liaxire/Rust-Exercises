pub fn ex_3() -> bool {
    let s = sum(1 , 2);
    assert_eq!(s, 3);

    println!("Success!");
    true
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}