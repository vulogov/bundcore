const TEST1T: &str = r#"
//
// This snippet is to dup function
//
"This snippet shall test concat_with_spaces and produce 'Hello 3.14 true' output" println nl
text "Hello" , 3.14 , true , println
"#;

const TEST2T: &str = r#"
//
// This snippet is to dup function
//
"This snippet shall test string.upper function" println nl
"Hello" string.upper
"#;


#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use bundcore::bundcore::Bund;


    #[test]
    fn test_run_test_text_concat_with_space() {
        let mut bc = Bund::new();
        bc.eval(TEST1T).expect("Fail to parse BUND program");
    }

    #[test]
    fn test_run_test_text_string_upper() {
        let mut bc = Bund::new();
        let val = bc.run(TEST2T).expect("Fail to parse BUND program");
        assert_eq!(val.expect("Expecting Value").cast_string().unwrap(), "HELLO");
    }

}
