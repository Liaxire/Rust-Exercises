use std::mem::size_of_val;
pub fn ex_1() -> bool {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4); 

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4); 
    true
}