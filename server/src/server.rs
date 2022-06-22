
    use std::net::TcpListener;
    use std::convert::TryFrom;
    use std::convert::TryInto;
    use std::io::Read;
    //use create::http::Request;
    use create::http::{Request, Response, StatusCode};


    pub trait Handler {
        fn handle_request(&mut self, request: &Request) -> Response;

        fn handle_bad_request(&mut self, e: &ParseError) -> Response{
            println!("Failed to parse request: {}",e);
            Response::new(StatusCode::BadRequest,None);
        }
    }


    pub struct Server {
        addr: String,
    }

    // fn arr(a: &[u8]) {

    // }

    impl Server {
       pub fn new(addr: String) -> Self{
            Self{
                addr
            }
        }

        pub fn run(self, mut handler: impl Handler) {
            println!("Listening on {}", self.addr);
            let listener = TcpListener::bind(&self.addr).unwrap();

            loop {
                match listener.accept(){
                    Ok((mut stream, _)) => {
                        // let a = [1,2,3,4];
                        // arr(&a);
                     let mut buffer = [0; 1024];
                     match stream.read(&mut buffer){
                        Ok(_) => {
                            println!("Recieved a request: {:?}", String::from_utf8_lossy(&buffer));
                            // buffer[1] = 0;
                            // let a = request;
                            match Request::try_from(&buffer[..]);{
                                Ok(request) => {
                                    //  dbg!(request); 
                                    //  let response = Response::new(StatusCode::Ok, Some("<h1> HTML works parse!! </h1>".to_string()));
                                    //  response.send(&mut stream)
                                    //  write!(stream,"{}", response);
                                    handler.handle_request(&request);

                                }
                                Err(e) => {
                                    handler.handle_bad_request(&e)
                                }
                            }
 


                        }
                        Err(e) => println!("Failed to read from connect: {}",e),
                     }
                    },
                    Err(e) => println!("Failed to establish a connection: {}",e),
                }
                let res = listener.accept();
                if res.is_err(){
                    continue;
                }

                let (stream, addr) = res.unwrap();
            }

              
        }
    }