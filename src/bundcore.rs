extern crate log;

use nanoid::nanoid;
use rust_multistackvm::multistackvm::VM;


///
/// Principial structure provining interface to all funcitonality of BUND language interpreter and Virtual Machine
///
#[derive(Clone)]
pub struct Bund {
    pub id:             String,
    pub vm:             VM,
}

impl Bund {
    fn init() -> Self {
        let vmid = nanoid!();
        Self {
            id:             vmid,
            vm:             VM::new(),
        }
    }
    ///
    /// Create and initialize  Bund
    ///
    pub fn new() -> Self {
        let mut res = Bund::init();
        match res.init_lib() {
            Ok(_) => {},
            Err(err) => {
                log::error!("Error during init_stdlib: {}", err);
            }
        };
        res
    }
}
