extern crate log;
use crate::bundvm::*;
use crate::bundcore::Bund;
use easy_error::{Error, bail};

impl BundVM {
    pub fn ephemeral(&mut self) -> Result<Bund, Error> {
        let mut bund = Bund::new();
        let _ = match bund.init_lib() {
            Ok(_) => {},
            Err(err) => bail!("{}", err),
        };
        let _ = match self.bootstrap_instance(&mut bund) {
            Ok(_) => {},
            Err(err) => bail!("{}", err),
        };
        Ok(bund)
    }

}
