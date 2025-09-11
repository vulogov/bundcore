extern crate log;
use easy_error::{Error, bail};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

pub mod bundcore;
pub mod bundcore_eval;
pub mod bundcore_run;
pub mod bundcore_display;
pub mod bundcore_init_stdlib;

use crate::bundcore::Bund;

pub type BundInitFn  = fn(&mut Bund) -> Result<&mut Bund, Error>;

lazy_static! {
    pub static ref STDLIB: Mutex<HashMap<String, BundInitFn>> = {
        let s: Mutex<HashMap<String, BundInitFn>> = Mutex::new(HashMap::new());
        s
    };
}

pub fn add_stdlib<N: AsRef<str> + ToString + std::fmt::Display>(name: N, fun: BundInitFn) -> Result<(), Error> {
    let mut stdlib = match STDLIB.lock() {
        Ok(stdlib) => stdlib,
        Err(err) => {
            log::error!("Error locking standard library handler");
            bail!("{}", err);
        }
    };
    let _ = stdlib.insert(name.to_string(), fun);
    drop(stdlib);
    Ok(())
}

pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string().clone()
}
