pub mod router;

use std::net::{TcpListener, TcpStream};
use log_rs::{Logger, OutputKind};
use router::Router;
use std::io::{Read, Write};
use crate::request::HttpRequest;
use crate::response::HttpResponse;
use http::StatusCode;

pub struct Server {
    listener: TcpListener,
    port: usize,
    router: Router
}

impl Server {
    pub fn new(port: usize) -> Result<Self, Box<dyn std::error::Error>>{
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
        Ok(Self {
            listener,
            port,
            router: Router::new()
        })
    }

    pub fn listen(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut log = Logger::new().unwrap();
        log.config_format("%l %t : %m");

        log.config_info(OutputKind::STDOUT);
        log.config_error(OutputKind::FILE("errlog.txt"));

        log.info(&format!("The server is listening on http://localhost:{}", self.port));

        for stream in self.listener.incoming(){
            match stream {
                Ok(stream) => self.handle_connection(stream)?,
                Err(e) => log.error(&format!("{}", e)),
            }
        }

        Ok(())
    }

    fn handle_connection(&self, mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        let mut log = Logger::new().unwrap();
        log.config_format("%l %t : %m");
        log.config_info(OutputKind::STDOUT);
        log.config_error(OutputKind::FILE("errlog.txt"));

        let mut buffer: [u8;1024] = [0; 1024];
        stream.read(&mut buffer)?;
        let request: HttpRequest = match String::from_utf8(buffer.into())?.try_into() {
            Ok(req) => req,
            Err(e) => {
                log.error(&format!("{}", e));
                HttpRequest::new()
            }
        };
    
        let response: HttpResponse;
    
        if request.is_empty() {
            response = HttpResponse::new(StatusCode::BAD_REQUEST)
                        .header("content-type: text/html")
                        .header("content-length: 0")
                        .body("".to_string())
                        .build();
        } else if request.is_for_static_file() {
            response = HttpResponse::serve_static_file(&request);
        }else {
            response = self.router.resolve(&request);
        }
    
        log.info(&format!("{:?} {} {}", request.method, request.path, response.status_code));
        
        stream.write(response.to_string().as_bytes())?;
        Ok(())
    }

    pub fn router(&mut self) -> &mut Router {
        &mut self.router
    }
}

