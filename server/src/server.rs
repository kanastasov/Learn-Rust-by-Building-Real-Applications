
    use std::net::TcpListener;
    use std::io::Read;
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
                     stream.read();
                    },
                    Err(e) => println!("Failed to establish a connection: ()",e),
                }
                let res = listener.accept();
                if res.is_err(){
                    continue;
                }

                let (stream, addr) = res.unwrap();
            }

              
        }
    }