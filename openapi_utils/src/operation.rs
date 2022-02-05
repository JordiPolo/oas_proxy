use alloc::vec::Vec;
use openapiv3::*;

/// Extension methods for Operation
pub trait OperationExt {
    /// For this operation object which contains multiple responses
    /// returns the response which will be returned in the service
    fn response(&self, status: u16) -> Option<&Response>;

    /// Returns all the required parameters of this operation
    fn required_parameters(&self) -> Vec<&Parameter>;

    /// Returns all the optional parameters of this operation
    fn optional_parameters(&self) -> Vec<&Parameter>;
}

impl OperationExt for Operation {
    /// Gets the response for a status code in the operation
    fn response(&self, status: u16) -> Option<&Response> {
        let status_code = StatusCode::Code(status);
        self.responses
            .responses
            .get(&status_code)
            .map(|ref_or_item| ref_or_item.as_item()).flatten()
    }

    fn required_parameters(&self) -> Vec<&Parameter> {
        self.parameters
            .iter()
            .map(|p| p.as_item())
            .flatten()
            .filter(|p| p.parameter_data_ref().required)
            .collect()
    }

    fn optional_parameters(&self) -> Vec<&Parameter> {
        self.parameters
            .iter()
            .map(|p| p.as_item())
            .flatten()
            .filter(|p| !p.parameter_data_ref().required)
            .collect()
    }
}
