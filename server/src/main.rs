fn main() {
    // let string = String::from("127.0.0.1:8080");
    let server = Server::new("127.0.0.1:8080".to_string());
    
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self { 
        Self { 
            addr
        }
    }

    fn run(self) {
        println!("Listening on {}", self.addr);
    }
}

/*
 GET /user?id=10 HTTP/1.1\r\n
 HEADERS \r\n
 BODY
 */