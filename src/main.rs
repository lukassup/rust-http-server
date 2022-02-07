mod http;
use http::Server;

fn main() {
    println!("webapp v0.0-dev");
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
