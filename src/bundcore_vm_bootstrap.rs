extern crate log;
use crate::bundvm::*;
use easy_error::{Error, bail};

impl BundVM {
    pub fn run_bootstrap(&mut self, name: String, script: String) -> Result<&mut BundVM, Error> {
        match self.adam.eval(script) {
            Ok(_) => {
                log::debug!("BUNDVM bootstrapped {} for {}", &name, self.id);
            },
            Err(err) => {
                log::error!("BUNDVM bootstrap {} initialized failed for {}", &name, self.id);
                bail!("BOOTSTRAP failed: {}", err);
            }
        }
        Ok(self)
    }

    pub fn bootstrap(&mut self) -> Result<&mut BundVM, Error> {
        let bs = match BOOTSTRAP.lock() {
            Ok(bs) => bs,
            Err(err) => {
                bail!("Error locking BOOSTRAP handler for BundVM: {}", err);
            }
        };
        for name in bs.clone().into_keys().collect::<Vec<_>>() {
            let script = match bs.get(&name) {
                Some(script) => script,
                None => bail!("Error getting script for: {}", &name),
            };
            match self.run_bootstrap(name.clone(), script.to_string()) {
                Ok(_) => {},
                Err(err) => {
                    drop(bs);
                    bail!("Error running bootstrap {}: {}", name.clone(), err);
                }
            }
        }
        drop(bs);
        Ok(self)
    }
}
