// Copyright 2026 Synthicsoft Labs LLC
// Licensed under the Apache License, Version 2.0 and the MIT License.

use std::sync::Arc;
use tokio::signal;
use tracing::{info, warn};
use turtle::Scheduler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .json()
        .init();

    let scheduler = Arc::new(Scheduler::default());
    info!("bowserd started");

    signal::ctrl_c().await?;
    warn!("shutdown signal received; beginning graceful drain");
    drop(scheduler);
    info!("bowserd stopped");
    Ok(())
}
