#![doc = include_str!("../readme.md")]

// Enable log macros
#[macro_use]
extern crate log;

mod app;
mod client;
mod err;
mod fluff;
mod gem_bytes;
mod gem_call;
mod interactive;

pub use app::App;
pub use client::Client;
pub use err::AppErr;
pub use fluff::Fluff;
pub use gem_bytes::GemBytes;
pub use gem_call::{GemCall, Static};

/// Procedural macro that must be used in implementations of [`GemBytes`].
pub use async_trait::async_trait;

/// Exported from [`trotter`].
pub use trotter::Status;
