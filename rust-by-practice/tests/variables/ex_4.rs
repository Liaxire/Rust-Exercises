pub fn ex_4() -> String {
    let x = define_x();
    format!("{}, world", x)
}

fn define_x() -> String{
    let x = "hello".to_string();
    x
}