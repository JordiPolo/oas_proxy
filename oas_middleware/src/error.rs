use crate::request::Attribute;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum E {
    #[error("The path `{0}` is not described in the OpenAPI file.")]
    PathError(String),

    #[error("The method `{0}` is not described in the OpenAPI file.")]
    MethodError(String),

    #[error("The parameter `{0}` is not described in the OpenAPI file.")]
    ParamError(String),

    #[error("The contract specifies `{param_name}` as {type_name} but got `{param_value}`.")]
    TypeError {
        type_name: String,
        param_name: String,
        param_value: String,
    },

    #[error("Type {0} not supported by the proxy. Fix me! ")]
    TypeNotsupported(String),

    #[error("{0}. Fix me! ")]
    FunctionalityNotsupported(String),

    #[error("The contract specifies `{param_name}` to have a {limit_name} of {limit_value} but got {param_value}.")]
    ValueLimit {
        param_name: String,
        limit_name: String,
        limit_value: String,
        param_value: String,
    }, // #[error("unknown data store error")]
       // Unknown
}

pub fn unsupported(text: &str) -> E {
    E::FunctionalityNotsupported(text.to_string())
}

pub fn type_error(type_name: &str, param: &Attribute) -> E {
    E::TypeError {
        type_name: type_name.to_string(),
        param_name: param.name.to_string(),
        param_value: param.value.to_string(),
    }
}

pub fn minimum_error(limit_value: &str, param: &Attribute) -> E {
    E::ValueLimit {
        limit_name: "minimum".to_string(),
        limit_value: limit_value.to_string(),
        param_name: param.name.to_string(),
        param_value: param.value.to_string(),
    }
}

pub fn maximum_error(limit_value: &str, param: &Attribute) -> E {
    E::ValueLimit {
        limit_name: "maximum".to_string(),
        limit_value: limit_value.to_string(),
        param_name: param.name.to_string(),
        param_value: param.value.to_string(),
    }
}
