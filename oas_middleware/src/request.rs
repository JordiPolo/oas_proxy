use anyhow::Result;
use hyper::body::Body;
use hyper::Uri;
use openapiv3::*;
use regex::Regex;

use crate::error::E;
use crate::spec_utils;
use openapi_deref::deref_own;

#[derive(Default, Debug)]
pub struct RequestBuilder {
    pub path_matches: Vec<PathMatcher>,
}

#[derive(Debug)]
pub struct PathMatcher {
    pub regex: Regex,
    pub path: PathItem,
}

#[derive(Debug)]
pub struct Request<'a> {
    pub path_variables: Vec<Attribute>,
    pub query_variables: Vec<Attribute>,
    //    pub operation: Operation,
    pub operation: &'a mut Operation,
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
// use futures::stream::Stream;

// fn body_variables(body: &Body) -> Option<Params> {
//     let data = body.concat2().wait().unwrap().into_bytes();
//     None
// }

impl RequestBuilder {
    pub fn new(spec: OpenAPI) -> Self {
        RequestBuilder {
            path_matches: RequestBuilder::create_path_regexes(spec),
        }
    }

    pub fn build<'a>(&'a mut self, request: &hyper::Request<hyper::Body>) -> Result<Request> {
        let path = self.find_path(request.uri().path())?;
        let path_variables = path_variables(&path.regex, &request.uri().path());
        let query_variables = query_variables(&request.uri().query());
        let operation = spec_utils::path_to_operation(&mut path.path, &request.method())?;
        spec_utils::used(&mut operation.description);
        //        body_variables(request.body());
        Ok(Request {
            path_variables,
            query_variables,
            operation,
        })
    }

    fn find_path<'a>(&'a mut self, path: &str) -> Result<&'a mut PathMatcher, E> {
        // /users/<user_id> and /users/copy regexes would match /users/copy path.
        // We choose the most specific one, the one with minimum number of variable captures.
        self.path_matches
            .iter_mut()
            .filter(|path_match| path_match.regex.is_match(&path))
            .min_by_key(|path_match| path_match.regex.captures_len())
            .ok_or_else(|| E::PathError(path.to_string()))
    }

    ///
    /// # Examples
    ///
    ///
    /// let result = oas_middleware::validator::spec_path_to_regex_str("/study/{uuid}/test");
    /// assert_eq!(result, "^/study/(?P<uuid>.*)/test$");
    ///
    fn spec_path_to_regex_str(path: &str) -> regex::Regex {
        let mut in_var = false;

        let mut rstr: Vec<u8> = Vec::new();
        for c in path.bytes() {
            if [c] == "{".as_bytes() {
                in_var = true;
                rstr.push(b"("[0]);
                rstr.push(b"?"[0]);
                rstr.push(b"P"[0]);
                rstr.push(b"<"[0]);
            }

            if [c] == "}".as_bytes() {
                in_var = true;
                rstr.push(b">"[0]);
                rstr.push(b"["[0]); // Match anything but forward slash
                rstr.push(b"^"[0]); // So we do not match long urls, only one variable
                rstr.push(b"/"[0]);
                rstr.push(b"]"[0]);
                rstr.push(b"*"[0]);
                rstr.push(b")"[0]);
            }

            if !in_var {
                rstr.push(c);
            }
            in_var = false;
        }
        let string = format!(r"^{}$", std::str::from_utf8(&rstr).unwrap());
        // string
        regex::Regex::new(&string).expect("Could not create regex")
    }

    fn base_path(server: &Server) -> String {
        if let Some(variables) = &server.variables {
            match variables.get("basePath") {
                Some(base_path) => {
                    let mut base_str = base_path.default.clone();
                    let last_character = base_str.chars().last().unwrap();
                    if last_character == '/' {
                        base_str.pop();
                    }
                    base_str.clone()
                }
                None => "".to_string(),
            }
        } else {
            let url_parse = &server.url.parse::<Uri>();
            match url_parse {
                Ok(url) => url.path().to_string(),
                Err(_) => "".to_string(),
            }
        }
    }

    fn create_path_regexes(spec: OpenAPI) -> Vec<PathMatcher> {
        let mut result = Vec::new();
        let base_path = RequestBuilder::base_path(&spec.servers[0]);
        for (p, path_item) in spec.paths {
            let path = format!("{}{}", base_path, p);
            let pr = PathMatcher {
                regex: RequestBuilder::spec_path_to_regex_str(&path),
                path: deref_own(path_item),
            };
            result.push(pr);
        }
        result
    }
}
