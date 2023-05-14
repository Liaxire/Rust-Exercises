mod ex_1;
mod ex_2;
mod ex_3;
mod ex_4;
mod ex_5;
mod ex_6;
mod ex_7;
mod ex_8;
mod ex_9;
mod ex_10;
mod ex_11;

#[cfg(test)]
mod numbers {
    use super::*;
    #[test]
    fn ex_1() {
        assert_eq!(ex_1::ex_1(), 1);
    }
    #[test]
    fn ex_2() {
        assert_eq!(ex_2::ex_2(), 38);
    }
    #[test]
    fn ex_3() {
        assert_eq!("i32".to_string(), ex_3::ex_3());
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
    #[test]
    fn ex_7() {
        assert!(ex_7::ex_7());
    }
    #[test]
    fn ex_8() {
        assert!(ex_8::ex_8());
    }
    #[test]
    fn ex_9() {
        assert!(ex_9::ex_9());
    }
    #[test]
    fn ex_10() {
        assert!(ex_10::ex_10());
    }
    #[test]
    fn ex_11() {
        assert!(ex_11::ex_11());
    }

}