extern crate log;
use crate::bundcore::*;
use crate::{BundInitFn, STDLIB};
use easy_error::{Error, bail};

impl Bund {
    pub fn init_stdlib(&mut self, name: String, fun: BundInitFn) -> Result<&mut Bund, Error> {
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

    pub fn init_lib(&mut self) -> Result<&mut Bund, Error> {
        let stdlib = match STDLIB.lock() {
            Ok(stdlib) => stdlib,
            Err(err) => {
                bail!("Error locking STDLIB handler for Bund: {}", err);
            }
        };
        for name in stdlib.clone().into_keys().collect::<Vec<_>>() {
            let fun = match stdlib.get(&name) {
                Some(fun) => fun,
                None => bail!("Error getting init function for: {}", &name),
            };
            match self.init_stdlib(name.clone(), *fun) {
                Ok(_) => {},
                Err(err) => {
                    drop(stdlib);
                    bail!("Error initializing library {}: {}", name.clone(), err);
                }
            }
        }
        drop(stdlib);
        Ok(self)
    }
}
