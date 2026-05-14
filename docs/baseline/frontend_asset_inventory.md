# TC-03 Frontend and Asset Inventory

- Task ID: `TC-03`
- Date: `2026-05-14`

## Search Scope

Legacy roots searched:

- `/home/pc/Downloads/inf/plan/shared-contracts`
- `/home/pc/Downloads/inf/plan/admin-dashboard`
- `/home/pc/Downloads/inf/plan/auth-core`
- `/home/pc/Downloads/inf/plan/user-reg`

Checked for:

- `src-tauri/`
- `tauri.conf.*`
- `capabilities/`
- `package.json`
- frontend roots (`frontend/`, `public/`, `assets/`)

## Findings

- No confirmed frontend app shell in analyzed legacy repositories.
- No confirmed `tauri.conf.*` file in analyzed legacy repositories.
- No confirmed `src-tauri/` directory in analyzed legacy repositories.
- No confirmed frontend package manifest (`package.json`) in analyzed legacy repositories.
- `.html` files found during search are Graphify outputs (`graphify-out/graph.html`), not app frontend sources.
- Tauri integration code is present only as Rust library module sources in:
  - `/home/pc/Downloads/inf/plan/user-reg/crates/auth-licensing-tauri/src/`

## Source-Verification Status

No confirmed frontend app shell or asset pipeline is currently available from the four legacy roots.  
This remains a **requires source verification** item for later integration cards.
