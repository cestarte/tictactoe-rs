use crate::input::*;

#[test]
fn is_input_format_acceptable_test() {
    assert_eq!(false, is_input_format_acceptable(""));
    assert_eq!(false, is_input_format_acceptable("alksdjhf"));
    assert_eq!(false, is_input_format_acceptable("x,y"));
    assert_eq!(false, is_input_format_acceptable("1,,4"));
    assert_eq!(false, is_input_format_acceptable("4,1"));
    assert_eq!(true, is_input_format_acceptable("0,0"));
    assert_eq!(true, is_input_format_acceptable("3,3"));
}

#[test]
fn split_input_coordinate_test() {
    assert_eq!((1,2), split_input_coordinate("1,2"));
}
