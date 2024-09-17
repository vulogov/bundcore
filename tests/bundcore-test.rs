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

}
