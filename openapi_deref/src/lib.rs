#[macro_use]
extern crate log;

pub use dereferer::{deref_all, deref, deref_mut, deref_own};
pub use error::DerefError;

mod dereferer;
mod error;
