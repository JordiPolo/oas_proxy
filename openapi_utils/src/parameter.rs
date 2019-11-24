use openapiv3::*;

pub trait ParameterDataExt {
    fn get_type(&self) -> &Type;
}

impl ParameterDataExt for ParameterData {
    fn get_type(&self) -> &Type {
        match &self.format {
            ParameterSchemaOrContent::Schema(reference) => match reference {
                ReferenceOr::Reference { reference: _ } => {
                    unimplemented!("References inside schemas are not supported")
                }
                ReferenceOr::Item(item) => match &item.schema_kind {
                    SchemaKind::Type(schema_type) => schema_type,
                    SchemaKind::OneOf { .. } => unimplemented!("OneOf not supported"),
                    SchemaKind::AnyOf { .. } => unimplemented!("AnyOf not supported"),
                    SchemaKind::AllOf { .. } => unimplemented!("AllOf not supported"),
                    SchemaKind::Any(_) => unimplemented!("Any not supported"),
                },
            },
            ParameterSchemaOrContent::Content(_content) => {
                unimplemented!("Not quite understand this one")
            }
        }
    }
}

pub trait ParameterExt {
    fn location_string(&self) -> String;
    fn to_parameter_data(&self) -> &ParameterData;
    fn to_parameter_data_mut(&mut self) -> &mut ParameterData;
}

impl ParameterExt for Parameter {
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
    fn to_parameter_data(&self) -> &ParameterData {
        match self {
            Parameter::Query { parameter_data, .. } => parameter_data,
            Parameter::Header { parameter_data, .. } => parameter_data,
            Parameter::Path { parameter_data, .. } => parameter_data,
            Parameter::Cookie { parameter_data, .. } => parameter_data,
        }
    }

    /// Convenience method to access the internal parameter data
    /// independent from the kind of paramete we are using, mutable context.
    fn to_parameter_data_mut(&mut self) -> &mut ParameterData {
        match self {
            Parameter::Query { parameter_data, .. } => parameter_data,
            Parameter::Header { parameter_data, .. } => parameter_data,
            Parameter::Path { parameter_data, .. } => parameter_data,
            Parameter::Cookie { parameter_data, .. } => parameter_data,
        }
    }
}
