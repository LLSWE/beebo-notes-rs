pub mod beebo {
    tonic::include_proto!("beebo.v1");
}

mod cron;
mod grpc;

use crate::cron::run_cron;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("CRON RODANDO ...");
    run_cron().await?;

    tokio::signal::ctrl_c().await?;
    println!("DESLIGANDO BEEBO...");
    Ok(())
}
