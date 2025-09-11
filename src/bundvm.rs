extern crate log;

use nanoid::nanoid;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use easy_error::{Error, bail};

use crate::bundcore;

lazy_static! {
    pub static ref BOOTSTRAP: Mutex<HashMap<String, String>> = {
        let s: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
        s
    };
}

#[derive(Clone)]
pub struct BundVM {
    pub id:             String,
    pub adam:           bundcore::Bund,
}

impl BundVM {
    fn init() -> Self {
        let vmid = nanoid!();
        Self {
            id:             vmid,
            adam:           bundcore::Bund::new(),
        }
    }

    pub fn new() -> Self {
        let res = BundVM::init();
        res
    }
}

pub fn add_bootstrap<N: AsRef<str> + ToString + std::fmt::Display>(name: N, script: String) -> Result<(), Error> {
    let mut bs = match BOOTSTRAP.lock() {
        Ok(bs) => bs,
        Err(err) => {
            log::error!("Error locking bootstrap handler");
            bail!("{}", err);
        }
    };
    let _ = bs.insert(name.to_string(), script);
    drop(bs);
    Ok(())
}
