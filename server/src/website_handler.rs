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

    fn read_file(&slef, file_path:&str) -> Option<string>{
        let path = format!("{}/{}",slef.public_path, file_path);
        fs::read_to_string(path).ok();
    }
}
impl Handler for WebsiteHandler{
    fn handler_request(&mut slef, request:&Request) -> Response {
        //Response::new(StatusCode::Ok, Some("<h1>Test</h1>".to_string()))
        match request.method(){
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                _ => Response::new(StatusCode::NotFound,None),

            }
            _ => Response::new(StatusCode::NotFound,None)
        }
    }
}