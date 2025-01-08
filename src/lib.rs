pub mod bundcore;
pub mod bundcore_eval;
pub mod bundcore_run;

pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string().clone()
}
