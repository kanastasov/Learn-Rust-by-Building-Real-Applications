use super::server::Handler;
use super::http::{Request,Response,StatusCode};
pub struct WebsiteHandler;

impl Handler for WebsiteHandler{
    fn handler_request(&mut slef, request:&Request) -> Response {
        Response::new(StatusCode::Ok, Some("<h1>Test</h1>".to_string()))
    }
}