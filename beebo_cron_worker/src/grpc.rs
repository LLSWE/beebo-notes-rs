use tonic::Request;

pub mod beebo {
    tonic::include_proto!("beebo.v1");
}

use beebo::{WriteBeeboRequest, call_beebo_client::CallBeeboClient};

pub async fn rpc_beebo() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CallBeeboClient::connect("http://127.0.0.1:50051").await?;

    let request = Request::new(WriteBeeboRequest {});

    let response = client.write_beebo(request).await?;

    println!("Resposta: {:?}", response.into_inner());

    Ok(())
}
