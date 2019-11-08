#[macro_use]
extern crate log;

pub use dereferer::{deref_all, read, read_and_deref_all, deref, deref_mut};
pub use error::DerefError;

mod dereferer;
mod error;
