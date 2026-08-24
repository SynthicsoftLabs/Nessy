// Copyright 2026 Synthicsoft Labs LLC
// Licensed under the Apache License, Version 2.0.

use std::sync::Arc;
use tracing::info;
use turtle::Scheduler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .json()
        .init();

    let scheduler = Arc::new(Scheduler::default());
    let kairos = kairos::KairosClient::from_environment()?;
    info!(
        kairos_endpoint = kairos.base_url(),
        "autonomous runtime initialized"
    );

    let runtime = capability::RuntimeDescriptor {
        id: uuid::Uuid::new_v4(),
        name: "kairos".into(),
        capabilities: [
            capability::Capability::Inference,
            capability::Capability::RemoteRuntime,
            capability::Capability::PeerMesh,
        ]
        .into_iter()
        .collect(),
        endpoint: Some(kairos.base_url().to_owned()),
        priority: 100,
        healthy: true,
    };

    info!(runtime = %runtime.name, fingerprint = %runtime.fingerprint(), "runtime registered");
    info!(queued_tasks = 0, "bowserd ready for autonomous scheduling");

    tokio::signal::ctrl_c().await?;
    info!("shutdown signal received; beginning graceful drain");
    drop(scheduler);
    info!("bowserd stopped");
    Ok(())
}
