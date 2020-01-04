use openapiv3::*;
use crate::reference::ReferenceOrExt;

/// Extends an openapi spec with a method to dereference all its contents
pub trait OperationExt {
    /// For this operation object which contains multiple responses
    /// returns the response which will be returned in the service
    fn response(&self, status: u16) ->Option<&Response>;
}

impl OperationExt for Operation {
    /// Gets the response for a status code in the operation
    fn response(&self, status: u16) ->Option<&Response> {
        let status_code = StatusCode::Code(status);
        self.responses.responses.get(&status_code).map(|ref_or_item| ref_or_item.to_item_ref())
    }
}
