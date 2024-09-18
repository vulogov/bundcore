# Introduction

The ```bundcore``` Rust crate is a library that implements a basic API for multi-stack virtual machine functionality and the BUND concatenative programming language. This crate does not implement any VM or parser logic, but it provides a convenient interface for feeding source code to the parser and executing it on the VM.

# Show me the code!

First, let's define a static string containing a classic HelloWorld program written in Bund language

```rust
const TEST2: &str = r#"
//
// This is BUND Hello World program
//
"Hello world!" println
"#;
```

Then, let's parse and run this code

```rust
let mut bc = Bund::new();
bc.eval(TEST2).expect("Fail to parse BUND program");
```

Both, parser and run-time error returned as one of the outcomes of this call. And output of this Bund::eval call is:

```
---- tests::test_parse_hello_world stdout ----
Hello world!
```

## What is API ?

Function name | Description |
|---|---|
| Bund::new() | Create and initialize interface to Bund parser and VM |
| Bund::eval() | Evaluate the code passed as string and return a reference to Bund object or Error |
| Bund::run() | Evaluate the code passed as string and return ether value stored in Workbench or if Workbench is empty, stored on top of the stack |
