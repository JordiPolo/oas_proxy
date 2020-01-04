use crate::reference::ReferenceOrExt;
use openapiv3::*;

/// Convenience methods for common cases
pub trait ResponseExt {
    /// Returns the Schema for this response if it responds with application/json
    fn json_schema(&self) -> Option<&Schema>;
}

impl ResponseExt for Response {
    /// Gets the response for a status code in the operation
    fn json_schema(&self) -> Option<&Schema> {
        self.content
            .get("application/json")
            .and_then(|media| media.schema.as_ref().map(|schema| schema.to_item_ref()))
    }
}
