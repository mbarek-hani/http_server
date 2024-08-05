#![allow(dead_code)]
use std::collections::HashMap;

use crate::{request::{Method, HttpRequest}, response::HttpResponse};
use http::StatusCode;

pub struct Router {
    routes: HashMap<(Method, String), fn(&HttpRequest) -> HttpResponse>
}

impl Router {
    pub fn new() -> Self{
        Self {
            routes: HashMap::new()
        }
    }
    pub fn get(&mut self, path: &str, handler: fn(&HttpRequest) -> HttpResponse) {
        self.routes.insert((Method::GET, path.to_string()), handler);
    }

    pub fn post(&mut self, path: &str, handler: fn(&HttpRequest) -> HttpResponse) {
        self.routes.insert((Method::POST, path.to_string()), handler);
    }

    pub fn put(&mut self, path: &str, handler: fn(&HttpRequest) -> HttpResponse) {
        self.routes.insert((Method::PUT, path.to_string()), handler);
    }

    pub fn patch(&mut self, path: &str, handler: fn(&HttpRequest) -> HttpResponse) {
        self.routes.insert((Method::PATCH, path.to_string()), handler);
    }

    pub fn delete(&mut self, path: &str, handler: fn(&HttpRequest) -> HttpResponse) {
        self.routes.insert((Method::DELETE, path.to_string()), handler);
    }

    pub fn resolve(&self, req: &HttpRequest) -> HttpResponse{
        if let Some(func) = self.routes.get(&(req.method.clone(), req.path.clone())){
            func(req)
        }else {
            Self::not_found(req)
        }
    }

    fn not_found(_req: &HttpRequest) -> HttpResponse {
        let body = "<h1>404 Not Found</h1>";
        let response = HttpResponse::new(StatusCode::NOT_FOUND)
                                            .header("content-type: text/html")
                                            .header(&format!("content-length: {}", body.len()))
                                            .body(body.to_string())
                                            .build();
        response
    }
}