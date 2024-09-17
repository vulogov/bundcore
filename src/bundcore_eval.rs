use rust_dynamic::types::*;
use crate::bundcore::*;
use bund_language_parser::bund_parse;
use easy_error::{Error, bail};

impl Bund {
    pub fn eval<N: AsRef<str> + ToString>(&mut self, value: N) -> Result<&mut Bund, Error> {
        let source = value.to_string();
        match bund_parse(&source) {
            Ok(words) => {
                for word in words {
                    match word.dt {
                        NONE => {
                            continue;
                        }
                        EXIT => {
                            break;
                        }
                        ERROR => {
                            match word.cast_error() {
                                Ok(error) => {
                                    bail!("Detected an Error posted on the stack: {:?}", error);
                                }
                                Err(err) => {
                                    bail!("Detected an Error posted on the stack, but extraction of error is failed: {}", err);
                                }
                            }
                        }
                        _ => {
                            match self.vm.apply(word.clone()) {
                                Ok(_) => {}
                                Err(err) => {
                                    bail!("Attempt to evaluate value {:?} returned error: {}", &word, err);
                                }
                            }
                        }
                    }
                }
            }
            Err(err) => {
                bail!("{}", err);
            }
        }
        Ok(self)
    }
}
