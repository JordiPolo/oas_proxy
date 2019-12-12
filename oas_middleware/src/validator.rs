use log::debug;
use openapiv3::*;
use anyhow::{Context, Result};
use openapi_utils::{ParameterExt, ParameterDataExt, ReferenceOrExt};

use crate::check_type;
use crate::error::E;
use crate::parts::OpenAPIParts;
use crate::request::{Attribute, Params, RequestParts};
use crate::spec_utils;


pub fn validate(openapi_parts: &mut OpenAPIParts, request_parts: &RequestParts) -> Result<()> {
    let mut operation = &mut openapi_parts.operation;

    validate_variables(&request_parts.path_variables, &mut operation)
        .context("Failure in a path variable.")?;

    validate_variables(&request_parts.query_variables, &mut operation)
        .context("Failure in a query parameter.")?;

    Ok(())
}

fn validate_variables(variables: &Params, operation: &mut Operation) -> Result<()> {
    variables
        .iter()
        .map(|variable| {
            let param = find_param(operation, &variable.name)?;
            check_format(param, variable)
        })
        .collect()
}

fn find_param<'a>(operation: &'a mut Operation, param_name: &str) -> Result<&'a ParameterData> {
    debug!("Searching for parameter {}", param_name);
    let mutable_params: &mut Vec<ReferenceOr<Parameter>> = operation.parameters.as_mut();
    //operation.responses.responses
    let a: StatusCode = StatusCode::Code(200);

    for parameter2 in mutable_params {
        let parameter: &mut ReferenceOr<Parameter> = parameter2;
        let param = parameter.to_item_mut();
        let mut param_data = param.parameter_data_mut();
        if param_data.name == param_name {
            debug!("Used! {}", param_name);
            param_data.description = Some("1".to_string());
            spec_utils::used(&mut param_data.description);
            return Ok(param_data);
        }
    }
    Err(E::ParamError(param_name.to_string()))?
}


fn check_format(param: &ParameterData, request_param_data: &Attribute) -> Result<()> {
    debug!("Checking parameter {:?}", request_param_data);
    // TODO: Why this works? Does it work?
    Ok(check_type::check_type(param.get_type(), request_param_data)?)
}
