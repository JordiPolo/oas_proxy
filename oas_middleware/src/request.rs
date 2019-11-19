use hyper::body::Body;
use regex::Regex;


// Body
use futures::stream::Stream;
use futures::future::Future;
use hyper::Chunk;
use serde_json::{Value};


#[derive(Debug)]
pub struct RequestParts {
    pub path_variables: Vec<Attribute>,
    pub query_variables: Vec<Attribute>,
    pub body: Option<Value>,
}

#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}
impl Attribute {
    fn new(name: &str, value: &str) -> Attribute {
        Attribute {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

pub type Params = Vec<Attribute>;


impl RequestParts {
    pub fn new<'a>(regex: &Regex, request: &hyper::Request<hyper::Body>) -> RequestParts {
        let path_variables = path_variables(regex, &request.uri().path());
        let query_variables = query_variables(&request.uri().query());
       // let body = body_variables(request.into_body());
        RequestParts {
            path_variables,
            query_variables,
            body:None,
        }
    }
}


/// Returns a list of query params from a string
/// # Examples
///
/// ```
///
/// let input = Some("user=me&role=root");
/// let output = Some(vec![Attribute::new("user","me"), Attribute::new("role", "root")]);
/// assert_eq!(query_variables(&input), &output);
/// ```
///
/// ```
/// assert_eq!(query_variables(&None), &None);
/// ```
fn query_variables(q: &Option<&str>) -> Params {
    match q {
        None => Vec::new(),
        Some(query) => query
            .split('&')
            // Use flat_map to filter out all malformed pairs.
            // Using map would result in a Vec<Option<(&str, &str)>>
            .flat_map(|pair| {
                pair.find('=') // This returns an option, since '=' might not exist
                    .map(|idx| pair.split_at(idx)) // split it into (&str, &str)
                    .map(|(a, b)| Attribute::new(a, &b[1..])) // Since split includes the '=' char, remove it.
            })
            .collect(),
    }
}

/// Returns a list of path params from a string and a regex
/// # Examples
///
///
/// let path = "/v1/users/username/action";
/// let regex = ...
/// let output = ...
/// assert_eq!(path_variables(&regex, path), output)
///
///
fn path_variables(regex: &Regex, path: &str) -> Params {
    let captures = regex.captures(&path).unwrap();
    regex
        .capture_names() // None indicate unnamed captures, like the one for the whole string.
        .filter_map(|n| n.map(|name| Attribute::new(name, captures.name(&name).unwrap().as_str())))
        .collect()
}



fn body_variables(body: Body) -> Option<Value> {
    let data = body.concat2().wait().unwrap();
    let v: Value = serde_json::from_slice(data.into_bytes().as_ref()).unwrap();
    Some(v)
    // match v {
    //     Value::Array(array) => None,
    //     Value::Object(a_map) => None,
    //     _ => unimplemented!("BUG: This Json format is not implemented!")
    // }
    // None
}
