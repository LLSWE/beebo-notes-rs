use std::sync::Arc;

pub use tonic::{Request, Response, Status};

pub mod beebo {
    tonic::include_proto!("beebo.v1");
}

use beebo::{WriteBeeboRequest, WriteBeeboResponse, call_beebo_server::CallBeebo};

use crate::{model::AppState, services::exec_beebo};

pub struct BeeboGrpcService {
    pub state: Arc<AppState>,
}

#[tonic::async_trait]
impl CallBeebo for BeeboGrpcService {
    async fn write_beebo(
        &self,
        _: Request<WriteBeeboRequest>,
    ) -> Result<Response<WriteBeeboResponse>, Status> {
        let msg = exec_beebo(&self.state)
            .await
            .map_err(|_| Status::internal("failed to execute beebo"))?;

        Ok(Response::new(WriteBeeboResponse { message: msg }))
    }
}
