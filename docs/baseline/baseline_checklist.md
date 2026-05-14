# TC-01 Baseline Checklist

## Task

- Task ID: `TC-01`
- Title: `Original Build and Test Baseline`
- Date: `2026-05-14`

## Scope

Capture baseline run evidence for the four original repositories before migration.

## Test-First Artifact

- Validation script: `docs/baseline/original-runs/validate_baseline_logs.sh`
- Expected behavior: fail when required baseline logs or required metadata fields are missing.

## Required Baseline Logs

- [x] `docs/baseline/original-runs/shared-contracts.log`
- [x] `docs/baseline/original-runs/admin-dashboard.log`
- [x] `docs/baseline/original-runs/auth-core.log`
- [x] `docs/baseline/original-runs/user-reg.log` (placeholder created; run output still pending)

## Required Metadata Per Log

- [x] `Command:`
- [x] `Working Directory:`
- [x] `Exit Code:`
- [x] `Timestamp:`

## Current Evidence Status

- `shared-contracts`: baseline captured from legacy log artifact.
- `admin-dashboard`: baseline captured from legacy log artifact.
- `auth-core`: baseline captured from legacy log artifact.
- `user-reg`: pending manual run output capture.

## Manual Commands Required (not run by agent)

```bash
cd /home/pc/Downloads/inf/plan/shared-contracts && cargo test
cd /home/pc/Downloads/inf/plan/admin-dashboard && cargo test
cd /home/pc/Downloads/inf/plan/auth-core && cargo test
cd /home/pc/Downloads/inf/plan/user-reg && cargo test --workspace
```

## Notes

- No Cargo manifests were edited.
- No legacy source/test files were edited.
- This task is implementation-complete but not execution-verified.
