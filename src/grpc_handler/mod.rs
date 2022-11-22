use tonic::{Request, Response, Status};

use crate::pb;

#[derive(Default)]
pub struct GreeterServiceImpl {}

#[tonic::async_trait]
impl pb::helloworld::greeter_server::Greeter for GreeterServiceImpl {
    async fn say_hello(
        &self,
        req: Request<pb::helloworld::HelloRequest>,
    ) -> Result<Response<pb::helloworld::HelloResponse>, Status> {
        let resp = pb::helloworld::HelloResponse {
            message: format!("Hello {}", req.get_ref().name),
            test_array: Vec::new(),
        };

        Ok(Response::new(resp))
    }
}
