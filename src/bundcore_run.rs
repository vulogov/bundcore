use crate::bundcore::Bund;
use rust_dynamic::value::Value;
use easy_error::{Error, bail};

impl Bund {
    pub fn run<N: AsRef<str> + ToString + std::fmt::Display>(&mut self, source: N) -> Result<Option<Value>, Error> {
        match self.eval(&source) {
            Ok(_) => {
                match self.vm.stack.pull_from_workbench() {
                    Some(value) => {
                        return Ok(Some(value))
                    }
                    None => {
                        return Ok(self.vm.stack.pull())
                    }
                }
            }
            Err(err) => {
                bail!("{}", err);
            }
        }
    }
}
