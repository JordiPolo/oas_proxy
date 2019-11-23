use openapiv3::*;
use regex::Regex;
//use anyhow::Result;

use openapi_utils::ReferenceOrExt;
use openapi_utils::ServerExt;

use crate::error::E;

#[derive(Debug)]
pub struct PathFinder {
    pub path_matches: Vec<PathMatch>,
}

#[derive(Debug)]
pub struct PathMatch {
    pub regex: Regex,
    pub path: PathItem,
}

impl PathFinder {
    pub fn new(spec: OpenAPI) -> Self {
        PathFinder {
            path_matches: Self::create_path_regexes(spec),
        }
    }
    pub fn find<'a>(&'a mut self, path: &str) -> Result<&'a mut PathMatch, E> {
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
    /// assert_eq!(result, "^/study/(?P<uuid>[^/]*)/test$");
    ///
    fn spec_path_to_regex_str(path: &str) -> Regex {
        let replaced = path.replace("{", "(?P<").replace("}", ">[^/]*)");
        let string = format!(r"^{}$", replaced);
        Regex::new(&string).expect(&format!("Could not create regex from path {}.", path))
    }


    fn create_path_regexes(spec: OpenAPI) -> Vec<PathMatch> {
        let mut result = Vec::new();
        let base_path = spec.servers[0].base_path();
        for (p, path_item) in spec.paths {
            let path = format!("{}{}", base_path, p);
            let pr = PathMatch {
                regex: Self::spec_path_to_regex_str(&path),
                path: path_item.to_item(),
            };
            result.push(pr);
        }
        result
    }
}
