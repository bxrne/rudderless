# rudderless

> [!IMPORTANT]
> In progress. Experimental.

Rudderless is an experiment in giving a small cluster useful workload orchestration without the weight of a full platform.

It uses a different model of state from Kubernetes. State is local, replicated peer to peer, and converged by the nodes. There is no single control plane to lose.

It runs containers. It aims for the practical subset of orchestration: specs, placement, restarts, logs and status.

## What it is not

- A cloud control plane
- A service mesh
- A platform for every workload shape

## Roadmap to 0.1.0

- One node runs one container workload
- Restart policy and logs
- Two nodes place work deterministically
- Health checks and failure recovery
- Status and metrics for operators
- Installable daemon and CLI
- Three-node demo survives a node death

## Development

```
cargo run -p rdld
cargo run -p rdlctl
cargo test --workspace
```
