mod ex_1;
mod ex_2;
mod ex_3;
mod ex_4;
mod ex_5;
mod ex_6;
mod ex_7;
mod ex_8;
mod ex_9;

#[cfg(test)]
mod variables {
    use super::*;
    #[test]
    fn ex_1() {
        assert_eq!(ex_1::ex_1(), 5);
    }
    #[test]
    fn ex_2() {
        assert_eq!(ex_2::ex_2(), 3);
    }
    #[test]
    fn ex_3() {
        let correct : String = "The value of x is 10 and value of y is 5".to_string();
        assert_eq!(ex_3::ex_3(), (correct.clone(), correct));
    }
    #[test]
    fn ex_4() {
        let correct : String = "hello, world".to_string();
        assert_eq!(ex_4::ex_4(), correct);
    }
    #[test]
    fn ex_5() {
        assert_eq!(ex_5::ex_5(), 42);
    }
    #[test]
    fn ex_6() {
        assert_eq!(ex_6::ex_6(), 1);
    }
    #[test]
    fn ex_7() {
        assert_eq!(ex_7::ex_7_1(), 1);
        assert_eq!(ex_7::ex_7_2(), 1);
    }
    #[test]
    fn ex_8() {
        assert_eq!(ex_8::ex_8(), 1);
    }
    #[test]
    fn ex_9() {
        assert_eq!(ex_9::ex_9(), 1);
    }

}