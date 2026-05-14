//! Tauri integration for auth and licensing.
//!
//! This module owns IPC DTOs and production adapters. Business rules live in
//! `auth_licensing_core`.

mod commands;
mod http_client;
mod persistence;

pub use commands::*;
pub use http_client::*;
pub use persistence::*;
