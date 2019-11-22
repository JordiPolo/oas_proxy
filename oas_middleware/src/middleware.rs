use hyper::header::HeaderValue;
use hyper::{Body, Request, Response, StatusCode};

use simple_proxy::proxy::error::MiddlewareError;
use simple_proxy::proxy::middleware::MiddlewareResult::{Next, RespondWith};
use simple_proxy::proxy::middleware::{Middleware, MiddlewareResult};
use simple_proxy::proxy::service::{ServiceContext, State};

use anyhow::Error;
use http::uri::Uri;
use log::{debug, info};
use serde_json::json;
use std::path::Path;

use openapi_utils::deref_all;

use crate::path_finder::PathFinder;
use crate::request;
use crate::spec_utils;
use crate::usage_report;
use crate::validator;

pub struct OASMiddleware {
    path_finder: PathFinder,
}
impl OASMiddleware {
    pub fn new<P: AsRef<Path>>(filename: P) -> Self {
        let spec = deref_all(spec_utils::read(filename));
        let path_finder = PathFinder::new(spec);
        debug!("{:?}", path_finder);

        OASMiddleware { path_finder }
    }
}

impl Middleware for OASMiddleware {
    fn name() -> String {
        String::from("OpenAPI Validator Middleware")
    }

    fn before_request(
        &mut self,
        req: &mut Request<Body>,
        _context: &ServiceContext,
        _state: &State,
    ) -> Result<MiddlewareResult, MiddlewareError> {
        info!("New request to {}", req.uri());

        if req.uri().path() == "/report" {
            let usage_report = usage_report::render_report(&self.path_finder);
            let mut response: Response<Body> = Response::new(Body::from(usage_report));
            response.headers_mut().insert(
                "Content-Type",
                HeaderValue::from_str("application/json").unwrap(),
            );
            return Ok(RespondWith(response));
        }

        let path = self.path_finder.find(req.uri().path()).unwrap(); //TODO: anyhow error here
                                                                     //   .map_err(|error| middleware_error(error, req.uri()))?;

        let request_parts = request::RequestParts::new(&path.regex, &req);
        let mut openapi_parts = crate::parts::OpenAPIParts::new(&mut path.path, &req)
            .map_err(|error| middleware_error(error, req.uri()))?;

        //let (openapi_parts, request_parts) = parts::get_parts(&req).map_err(|error| middleware_error(error, req.uri()))?;

        match validator::validate(&mut openapi_parts, &request_parts) {
            Ok(()) => {
                info!("Proxying");
                let headers = req.headers_mut();
                headers.insert("OAS-Proxied", HeaderValue::from_str("true").unwrap());
                Ok(Next)
            }
            Err(error) => {
                let e = error.context("Failed validation of request variables.");
                Err(middleware_error(e, req.uri()))
            }
        }
    }

    fn after_request(
        &mut self,
        _res: Option<&mut Response<Body>>,
        _context: &ServiceContext,
        _state: &State,
    ) -> Result<MiddlewareResult, MiddlewareError> {
        Ok(Next)
    }
}

fn middleware_error(error: Error, uri: &Uri) -> MiddlewareError {
    info!("Failed to validate. Not proxying");
    info!("{:?}", error);
    MiddlewareError::new(
        String::from("Request not consistent with OpenAPI description."),
        Some(error_to_json(error, uri)),
        StatusCode::BAD_REQUEST,
    )
}

fn error_to_json(error: Error, uri: &Uri) -> String {
    let causes: Vec<String> = error.chain().map(|e| e.to_string()).collect();

    json!({
        "type": "errors:contract_broken",
        "title": "The request does not agree with the API contract. Not proxying.",
        "failed_url": uri.to_string(),
        "causes": causes,
        "status": 422,
    })
    .to_string()
}
