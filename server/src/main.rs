use server::Server;
use http::Request;
use http::Method;

mod server;
mod http;

fn main() {

    // let string = String::from("127.0.0.1:8080");
    // let string_slice = &string[10..14];
    // let string_borrow: &str = &string;
    // let string_literal = "1234";


    // dbg!(&string);
    // dbg!(&string_slice);
    
    // dbg!(&string_borrow);
    // dbg!(&string_literal);

    let get = Method::GET;
    let delete = Method::DELETE;
    let post = Method::POST;
    let put = Method::PUT;

    let server = Server::new("127.0.01.1:8080".to_string());
    server.run();
}
