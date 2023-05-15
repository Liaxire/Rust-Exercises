pub fn ex_4() -> bool {
    let f = true;
    let t = true || false;
    assert_eq!(t, f);

    true
}