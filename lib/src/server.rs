use tonic::transport::Server;

use public_ip::{dns, http, ToResolver, BoxToResolver};

use node_lib::hello_world::greeter_server::GreeterServer;
use node_lib::MyGreeter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resolver = vec![
        BoxToResolver::new(dns::OPENDNS_RESOLVER),
        BoxToResolver::new(http::HTTP_IPIFY_ORG_RESOLVER),
    ].to_resolver();

    let ip = if let Some(ip) = public_ip::resolve_address(resolver).await {
        println!("public ip address: '{}'", ip);
        ip
    } else {
        panic!("coudln't get an IP address");
    };

    let address = format!("{}:50051", ip);
    qr2term::print_qr(address).unwrap();

    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}