#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![deny(unused_imports)]
//#![deny(clippy::all)]
#![no_std]
extern crate alloc;

mod dereferer;
mod error;
mod operation;
mod parameter;
mod reference;
mod response;
mod schema;
mod server;
mod types;

pub use dereferer::SpecExt;
pub use error::DerefError;
pub use operation::OperationExt;
pub use parameter::{ParameterDataExt, ParameterExt};
pub use reference::ReferenceOrExt;
pub use response::ResponseExt;
pub use schema::SchemaExt;
pub use server::ServerExt;
pub use types::{NumberTypeExt, IntegerTypeExt, TypeExt};
