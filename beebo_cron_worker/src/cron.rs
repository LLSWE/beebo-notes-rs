use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

use crate::grpc::rpc_beebo;

pub async fn run_cron() -> Result<(), JobSchedulerError> {
    let mut sched = JobScheduler::new().await?;

    sched
        .add(Job::new_async("0 0 */12 * * *", move |_uuid, mut _l| {
            Box::pin(async move {
                println!("[CRON] Chamando beebo via gRPC");
                rpc_beebo()
                    .await
                    .map_err(|e| eprintln!("Error sending RPC : {e}"));
            })
        })?)
        .await?;

    sched.start().await?;

    Ok(())
}
