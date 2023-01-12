use tonic::{Request, Response, Status};

use crate::pb;

#[derive(Default)]
pub struct GreeterServiceImpl {}

#[tonic::async_trait]
impl pb::helloworld::greeter_server::Greeter for GreeterServiceImpl {
    async fn say_hello(
        &self,
        _req: Request<pb::helloworld::HelloRequest>,
    ) -> Result<Response<pb::helloworld::HelloResponse>, Status> {
        let resp = pb::helloworld::HelloResponse { timestamp: 123 };

        Ok(Response::new(resp))
    }
}
