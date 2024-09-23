const TEST1SO: &str = r#"
//
// This snippet is to dup function
//
"This snippet shall duplicate value on top of the stack" println
42.0 dup
"#;

const TEST2SO: &str = r#"
//
// This snippet is to dup function
//
"This snippet shall swap two values on top of the stack" println
42.0 41 swap
"#;


#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use rust_dynamic::value::Value;
    use bundcore::bundcore::Bund;


    #[test]
    fn test_run_test_stackops_dup() {
        let mut bc = Bund::new();
        bc.eval(TEST1SO).expect("Fail to parse BUND program");
        assert_eq!(bc.vm.stack.current_stack_len(), 2);
    }

    #[test]
    fn test_run_test_stackops_swap() {
        let mut bc = Bund::new();
        let val = bc.run(TEST2SO).expect("Fail to parse BUND program");
        assert_eq!(val.expect("Expecting FLOAT").cast_float().unwrap(), 42.0 as f64);
    }

}
