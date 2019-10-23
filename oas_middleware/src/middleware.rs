use hyper::{Body, Request, Response, StatusCode};
use simple_proxy::proxy::error::MiddlewareError;
use simple_proxy::proxy::middleware::MiddlewareResult::{Next, RespondWith};
use simple_proxy::proxy::middleware::{Middleware, MiddlewareResult};
use simple_proxy::proxy::service::{ServiceContext, State};

use anyhow::Error;
use http::uri::Uri;
use serde_json::json;
use std::path::Path;

use crate::validator;
use crate::spec_reader;
use crate::request;


pub struct OASMiddleware{//<'a> {
  //  spec: OpenAPI,
    request_builder: request::RequestBuilder,//<'a>,
}
impl OASMiddleware {
    pub fn new<P: AsRef<Path>>(filename: P) -> Self {
        let spec = spec_reader::read_and_deref_all(filename);
        let request_builder = request::RequestBuilder::new(spec);
        OASMiddleware {
         //   spec,
            request_builder,
        }
    }
}

fn error_to_json(error: Error, uri: &Uri) -> String {
    let causes: Vec<String> = error.chain().map(|e| e.to_string()).collect();

    json!({
        "type": "FailedContractValidation",
        "title": "Request not consistent with OpenAPI description.",
        "failed_url": uri.to_string(),
        "causes": causes,
    })
    .to_string()
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
            let spec = format!("{:?}", &self.request_builder);
            let ok: Response<Body> = Response::new(Body::from(spec));
            return Ok(RespondWith(ok));
        }

        let mut request = self.request_builder.build(&req)?;
        debug!("Request {:?}", request);


        match validator::validate(&mut request) {
            Ok(()) => {
                info!("Proxying");
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
