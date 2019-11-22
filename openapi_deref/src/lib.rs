mod dereferer;
mod error;

pub use dereferer::{to_item, to_item_ref, to_item_mut, deref_all};
pub use error::DerefError;
