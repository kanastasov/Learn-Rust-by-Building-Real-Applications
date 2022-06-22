use super::server::Handler;
use super::http::{Request,Response,StatusCode, Method};
pub struct WebsiteHandler;

pub struct WebsiteHandler {
    public_path:String
}

impl WebsiteHandler{
    pub fn new (public_path: String) -> Self {
        Self{public_path}
    }
}
impl Handler for WebsiteHandler{
    fn handler_request(&mut slef, request:&Request) -> Response {
        //Response::new(StatusCode::Ok, Some("<h1>Test</h1>".to_string()))
        match request.method(){
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, Some("<h1>Welcome here</h1>".to_string())),
                "/hello" => Response::new(StatusCode::Ok, Some("<h1>hello</h1>".to_string())),
                _ => Response::new(StatusCode::NotFound,None),

            }
            _ => Response::new(StatusCode::NotFound,None)
        }
    }
}