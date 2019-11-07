use openapiv3::*;

use crate::check_type;
use crate::error::unsupported;
use crate::error::E;
use crate::request::{Request, Attribute, Params, RequestBuilder};
use crate::spec_utils;
use openapi_deref::deref;

use anyhow::{Context, Result};

    pub fn validate(request: &mut Request) -> Result<()> {
        let mut operation = &mut request.operation;

        if let Some(variables) = &mut request.path_variables {
            validate_variables(operation, &variables)
                .context("Failure in a path variable.")?;
        }

        if let Some(variables) = &request.query_variables {
            validate_variables(&mut request.operation.clone(), &variables)
                .context("Failure in a query parameter.")?;
        }

        Ok(())
    }

    // pub fn new(spec: &'a OpenAPI) -> Self {
    //    // let request_builder = RequestBuilder::new(&spec);
    //     //   let used = UsedSpec { paths: Vec::new() };
    //     Validator {
    //         spec,
    //    //     request_builder,
    //         //        used,
    //     }
    // }

    fn validate_variables(operation: &mut Operation, variables: &Params) -> Result<()> {
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
            let mut parameter : &mut ReferenceOr<Parameter> = parameter2;
            let mut param = deref(parameter);
            let mut param_data = spec_utils::parameter_to_parameter_data(param);
            if param_data.name == param_name {
                error!("Used! {}", param_name);
                param_data.description =  Some("1".to_string());
                //spec_utils::used(&mut param_data.description);
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
            ParameterSchemaOrContent::Content(_content) => unimplemented!("Not quite understand this one"),
        }
        Ok(())
    }



