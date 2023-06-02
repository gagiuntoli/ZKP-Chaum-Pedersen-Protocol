use num_bigint::BigUint;
use num_traits::Zero;
use std::collections::HashMap;
use std::sync::Mutex;
use tonic::{transport::Server, Code, Request, Response, Status};
use zkp_course::{deserialize, random_number, random_string, serialize, verify, G, H, P};

pub mod zkp_auth {
    include!("./zkp_auth.rs"); // The string specified here must match the proto package name
}

use zkp_auth::auth_server::{Auth, AuthServer};
use zkp_auth::{
    AuthenticationChallengeRequest, AuthenticationChallengeResponse, AutheticationAnswerRequest,
    AutheticationAnswerResponse, RegisterRequest, RegisterResponse,
};

#[derive(Debug, Default)]
pub struct UserInfo {
    pub y1: BigUint,
    pub y2: BigUint,
    pub r1: BigUint,
    pub r2: BigUint,
    pub c: BigUint,
    pub session_id: String,
}

#[derive(Debug, Default)]
pub struct AuthImpl {
    pub user_info: Mutex<HashMap<String, UserInfo>>, // <user_id, UserInfo>
    pub auth_info: Mutex<HashMap<String, String>>,   // <auth_id, user_id>
}

#[tonic::async_trait]
impl Auth for AuthImpl {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        println!("Got a RegisterRequest: {:?}", request);

        let request = request.into_inner();
        let user_id = request.user;

        let user_info = UserInfo {
            y1: deserialize(&request.y1),
            y2: deserialize(&request.y2),
            r1: BigUint::zero(),
            r2: BigUint::zero(),
            c: BigUint::zero(),
            session_id: String::new(),
        };

        let user_info_hashmap = &mut self.user_info.lock().unwrap(); // TODO: check if we can improve this `unwrap`
        user_info_hashmap.insert(user_id, user_info);

        println!("{:?}", user_info_hashmap);

        Ok(Response::new(RegisterResponse {}))
    }

    async fn create_authentication_challenge(
        &self,
        request: tonic::Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        println!("Got a AuthenticationChallengeRequest: {:?}", request);

        let request = request.into_inner();
        let user_id = request.user;

        let user_info_hashmap = &mut self.user_info.lock().unwrap(); // TODO: check if we can improve this `unwrap`

        if let Some(user_info) = user_info_hashmap.get_mut(&user_id) {
            let auth_id = random_string(6);
            let c = random_number();

            user_info.r1 = deserialize(&request.r1);
            user_info.r2 = deserialize(&request.r2);
            user_info.c = c.clone();

            println!("{:?}", user_info_hashmap);

            // We store the <auth_id, user_id> key-value pair
            let auth_info_hashmap = &mut self.auth_info.lock().unwrap(); // TODO: check if we can improve this `unwrap`
            auth_info_hashmap.insert(auth_id.clone(), user_id);

            Ok(Response::new(AuthenticationChallengeResponse {
                auth_id,
                c: serialize(&c),
            }))
        } else {
            Err(Status::new(
                Code::NotFound,
                format!("User {} not found", user_id),
            ))
        }
    }

    async fn verify_authentication(
        &self,
        request: Request<AutheticationAnswerRequest>,
    ) -> Result<tonic::Response<AutheticationAnswerResponse>, Status> {
        let request = request.into_inner();
        let auth_id = request.auth_id;

        let auth_info_hashmap = &self.auth_info.lock().unwrap(); // TODO: check if we can improve this `unwrap`

        if let Some(user_id) = auth_info_hashmap.get(&auth_id) {
            let user_info_hashmap = &mut self.user_info.lock().unwrap(); // TODO: check if we can improve this `unwrap`

            if let Some(user_info) = user_info_hashmap.get_mut(user_id) {
                let s = deserialize(&request.s);
                let y1 = &user_info.y1;
                let y2 = &user_info.y2;
                let r1 = &user_info.r1;
                let r2 = &user_info.r2;
                let c = &user_info.c;

                if verify(
                    &deserialize(G),
                    &deserialize(H),
                    &deserialize(P),
                    y1,
                    y2,
                    r1,
                    r2,
                    c,
                    &s,
                ) {
                    let session_id = random_string(6);
                    user_info.session_id = session_id.clone();

                    println!("{:?}", user_info_hashmap);

                    Ok(Response::new(AutheticationAnswerResponse { session_id }))
                } else {
                    Err(Status::new(
                        Code::Unauthenticated,
                        format!("Challenge not solved correctly"),
                    ))
                }
            } else {
                Err(Status::new(
                    Code::NotFound,
                    format!("User {} not found -> This shouldn't happen", user_id),
                ))
            }
        } else {
            Err(Status::new(
                Code::NotFound,
                format!("Auth ID {} not found", auth_id),
            ))
        }
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
