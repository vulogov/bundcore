const TEST1LOGIC: &str = r#"
//
// This snippet is to test of compare function
//
"This snippet shall leave TRUE on stack" println
1 0 <
"#;

const TEST2LOGIC: &str = r#"
//
// This snippet is to test of IF function
//
"This snippet shall leave 42 on stack" println
1 0 < { 42 } if
"#;


#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use bundcore::bundcore::Bund;


    #[test]
    fn test_run_test_compare1() {
        let mut bc = Bund::new();
        let val = bc.run(TEST1LOGIC).expect("Fail to parse BUND program");
        assert_eq!(val.expect("Expecting value").cast_bool().unwrap(), true);
    }

    #[test]
    fn test_run_test_if1() {
        let mut bc = Bund::new();
        let val = bc.run(TEST2LOGIC).expect("Fail to parse BUND program");
        assert_eq!(val.expect("Expecting value").cast_int().unwrap(), 42 as i64);
    }

}
