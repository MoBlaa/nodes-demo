use tonic::transport::Server;

use lib::hello_world::greeter_server::GreeterServer;
use lib::MyGreeter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    qr2term::print_qr("https://rust-lang.org/").unwrap();

    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}