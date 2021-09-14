use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
use std::{thread, time};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let mut response;

    let mut count = 0;

    loop {
        count = count + 1;
        let request = tonic::Request::new(HelloRequest {
            name: "Tonic".into(),
        });

        response = client.say_hello(request).await?;
        let ten_millis = time::Duration::from_millis(1000);
        thread::sleep(ten_millis);

        if count == 100 {
            break;
        }
    }

    println!("RESPONSE={:?}", response);

    Ok(())
}
