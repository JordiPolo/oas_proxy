use openapiv3::*;
use log::{debug};

use crate::check_type;
use crate::error::unsupported;
use crate::error::E;
use crate::request::{Attribute, Params, RequestParts};
use crate::parts::OpenAPIParts;
use crate::spec_utils;
use anyhow::{Context, Result};
use openapi_deref::deref_mut;

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

    for parameter2 in mutable_params {
        let parameter: &mut ReferenceOr<Parameter> = parameter2;
        let param = deref_mut(parameter);
        let mut param_data = spec_utils::parameter_to_parameter_data_mut(param);
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
    match &param.format {
        ParameterSchemaOrContent::Schema(reference) => match reference {
            ReferenceOr::Reference { reference: _ } => {
                Err(unsupported("References inside schemas are not supported"))?
            }
            ReferenceOr::Item(item) => match &item.schema_kind {
                SchemaKind::Type(schema_type) => {
                    check_type::check_type(schema_type, request_param_data)?
                }
                SchemaKind::OneOf { .. } => Err(unsupported("OneOf not supported"))?,
                SchemaKind::AnyOf { .. } => Err(unsupported("AnyOf not supported"))?,
                SchemaKind::AllOf { .. } => Err(unsupported("AllOf not supported"))?,
                SchemaKind::Any(_) => Err(unsupported("Any not supported"))?,
            },
        },
        ParameterSchemaOrContent::Content(_content) => {
            unimplemented!("Not quite understand this one")
        }
    }
    Ok(())
}
