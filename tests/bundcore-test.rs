const TEST1: &str = r#"
//
//
//
"#;

const TEST2: &str = r#"
//
// This is BUND Hello World program
//
"Hello world!" println
"#;

const TEST3: &str = r#"
//
// This is BUND Hello World program
//
42 .
"#;

const TEST4: &str = r#"
//
// This is BUND Hello World program
//
"Test value" 41 42
"#;

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use bundcore::bundcore::Bund;

    #[test]
    fn test_parse_comments() {
        let mut bc = Bund::new();
        bc.eval(TEST1).expect("Fail to parse BUND program");
    }

    #[test]
    fn test_parse_hello_world() {
        let mut bc = Bund::new();
        bc.eval(TEST2).expect("Fail to parse BUND program");
    }

    #[test]
    fn test_run1_42() {
        let mut bc = Bund::new();
        let val = bc.run(TEST3).expect("Fail to parse BUND program");
        assert_eq!(val.expect("Expecting value").cast_int().unwrap(), 42 as i64);
    }

    #[test]
    fn test_run2_42() {
        let mut bc = Bund::new();
        let val = bc.run(TEST4).expect("Fail to parse BUND program");
        assert_eq!(val.expect("Expecting value").cast_int().unwrap(), 42 as i64);
    }

}
