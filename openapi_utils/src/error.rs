use std::fmt;

/// This error happens when it is not possible to dereference
/// the full contents of the Openapi file.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DerefError {
    /// Parameter could not be dereferenced.
    ReferenceError {
        /// The name of the refrenced element that failed
        name: String
    },
}

impl ::std::error::Error for DerefError {
    fn description(&self) -> &str {
        "It was not possible to resolve all references in the file."
    }
}

impl fmt::Display for DerefError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DerefError::ReferenceError { ref name } => {
                write!(f, "The reference {} could not be dereferenced", name)
            }
        }
    }
}
