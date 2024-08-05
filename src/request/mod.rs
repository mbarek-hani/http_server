#![allow(unused)]
use std::{collections::HashMap, error::Error, fmt::Display, str::FromStr};
use http::{HeaderMap, HeaderName, HeaderValue};
use std::fs;

#[derive(Debug)]
pub enum ParseRequestError {
    InvalidRequest,
    ParseMethodError,
    ParseUriError,
    ParseParamsError,
    ParseHeadersError,
    ParseBodyError
}
impl Display for ParseRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::InvalidRequest => "ERROR: invalid request.",
            Self::ParseMethodError => "ERROR: can't parse the method of the request.",
            Self::ParseUriError => "ERROR: can't parse the uri of the request.",
            Self::ParseParamsError => "ERROR: can't parse the query parameters of the request.",
            Self::ParseHeadersError => "ERROR: can't parse the headers of the request.",
            Self::ParseBodyError => "ERROR: can't parse the body of the request."
        };
        f.write_str(msg)
    }
}

impl Error for ParseRequestError {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "get" | "GET" => Self::GET,
            "post" | "POST" => Self::POST,
            "put" | "PUT"=> Self::PUT,
            "patch" | "PATCH" => Self::PATCH,
            "delete" | "DELETE" => Self::DELETE,
            "connect" | "CONNECT" => Self::CONNECT,
            "options" | "OPTIONS" => Self::OPTIONS,
            "trace" | "TRACE" => Self::TRACE,
            "head" | "HEAD" => Self::HEAD,
            _ => Self::GET,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HttpRequest {
    pub method: Method,
    pub path: String,
    pub params: HashMap<String, String>,
    pub body: String,
    pub headers: HeaderMap,
}

impl TryFrom<String> for HttpRequest {

    type Error = ParseRequestError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if !value.contains("HTTP") {
            return Err(ParseRequestError::InvalidRequest)
        }
        let request:Vec<&str> = value.split("\r\n").collect();
        let first_line: Vec<&str> = request.get(0).ok_or(ParseRequestError::InvalidRequest)?.split(" ").collect();
        let method = Method::from(*first_line.get(0).ok_or(ParseRequestError::ParseMethodError)?);
        let mut path = first_line.get(1).ok_or(ParseRequestError::ParseUriError)?.to_string();
        let mut new_path: &str = "";
        let mut params: HashMap<String, String> = HashMap::new();
        let mut body = String::new();
        let mut headers = HeaderMap::new();
        if path.contains("?") {
            let splited_path: Vec<_> = path.split("?").collect();
            new_path = splited_path.get(0).ok_or(ParseRequestError::ParseUriError)?;
            let parameters = splited_path.get(1).ok_or(ParseRequestError::ParseParamsError)?;
            if parameters.contains("&") && parameters.contains('=') {
                let parameters: Vec<_> = parameters.split("&").collect();
                for param in parameters {
                    let splited: Vec<_> = param.split("=").collect();
                    let key = String::from(*splited.get(0).ok_or(ParseRequestError::ParseParamsError)?);
                    let value = String::from(*splited.get(1).ok_or(ParseRequestError::ParseParamsError)?);
                    params.insert(key, value);
                }
            }else {
                let splited: Vec<_> = parameters.split("=").collect();
                let key = String::from(*splited.get(0).ok_or(ParseRequestError::ParseParamsError)?);
                let value = String::from(*splited.get(1).ok_or(ParseRequestError::ParseParamsError)?);
                params.insert(key, value);
            }
        }

        if !new_path.is_empty() {
            path = new_path.to_owned();
        }

        if path.ends_with('/') && path != "/" {
            path = path.strip_suffix('/').unwrap().to_owned(); // remove '/' from the end of the path
        }

        for i in 1..request.len(){
            if request.get(i).ok_or(ParseRequestError::InvalidRequest)?.is_empty() {
                //we got to the end of headers
                break;
            }
            let splited: Vec<_> = request.get(i).ok_or(ParseRequestError::ParseHeadersError)?.split(": ").collect();
            let key: HeaderName = HeaderName::from_str(*splited.get(0).ok_or(ParseRequestError::ParseHeadersError)?).map_err(|_| ParseRequestError::ParseHeadersError)?;
            let value : HeaderValue= HeaderValue::from_str(*splited.get(1).ok_or(ParseRequestError::ParseHeadersError)?).map_err(|_| ParseRequestError::ParseHeadersError)?;
            headers.insert(key, value);
        }
        body =  request.get(request.len()-1).ok_or(ParseRequestError::ParseBodyError)?.trim_end_matches('\0').to_string();
        // if clean_body.contains('&') && clean_body.contains('=') {
        //     let pairs: Vec<_> = clean_body.split('&').collect();
        //     for pair in pairs {
        //         let splited: Vec<_> = pair.split("=").collect();
        //         let key = *splited.get(0).ok_or(ParseRequestError::ParseBodyError)?;
        //         let value = *splited.get(1).ok_or(ParseRequestError::ParseBodyError)?;
        //         body.insert(key.to_string(), value.to_string());
        //     }
        // }else if !clean_body.is_empty() && clean_body.contains('='){
        //     let pair: Vec<_> = clean_body.split("=").collect();
        //     let key = *pair.get(0).ok_or(ParseRequestError::ParseBodyError)?;
        //     let value = *pair.get(1).ok_or(ParseRequestError::ParseBodyError)?;
        //     body.insert(key.to_string(), value.to_string());
        // }
        Ok(Self {
            method,
            path,
            params,
            body,
            headers
        })


    }
}

impl HttpRequest {
    pub fn new() -> Self {
        Self {
            method: Method::GET,
            path: "".to_owned(),
            params: HashMap::new(),
            body: String::new(),
            headers: HeaderMap::new(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.method == Method::GET && self.path.is_empty() && self.params.is_empty() && self.body.is_empty() && self.headers.is_empty()
    }

    pub fn is_for_static_file(&self) -> bool {
        if self.path != "/" {
            let file_name = self.path.strip_prefix('/').unwrap();
            if let Ok(_) = fs::File::open(file_name){
                return true;
            }
        }
        false
        
    }
}