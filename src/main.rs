mod request;
mod response;
mod server;


use server::Server;
use response::HttpResponse;
use request::HttpRequest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = Server::new(8000)?;

    let router = server.router();

    router.get("/", home);
    router.get("/tohome", tohome);


    server.listen()?;
    Ok(())
}

fn home(_req: &HttpRequest) -> HttpResponse {
    let body = String::from("{\"message\": \"Welcome home!\"}");
    HttpResponse::json(body)
}

fn tohome(_req: &HttpRequest) -> HttpResponse {
    HttpResponse::redirect("/")
}
