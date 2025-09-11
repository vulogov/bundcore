use std::fmt;

use crate::bundcore::Bund;

impl fmt::Display for Bund {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BUND ID={}", self.id)
    }
}

impl fmt::Debug for Bund {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BUND ID={}", self.id)
    }
}
