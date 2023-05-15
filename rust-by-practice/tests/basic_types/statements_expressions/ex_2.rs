pub fn ex_2() -> bool {
    let v = {let x = 3;
        x
    };
 
    assert!(v == 3);
 
    println!("Success!");
    true
 }
 