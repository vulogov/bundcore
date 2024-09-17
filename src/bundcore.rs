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
        let res = Bund::init();
        res
    }
}
