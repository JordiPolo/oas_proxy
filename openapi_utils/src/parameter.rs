use openapiv3::*;
use crate::reference::ParameterSchemaOrContentExt;
use crate::schema::SchemaExt;

/// Parameter methods
pub trait ParameterDataExt {
    /// Returns the type of the schema for this parameter
    /// Panics for oneOf, anyOf, allOf
    fn get_type(&self) -> &Type;
}

impl ParameterDataExt for ParameterData {
    fn get_type(&self) -> &Type {
        self.format.item().get_type()
    }
}

/// Parameter contains most interesting things in the parameter_data,
/// convenience methods to access inside that.
pub trait ParameterExt {
    /// Returns "query", "header", "path" or "cookie" depending on
    /// where the parameter lives in
    fn location_string(&self) -> String;

    /// borrows the internal parameter data
    fn parameter_data(&self) -> &ParameterData;

    /// mutably borrows the internal parameter data
    fn parameter_data_mut(&mut self) -> &mut ParameterData;

    /// Returns the name of the parameter
    fn name(&self) -> &str;
}

impl ParameterExt for Parameter {
    /// Name inside the data of the parameter
    fn name(&self) -> &str {
        &self.parameter_data().name
    }

    /// Returns a string representing the enum of the parameter
    /// Parameter::Query becomes "query".
    fn location_string(&self) -> String {
        match self {
            Parameter::Query { .. } => "query".to_string(),
            Parameter::Header { .. } => "header".to_string(),
            Parameter::Path { .. } => "path".to_string(),
            Parameter::Cookie { .. } => "cookie".to_string(),
        }
    }

    /// Convenience method to access the internal parameter data
    /// independent from the kind of parameter we are using.
    fn parameter_data(&self) -> &ParameterData {
        match self {
            Parameter::Query { parameter_data, .. } => parameter_data,
            Parameter::Header { parameter_data, .. } => parameter_data,
            Parameter::Path { parameter_data, .. } => parameter_data,
            Parameter::Cookie { parameter_data, .. } => parameter_data,
        }
    }

    /// Convenience method to access the internal parameter data
    /// independent from the kind of paramete we are using, mutable context.
    fn parameter_data_mut(&mut self) -> &mut ParameterData {
        match self {
            Parameter::Query { parameter_data, .. } => parameter_data,
            Parameter::Header { parameter_data, .. } => parameter_data,
            Parameter::Path { parameter_data, .. } => parameter_data,
            Parameter::Cookie { parameter_data, .. } => parameter_data,
        }
    }
}
