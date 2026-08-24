# BowserAI Autonomous Engineering

BowserAI/Nessy is an autonomous software and runtime system. Repository validation, deployment configuration, runtime discovery, service installation, provider registration, recovery, and promotion are designed to execute automatically without a human-operated setup or configuration phase.

## Autonomous contract

- Source changes are formatted, compiled, tested, linted, audited, licensed, packaged, and checked for SBOM/reproducibility automatically.
- Runtime configuration is generated from environment, discovered capabilities, persisted state, and provider metadata rather than requiring hand-authored configuration.
- Kairos is a first-class always-on runtime target and is discovered alongside local, peer, and cloud runtimes.
- Inference providers are registered through the capability layer and selected dynamically by capability, health, priority, and availability.
- Persistent state, cache, secrets, logs, and runtime configuration are provisioned automatically under the Bowser home directory.
- The bootstrap installer builds the complete workspace, installs the daemon, generates its runtime environment, and enables the platform service when the host service manager is available.
- Failure recovery is automatic: unhealthy runtimes are removed from active selection, queued work remains addressable, and alternate capable runtimes may be selected.
- Repository automation may validate, repair, promote, and release changes without a manual approval step when repository policy permits autonomous writes.
- The project license is Apache License 2.0 only.

## Bootstrap

The supported bootstrap path is:

```bash
./scripts/bootstrap.sh
```

The bootstrap process creates the Bowser runtime directories, builds the entire workspace, installs `bowserd`, generates the runtime environment, discovers the configured Kairos endpoint, and installs/enables a persistent user service when supported by the operating system.

## Runtime configuration

The generated environment is intentionally minimal. Providers and runtimes are represented as capabilities and may be discovered or registered by the running system. `KAIROS_URL` may override the default Kairos endpoint without editing source files.

## Failure handling

A failed validation or runtime operation is handled by automation. Validation failures trigger corrective automation where available and rerun the complete gate. Runtime failures trigger health-state changes, retry/backoff, alternate-provider selection, and persisted task recovery. The system does not require an operator to manually select a fallback runtime or reconstruct configuration.
