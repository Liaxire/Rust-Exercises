pub fn ex_3() -> (String, String) {
    let x: i32 = 10;
    let y: i32 = 5;
    let s1: String;
    {
        s1 = format!("The value of x is {} and value of y is {}", x, y);
    }
    let s2: String = format!("The value of x is {} and value of y is {}", x, y);
    (s1, s2)
}