
    use std::net::TcpListener;
    use std::convert::TryFrom;
    use std::convert::TryInto;
    use std::io::Read;
    use create::http::Request;

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

        pub fn run( self) {
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
                            buffer[1] = 0;
                            // let a = request;
                            Request::try_from(&buffer[..]);



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