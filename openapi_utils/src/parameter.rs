use openapiv3::*;

pub trait ParameterExt {
    fn location_string(&self) -> String;
    fn to_parameter_data(&self) -> &ParameterData;
    fn to_parameter_data_mut(&mut self) -> &mut ParameterData;
}

impl ParameterExt for Parameter {
    fn location_string(&self) -> String {
        match self {
            Parameter::Query { .. } => "query".to_string(),
            Parameter::Header { .. } => "header".to_string(),
            Parameter::Path { .. } => "path".to_string(),
            Parameter::Cookie { .. } => "cookie".to_string(),
        }
    }

    fn to_parameter_data(&self) -> &ParameterData {
        match self {
            Parameter::Query { parameter_data, .. } => parameter_data,
            Parameter::Header { parameter_data, .. } => parameter_data,
            Parameter::Path { parameter_data, .. } => parameter_data,
            Parameter::Cookie { parameter_data, .. } => parameter_data,
        }
    }

    fn to_parameter_data_mut(&mut self) -> &mut ParameterData {
        match self {
            Parameter::Query { parameter_data, .. } => parameter_data,
            Parameter::Header { parameter_data, .. } => parameter_data,
            Parameter::Path { parameter_data, .. } => parameter_data,
            Parameter::Cookie { parameter_data, .. } => parameter_data,
        }
    }
}
