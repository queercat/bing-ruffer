use index::greeter_server::GreeterServer;
use index::{greeter_server::Greeter, HelloReply, HelloRequest};
use tonic::{transport::Server, Request, Response};

pub mod index {
    tonic::include_proto!("index");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, tonic::Status> {
        println!("Got a request: {:?}", request);
        let reply = index::HelloReply {
            message: "Hello there!".into(),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    println!("Greeter server listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
