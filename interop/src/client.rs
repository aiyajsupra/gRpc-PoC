use crate::{
    pb::test_service_client::*, pb::unimplemented_service_client::*, pb::*, test_assert,
    TestAssertion,
};
use futures_util::{future, stream, StreamExt};
use tokio::sync::mpsc;
use tonic::transport::Channel;
use tonic::{metadata::MetadataValue, Code, Request, Response, Status};
use std::time::{Duration, Instant};

pub type TestClient = TestServiceClient<Channel>;
pub type UnimplementedClient = UnimplementedServiceClient<Channel>;

const LARGE_REQ_SIZE: usize = 2710_828;
const LARGE_RSP_SIZE: i32 = 3140_159;

pub async fn empty_unary(client: &mut TestClient, assertions: &mut Vec<TestAssertion>) {
    let result = client.empty_call(Request::new(Empty {})).await;

    assertions.push(test_assert!(
        "call must be successful",
        result.is_ok(),
        format!("result={:?}", result)
    ));

    if let Ok(response) = result {
        let body = response.into_inner();
        assertions.push(test_assert!(
            "body must not be null",
            body == Empty {},
            format!("body={:?}", body)
        ));
    }
}

pub async fn large_unary(client: &mut TestClient, assertions: &mut Vec<TestAssertion>) {
    let now = Instant::now();
    println!("{}", now.elapsed().as_millis());
    use std::mem;
    let payload = crate::client_payload(LARGE_REQ_SIZE);
    let req = SimpleRequest {
        response_type: PayloadType::Compressable as i32,
        response_size: LARGE_RSP_SIZE,
        payload: Some(payload),
        ..Default::default()
    };

    let result = client.unary_call(Request::new(req)).await;

    // assertions.push(test_assert!(
    //     "call must be successful",
    //     result.is_ok(),
    //     format!("result={:?}", result)
    // ));

    if let Ok(response) = result {
        let body = response.into_inner();
        let payload_len = body.payload.as_ref().map(|p| p.body.len()).unwrap_or(0);
        println!("payload length: {:?} Bytes", payload_len);
        println!("{}", now.elapsed().as_millis());

        assertions.push(test_assert!(
            "body must be 3140159 bytes",
            payload_len == LARGE_RSP_SIZE as usize,
            format!("mem::size_of_val(&body)={:?}", mem::size_of_val(&body))
        ));
    }
}