mod ex_1;
mod ex_2;
mod ex_3;

#[cfg(test)]
mod statements_expressions {
    use super::*;
    #[test]
    fn ex_1() {
        assert!(ex_1::ex_1_1());
        assert!(ex_1::ex_1_2());
    }
    #[test]
    fn ex_2() {
        assert!(ex_2::ex_2());
    }
    #[test]
    fn ex_3() {
        assert!(ex_3::ex_3());
    }

}