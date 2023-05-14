#[allow(unused_variables)]
pub fn ex_7() -> bool {
    let x = 1_000.000_1; // ?
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    true
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}