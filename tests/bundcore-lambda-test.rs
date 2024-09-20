const TEST1L: &str = r#"
//
// This snippet is to test "!" or "execute" feature. This anonymous function shall be executed
//
{ "This is message from lambda that leaves 42 on stack" println 42 } !
"#;


const TEST2L: &str = r#"
//
// This is test of "if" feature
//
true {
    "This is message from lambda that leaves 42 on stack and it is executed if true is on the stack "
    println
    42
} if
"#;

const TEST3L: &str = r#"
//
// This is test of "loop" feature
//
[ 42 ] {
    "This is message from lambda that leaves 42 on stack and it is executed for all elements in loop "
    println
    .
} loop
"#;

const TEST4L: &str = r#"
//
// This is test of "loop" feature
//
true {
    "This is message from lambda that leaves 42 on stack and it is executed while TRUE is on top of stack "
    println     // We print message
    42          // We leave 42 on stack
    false       // We leave FALSE on stack that will be removed by "while"
} while
"#;

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use bundcore::bundcore::Bund;


    #[test]
    fn test_run_lambda_execute() {
        let mut bc = Bund::new();
        let val = bc.run(TEST1L).expect("Fail to parse BUND program");
        assert_eq!(val.expect("Expecting value").cast_int().unwrap(), 42 as i64);
    }

    #[test]
    fn test_run_lambda_if() {
        let mut bc = Bund::new();
        let val = bc.run(TEST2L).expect("Fail to parse BUND program");
        assert_eq!(val.expect("Expecting value").cast_int().unwrap(), 42 as i64);
    }

    #[test]
    fn test_run_lambda_loop() {
        let mut bc = Bund::new();
        let val = bc.run(TEST3L).expect("Fail to parse BUND program");
        assert_eq!(val.expect("Expecting value").cast_int().unwrap(), 42 as i64);
    }

    #[test]
    fn test_run_lambda_while() {
        let mut bc = Bund::new();
        let val = bc.run(TEST4L).expect("Fail to parse BUND program");
        assert_eq!(val.expect("Expecting value").cast_int().unwrap(), 42 as i64);
    }

}
