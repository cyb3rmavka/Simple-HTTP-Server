use std::option::Option;

fn main() {

    let server = Server::new("127.0.0.1::8082".to_string());
    server.run();

}

struct Server{
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    fn run(self){
        todo!();
    }
}

struct Request {
    path: String,
    query_string: Option<String>,
    method: todo!(),
}

enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATH
}