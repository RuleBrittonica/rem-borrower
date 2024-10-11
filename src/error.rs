use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum BorrowerError {
    BorrowerFailed,
}

impl fmt::Display for BorrowerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BorrowerError::BorrowerFailed => write!(f, "Borrower failed"),
        }
    }
}