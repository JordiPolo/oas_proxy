use serde::Serialize;
use std::collections::HashMap;

use crate::path_finder::PathFinder;
use crate::spec_utils;
use openapi_deref::to_item_ref;

#[derive(Serialize)]
struct UsedSpec {
    spec: HashMap<String, Vec<UsedMethod>>,
}

#[derive(Serialize)]
struct UsedMethod {
    used: bool,
    method: String,
    parameters: Vec<UsedParam>,
    body: HashMap<String, UsedSchema>,
    responses: HashMap<String, UsedSchema>,
}

#[derive(Serialize)]
struct UsedSchema {
    used: bool,
    properties: Vec<UsedProperty>,
}

#[derive(Serialize)]
struct UsedProperty {
    used: bool,
    name: String,
}

#[derive(Serialize)]
struct UsedParam {
    used: bool,
    name: String,
    location: String,
}

pub fn render_report(builder: &PathFinder) -> String {
    serde_json::to_string(&usage_summary(&builder))
        .expect("Not possible to render usage report. This is a bug.")
}

fn usage_summary(builder: &PathFinder) -> UsedSpec {
    let mut spec = HashMap::new();
    //let mut paths = Vec::new();
    for path_match in &builder.path_matches {
        //let path = path_match.path.clone();
        let mut methods = Vec::new();
        for (name, operation) in spec_utils::operation_list(&path_match.path) {
            let mut params = Vec::new();
            for parameter in &operation.parameters {
                //  parameter_location
                let param = to_item_ref(&parameter);
                let param_data = spec_utils::parameter_to_parameter_data(param);
                let used = UsedParam {
                    used: is_used(&param_data.description),
                    name: param_data.name.clone(),
                    location: spec_utils::parameter_location(param),
                };
                params.push(used);
            }
            methods.push(UsedMethod {
                used: is_used(&operation.description),
                method: name.to_string(),
                parameters: params,
                body: HashMap::new(),
                responses: HashMap::new(),
            });
        }
        spec.insert(path_match.regex.to_string(), methods);
    }
    UsedSpec { spec }
}

fn is_used(description: &Option<String>) -> bool {
    *description == Some("1".to_string())
}
