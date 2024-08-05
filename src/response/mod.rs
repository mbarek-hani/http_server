#![allow(dead_code)]
use std::{fmt::Display, fs};

use http::{HeaderMap, HeaderName, StatusCode};

use crate::request::HttpRequest;


pub struct HttpResponse {
    pub status_code: StatusCode,
    pub headers: HeaderMap,
    pub body: String
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = format!("HTTP/1.1 {}\r\n", self.status_code);
        let headers: String = self.headers.iter().map(|(k,v)| format!("{}: {}\r\n", k, v.to_str().unwrap())).collect();
        
        write!(f, "{}{}\r\n{}", status, headers, self.body)

    }
}
pub struct HttpResponseBuilder {
    status_code: StatusCode,
    headers: HeaderMap,
    body: String
}

impl HttpResponse {
    pub fn new(code: StatusCode) -> HttpResponseBuilder{
        HttpResponseBuilder {
            status_code: code,
            headers: HeaderMap::new(),
            body: String::new()
        }
    }

    pub fn render(file_path: &str) -> HttpResponse {
        let template = fs::read_to_string(file_path).expect(&format!("template {} not found", file_path));
        HttpResponse::new(StatusCode::OK)
            .header("content-type: text/html")
            .header(&format!("content-length: {}", template.len()))
            .body(template)
            .build()
    }

    pub fn json(body: String) -> HttpResponse {
        HttpResponse::new(StatusCode::OK)
            .header("content-type: application/json")
            .header(&format!("content-length: {}", body.len()))
            .body(body)
            .build()
    }


    pub fn serve_static_file(req: &HttpRequest) -> HttpResponse {
        let file_result = fs::read_to_string(req.path.strip_prefix('/').unwrap());
        match file_result {
            Ok(file) => {
                let extention = req.path.split('.').skip(1).next().unwrap();
                let content_type = match extention {
                    "html" => "text/html",
                    "css" => "text/css",
                    "js" => "text/javascript",
                    "jpeg" => "image/jpeg",
                    "jpg" => "image/jpg",
                    "png" => "image/png",
                    _ => "text/plain"
                };
                return HttpResponse::new(StatusCode::OK)
                    .header(&format!("content-length: {}", file.len()))
                    .header(&format!("content-type: {}", content_type))
                    .body(file)
                    .build();
            },
            Err(_) => {
                return HttpResponse::new(StatusCode::NOT_FOUND)
                    .header("content-length: 0")
                    .header("content-type: text/plain")
                    .body("".to_string())
                    .build();
            }
        }
        
    }

    pub fn redirect(to: &str) -> HttpResponse {
        let response = HttpResponse::new(StatusCode::PERMANENT_REDIRECT)
                                                .header(&format!("Location: {}", to))
                                                .build();
        response
    }
}

impl HttpResponseBuilder {
    pub fn header(mut self, header_string: &str) -> HttpResponseBuilder {
        let mut header = header_string.split(": ");
        self.headers.append(header.next().unwrap().parse::<HeaderName>().unwrap(), header.next().unwrap().parse().unwrap());
        self
    }

    pub fn body(mut self, body: String) -> HttpResponseBuilder {
        self.body = body;
        self
    }

    pub fn build(self) -> HttpResponse {
        HttpResponse {
            status_code: self.status_code,
            headers: self.headers,
            body: self.body
        }
    }

}
