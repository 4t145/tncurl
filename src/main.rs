use client_util::{
    http_body_util::BodyExt,
    request::{RequestBuilderExt, RequestExt},
    response::ResponseExt,
};
use http::{header::CONTENT_TYPE, HeaderValue};
use tonic::{client::GrpcService, Status};
use tower::util::ServiceExt;
#[tokio::main]
async fn main() {
    let conn = tonic::transport::Endpoint::from_static("https://grpc.postman-echo.com");
    let mut channel = conn.connect().await.expect("valid connection");

    let req = http::Request::post("https://grpc.postman-echo.com/HelloService/SayHello")
        .json(&serde_json::json!({
            "greeting": "tncurl"
        }))
        .expect("valid request")
        .with_header(CONTENT_TYPE, HeaderValue::from_static("application/grpc+json"))
        .map(|b| b.map_err(|_| unreachable!()).boxed_unsync());
    let (parts, json) = channel
        .ready()
        .await
        .expect("ready")
        .call(req)
        .await
        .expect("valid response")
        .text()
        .await
        .expect("valid text")
        .into_parts();
    println!("{:?}", parts);
    println!(
        "{}",
        serde_json::to_string_pretty(&json).expect("valid json")
    );
}
