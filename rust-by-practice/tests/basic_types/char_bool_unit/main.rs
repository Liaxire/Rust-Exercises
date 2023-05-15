mod ex_1;
mod ex_2;
mod ex_3;
mod ex_4;
mod ex_5;
mod ex_6;

#[cfg(test)]
mod char_bool_unit {
    use super::*;
    #[test]
    fn ex_1() {
        assert!(ex_1::ex_1());
    }
    #[test]
    fn ex_2() {
        assert!(ex_2::ex_2());
    }
    #[test]
    fn ex_3() {
        assert!(ex_3::ex_3());
    }
    #[test]
    fn ex_4() {
        assert!(ex_4::ex_4());
    }
    #[test]
    fn ex_5() {
        assert!(ex_5::ex_5());
    }
    #[test]
    fn ex_6() {
        assert!(ex_6::ex_6());
    }

}