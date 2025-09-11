extern crate log;
use crate::bundcore::*;
use crate::BundInitFn;
use easy_error::{Error, bail};

impl Bund {
    pub fn init_stdlib<N: AsRef<str> + ToString + std::fmt::Display>(&mut self, name: N, fun: BundInitFn) -> Result<&mut Bund, Error> {
        match fun(self) {
            Ok(_) => {
                log::debug!("BUND standard library {} initialized for {}", &name, self.id);
            },
            Err(err) => {
                log::error!("BUND standard library {} initialized failed for {}", &name, self.id);
                bail!("STDLIB init failed: {}", err);
            }
        }
        Ok(self)
    }
}
