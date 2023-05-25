use tonic::{transport::Server, Request, Response, Status};

pub mod zkp_auth {
    include!("./zkp_auth.rs"); // The string specified here must match the proto package name
}

use zkp_auth::auth_server::{Auth, AuthServer};
use zkp_auth::{
    AuthenticationChallengeRequest, AuthenticationChallengeResponse, AutheticationAnswerRequest,
    AutheticationAnswerResponse, RegisterRequest, RegisterResponse,
};

#[derive(Debug, Default)]
pub struct AuthImpl {}

#[tonic::async_trait]
impl Auth for AuthImpl {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        println!("Got a request: {:?}", request);

        // let reply = hello_world::HelloReply {
        //     message: format!("Hello {}!", request.into_inner().name).into(),
        // };

        Ok(Response::new(RegisterResponse {}))
    }

    async fn create_authentication_challenge(
        &self,
        request: tonic::Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        todo!()
    }

    async fn verify_authentication(
        &self,
        request: Request<AutheticationAnswerRequest>,
    ) -> Result<tonic::Response<AutheticationAnswerResponse>, Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running the server");

    let addr = "127.0.0.1:50051".parse()?;
    let auth_impl = AuthImpl::default();

    Server::builder()
        .add_service(AuthServer::new(auth_impl))
        .serve(addr)
        .await?;

    Ok(())
}
