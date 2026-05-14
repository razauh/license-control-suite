# license-control-suite

Unified Rust crate that merges four legacy domains into a single module-structured codebase:

- `shared_contracts`
- `admin_dashboard`
- `auth_core`
- `user_reg` (core + tauri adapters + licensing worker)

This repository is currently **crate-first** (library + binary skeleton) and not a full `src-tauri` shell project yet.

## Graphify Snapshot

Latest Graphify run (`graphify-out/GRAPH_REPORT.md`) reported:

- **550 nodes**
- **942 edges**
- **25 communities**
- extraction mix: **75% EXTRACTED**, **25% INFERRED**

This README is aligned with that graph and the current source tree.

## Repository Structure

```text
src/
  lib.rs
  main.rs
  modules/
    shared_contracts/
    admin_dashboard/
    auth_core/
    user_reg/
      auth_licensing_core/
      auth_licensing_tauri/
      licensing_worker/
tests/
  baseline/
  contracts/
  integration/
  ipc/
  regression/
docs/
  baseline/
  migration/
fixtures/
scripts/
```

## High-Level Architecture

```mermaid
flowchart LR
  SC[shared_contracts]
  AD[admin_dashboard]
  AC[auth_core]
  URC[user_reg::auth_licensing_core]
  URT[user_reg::auth_licensing_tauri]
  URW[user_reg::licensing_worker]

  SC --> AD
  SC --> AC
  SC --> URC
  URC --> URT
  URW --> URC
  URT --> URC
```

## Module Boundary Map

```mermaid
flowchart TB
  subgraph Core["Core Domain Layer"]
    URC[user_reg::auth_licensing_core]
    SC[shared_contracts]
    AC[auth_core]
    AD[admin_dashboard]
  end

  subgraph Adapter["Adapter / Edge Layer"]
    URT[user_reg::auth_licensing_tauri]
    HTTP[HttpWorkerClient]
    PERSIST[KeychainSecretStore + AppDataStateStore]
  end

  subgraph Worker["Worker Runtime"]
    URW[user_reg::licensing_worker]
    STORE[In-memory/store state]
    AUDIT[Audit events]
  end

  URT --> URC
  URT --> HTTP
  URT --> PERSIST
  URW --> URC
  URW --> STORE
  URW --> AUDIT
  SC --> URC
  SC --> AD
  SC --> AC
```

## Tauri Command Surface

The unified command inventory is fixed at six commands:

- `activate_license`
- `validate_session`
- `request_device_reset`
- `get_device_reset_status`
- `clear_local_session`
- `get_auth_state`

```mermaid
flowchart LR
  UI[Frontend / Invoke Caller]
  CMD[auth_licensing_tauri::commands]
  SVC[AuthService]

  UI -->|invoke(command)| CMD
  CMD --> SVC
```

## Command-to-Core Flow

```mermaid
sequenceDiagram
  participant C as Command Caller
  participant T as Tauri Command
  participant S as AuthService
  participant W as WorkerClient
  participant K as SecretStore
  participant L as LocalStateStore

  C->>T: activate_license(license_key)
  T->>S: activate_license(...)
  S->>W: activate(request)
  W-->>S: ActivationOutcome
  S->>K: put_device_keypair, put_license_key, put_access_token
  S->>L: save_session_state(Licensed)
  S-->>T: ActivationView
  T-->>C: Result<ActivationView>
```

## Device Reset Flow

```mermaid
sequenceDiagram
  participant C as Command Caller
  participant S as AuthService
  participant W as WorkerClient
  participant L as LocalStateStore
  participant K as SecretStore

  C->>S: request_device_reset(email, receipt_ref)
  S->>W: request_device_reset(DeviceResetRequest)
  W-->>S: DeviceResetStatus::Pending
  S->>L: save_reset_status(pending)
  S->>L: save_session_state(ResetPending)
  S-->>C: DeviceResetView

  C->>S: get_device_reset_status(request_id)
  S->>W: get_device_reset_status(request_id)
  W-->>S: Approved/Rejected/Expired
  S->>L: save_reset_status(status)
  alt Approved
    S->>K: clear_session_secrets()
  end
  S->>L: save_session_state(mapped)
  S-->>C: DeviceResetView
```

## Storage and Secret Responsibilities

```mermaid
flowchart LR
  SVC[AuthService]
  SEC[SecretStore]
  KEY[KeychainSecretStore]
  LOC[LocalStateStore]
  APP[AppDataStateStore]
  JSON[(session_state.json / reset_status.json)]
  OS[(OS Keyring)]

  SVC --> SEC
  SEC --> KEY
  KEY --> OS

  SVC --> LOC
  LOC --> APP
  APP --> JSON
```

## Worker Domain Flow

```mermaid
flowchart TB
  ACT[activate request]
  VAL[validate session]
  RST[request reset]
  DEC[admin approve/reject]
  AUD[audit append]

  ACT --> AUD
  VAL --> AUD
  RST --> AUD
  DEC --> AUD
```

## Trait-Oriented Core Interfaces

`user_reg::auth_licensing_core` is designed around injectable traits:

- `WorkerClient`
- `SecretStore`
- `LocalStateStore`
- `DeviceIdentityProvider`
- `Clock`

```mermaid
classDiagram
  class AuthService
  class WorkerClient
  class SecretStore
  class LocalStateStore
  class DeviceIdentityProvider
  class Clock

  AuthService --> WorkerClient
  AuthService --> SecretStore
  AuthService --> LocalStateStore
  AuthService --> DeviceIdentityProvider
  AuthService --> Clock
```

## Test Topology

```mermaid
flowchart LR
  B[tests/baseline]
  C[tests/contracts]
  I[tests/integration]
  P[tests/ipc]
  R[tests/regression]
  M[src/modules/*]

  B --> M
  C --> M
  I --> M
  P --> M
  R --> M
```

## Verification Pipeline

Primary orchestrator:

- `scripts/run_full_verification_logged.sh`

It executes ordered checks and writes per-command logs under `logs/verification_<timestamp>/`.

```mermaid
flowchart TD
  A[Docs presence checks]
  B[Baseline discovery checks]
  C[Legacy metadata checks]
  D[Unified cargo checks]
  E[Targeted test slices]
  F[Full regression]
  G[Packaging + tauri smoke]
  H[Summary log]

  A --> B --> C --> D --> E --> F --> G --> H
```

## Build and Test

### Fast local checks

```bash
cargo check
cargo test
```

### Full logged verification

```bash
bash scripts/run_full_verification_logged.sh
```

## Current Tauri Shell Status

This repo currently has command modules and IPC contracts, but no complete Tauri app shell (`src-tauri/tauri.conf.*`).

Implications:

- Rust module tests/builds can pass.
- Tauri packaging smoke commands can report `BLOCKED` until shell/config/capability files are added.

## Key Docs

- `docs/migration/final_regression_report.md`
- `docs/migration/final_acceptance_checklist.md`
- `docs/migration/handoff_summary.md`
- `../docs/unified_merge_unresolved_issues.md`
- `../docs/unified_merge_verification_runbook.md`

## Graph Artifacts

- `graphify-out/graph.json`
- `graphify-out/graph.html`
- `graphify-out/GRAPH_REPORT.md`

