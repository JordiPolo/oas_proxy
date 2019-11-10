#[macro_use]
extern crate log;

pub use dereferer::{deref, deref_all, deref_mut, deref_own};
pub use error::DerefError;

mod dereferer;
mod error;
