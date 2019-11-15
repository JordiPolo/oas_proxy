use hyper::{Body, Request, Response, StatusCode};
use simple_proxy::proxy::error::MiddlewareError;
use simple_proxy::proxy::middleware::MiddlewareResult::{Next, RespondWith};
use simple_proxy::proxy::middleware::{Middleware, MiddlewareResult};
use simple_proxy::proxy::service::{ServiceContext, State};

use anyhow::Error;
use http::uri::Uri;
use serde_json::json;
use std::path::Path;
use log::{info, debug};

use openapi_deref::{deref_all};

use crate::request;
use crate::spec_utils;
use crate::validator;
use crate::usage_report;

pub struct OASMiddleware {
    request_builder: request::RequestBuilder,
}
impl OASMiddleware {
    pub fn new<P: AsRef<Path>>(filename: P) -> Self {
        let spec = deref_all(spec_utils::read(filename));
        let request_builder = request::RequestBuilder::new(spec);
        debug!("{:?}", request_builder);

        OASMiddleware {
            request_builder,
        }
    }
}

fn error_to_json(error: Error, uri: &Uri) -> String {
    let causes: Vec<String> = error.chain().map(|e| e.to_string()).collect();

    json!({
        "type": "errors:contract_broken",
        "title": "The request does not follow the rules of the API contract.",
        "failed_url": uri.to_string(),
        "causes": causes,
        "status": 422,
    })
    .to_string()
}
use hyper::header::HeaderValue;

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
            let usage_report = usage_report::render_report(&self.request_builder);
            let mut response: Response<Body> = Response::new(Body::from(usage_report));
            response.headers_mut().insert(
                "Content-Type", HeaderValue::from_str("application/json").unwrap()
            );
            return Ok(RespondWith(response));
        }

        let mut request = self.request_builder.build(&req).map_err(|error| {
            info!("Failed to create request. Not proxying");
            info!("{:?}", error);
            MiddlewareError::new(
                String::from("Request information not found in the OpenAPI file."),
                Some(error_to_json(error, req.uri())),
                StatusCode::BAD_REQUEST,
            )
        })?;

        debug!("Request {:?}", request);

        match validator::validate(&mut request) {
            Ok(()) => {
                info!("Proxying");
                let headers = req.headers_mut();
                headers.insert("OAS-Proxied", HeaderValue::from_str("true").unwrap());
                Ok(Next)
            }
            Err(error) => {
                info!("Failed to validate. Not proxying");
                info!("{:?}", error);
                Err(MiddlewareError::new(
                    String::from("Request not consistent with OpenAPI description."),
                    Some(error_to_json(error, req.uri())),
                    StatusCode::BAD_REQUEST,
                ))
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
