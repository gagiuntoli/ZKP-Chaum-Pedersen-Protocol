use tonic::{transport::Server, Code, Request, Response, Status};
use zkp_course::{exponentiate, random_number, solve, G, H, P, Q};

pub mod zkp_auth {
    include!("./zkp_auth.rs"); // The string specified here must match the proto package name
}

use zkp_auth::auth_client::AuthClient;
use zkp_auth::{
    AuthenticationChallengeRequest, AuthenticationChallengeResponse, AutheticationAnswerRequest,
    AutheticationAnswerResponse, RegisterRequest, RegisterResponse,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AuthClient::connect("http://127.0.0.1:50051").await?;

    let user_id = "Guido".to_string();
    let x = 6;

    let y1 = exponentiate(G, x, P);
    let y2 = exponentiate(H, x, P);

    let request = tonic::Request::new(RegisterRequest {
        user: user_id.clone(),
        y1,
        y2,
    });

    let response = client.register(request).await?;

    let k = random_number() % 10;
    let r1 = exponentiate(G, k, P);
    let r2 = exponentiate(H, k, P);

    let request = tonic::Request::new(AuthenticationChallengeRequest {
        user: user_id,
        r1,
        r2,
    });

    let response = client.create_authentication_challenge(request).await?;

    let response = response.into_inner();
    let auth_id = response.auth_id;
    let c = response.c;

    let s = solve(x, k, c, Q);

    let request = tonic::Request::new(AutheticationAnswerRequest { auth_id, s });
    let response = client.verify_authentication(request).await?;

    let response = response.into_inner();
    let session_id = response.session_id;

    println!("session id = {:?}", session_id);

    Ok(())
}
