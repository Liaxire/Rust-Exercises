pub fn ex_5() -> bool {
    let v1 = 251_u8.wrapping_add(8);
    let v2 = i8::checked_add(119, 8).unwrap();
    println!("{},{}",v1,v2);
    true
 }
 