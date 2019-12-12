use openapiv3::*;
use crate::reference::ReferenceOrExt;

pub trait OperationExt {
    fn response(&self, status: u16) ->Option<&Response>;
}

impl OperationExt for Operation {
    fn response(&self, status: u16) ->Option<&Response> {
        let status_code = StatusCode::Code(status);
        self.responses.responses.get(&status_code).map(|ref_or_item| ref_or_item.to_item_ref())
    }
}
