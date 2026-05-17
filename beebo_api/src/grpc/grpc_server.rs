use std::sync::Arc;

use tonic::transport::Server;

use crate::{
    grpc::grpc_service::{BeeboGrpcService, beebo::call_beebo_server::CallBeeboServer},
    model::AppState,
};

pub async fn grpc_server(app_state: Arc<AppState>) {
    let addr = "127.0.0.1:50051".parse().unwrap();

    let grpc_service = BeeboGrpcService { state: app_state };

    Server::builder()
        .add_service(CallBeeboServer::new(grpc_service))
        .serve(addr)
        .await
        .unwrap();
}
