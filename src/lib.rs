use easy_error::{Error};

pub mod bundcore;
pub mod bundcore_eval;
pub mod bundcore_run;
pub mod bundcore_display;
pub mod bundcore_init_stdlib;

use crate::bundcore::Bund;

pub type BundInitFn  = fn(&mut Bund) -> Result<&mut Bund, Error>;

pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string().clone()
}
