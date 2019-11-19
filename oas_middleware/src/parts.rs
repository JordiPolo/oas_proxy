use crate::spec_utils;
use anyhow::Result;
use openapiv3::*;

#[derive(Debug)]
pub struct OpenAPIParts<'a> {
    pub operation: &'a mut Operation,
}

impl<'a> OpenAPIParts<'a> {
    pub fn new(
        path: &'a mut PathItem,
        request: &hyper::Request<hyper::Body>,
    ) -> Result<OpenAPIParts<'a>> {
        let mut operation = spec_utils::path_to_operation(path, &request.method())?;
        spec_utils::used(&mut operation.description);
        Ok(OpenAPIParts { operation })
    }
}
