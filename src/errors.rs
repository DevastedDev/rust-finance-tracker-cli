use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum FileErrors {
    FileNotFound,
    FileParsingFailed,
}

#[derive(Debug)]
pub enum CommonError {
    RemovingTransactionFailed,
    ToPrettyStringFailed,
    AddingTransactionFailed,
}
impl Error for FileErrors {}
impl Error for CommonError {}

impl Display for FileErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::FileNotFound => "File not found",
            Self::FileParsingFailed => "File parsing failed",
        };
        write!(f, "Error: {message}")
    }
}
impl Display for CommonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::RemovingTransactionFailed => "Failed To Remove Transaction",
            Self::AddingTransactionFailed => "Failed To Add Transaction",
            Self::ToPrettyStringFailed => "Failed To Format to Pretty String"
        };
        write!(f, "Error: {message}")
    }
}
