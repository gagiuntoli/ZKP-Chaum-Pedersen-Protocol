use hex;
use num_bigint::BigUint;
use std::io::stdin;
use zkp_course::{deserialize, exponentiate, random_number, serialize, solve, G, H, P, Q};

pub mod zkp_auth {
    include!("./zkp_auth.rs"); // The string specified here must match the proto package name
}

use zkp_auth::auth_client::AuthClient;
use zkp_auth::{AuthenticationChallengeRequest, AutheticationAnswerRequest, RegisterRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AuthClient::connect("http://127.0.0.1:50051").await?;

    let mut buffer = String::new();

    let g = deserialize(&G);
    let h = deserialize(&H);
    let p = deserialize(&P);
    let q = deserialize(&Q);

    println!("Please enter the user ID:");
    stdin().read_line(&mut buffer).expect("Expected an input");
    let user_id = buffer.trim().to_string();

    println!("Please enter the password x (should be large number)");
    buffer = String::new();
    stdin().read_line(&mut buffer).expect("Expected an input");

    let x = buffer
        .trim()
        .parse::<String>()
        .expect("Expected a valid string");

    let x = BigUint::from_bytes_be(&hex::decode(x).expect("Expected a hexadecimal number"));

    let y1 = exponentiate(&g, &x, &p);
    let y2 = exponentiate(&h, &x, &p);

    let request = tonic::Request::new(RegisterRequest {
        user: user_id.clone(),
        y1: serialize(&y1),
        y2: serialize(&y2),
    });

    let _response = client.register(request).await?;

    let k = random_number();
    let r1 = exponentiate(&g, &k, &p);
    let r2 = exponentiate(&h, &k, &p);

    let request = tonic::Request::new(AuthenticationChallengeRequest {
        user: user_id,
        r1: serialize(&r1),
        r2: serialize(&r2),
    });

    let response = client.create_authentication_challenge(request).await?;

    let response = response.into_inner();
    let auth_id = response.auth_id;
    let c = deserialize(&response.c);

    let s = solve(&x, &k, &c, &q);

    let request = tonic::Request::new(AutheticationAnswerRequest {
        auth_id,
        s: serialize(&s),
    });

    let response = client.verify_authentication(request).await?;

    let response = response.into_inner();
    let session_id = response.session_id;

    println!("session id = {:?}", session_id);

    Ok(())
}
