//! # Wasmtime's WASI Preview 2 Implementation
//!
//! Welcome to the (new!) WASI implementation from the Wasmtime team. The goal
//! of this implementation is to support WASI Preview 2 via the Component
//! Model, as well as to provide legacy Preview 1 host support with an adapter
//! that is implemented in terms of the Preview 2 interfaces.
//!
//! Presently, this crate is experimental. We don't yet recommend you use it
//! in production. Specifically:
//! * it does not yet support a synchronous rust embedding
//! * polling and streams need a redesign. IO that currently should be
//! non-blocking may be blocking. poll probably doesn't work at all.
//! * its internal organization could use some love
//! * the wit files in tree describing preview 2 are not faithful to the
//! standards repos
//!
//! Once these issues are resolved, we expect to move this namespace up to the
//! root of the wasmtime-wasi crate, and move its other exports underneath a
//! `pub mod legacy` with an off-by-default feature flag, and after 2
//! releases, retire and remove that code from our tree.

// TODO: make all of these mods private
pub mod clocks;
mod ctx;
mod error;
pub(crate) mod filesystem;
pub mod pipe; // pipe can remain a module
mod poll;
#[cfg(feature = "preview1-on-preview2")]
pub mod preview1;
pub mod preview2;
pub mod random;
pub mod stdio;
pub mod stream;
pub mod table;
pub mod wasi;

pub use self::clocks::{WasiClocks, WasiMonotonicClock, WasiWallClock};
pub use self::ctx::{WasiCtx, WasiCtxBuilder, WasiView};
pub use self::error::I32Exit;
pub use self::filesystem::{DirPerms, FilePerms};
pub use self::poll::{HostPollable, TablePollableExt};
#[cfg(unix)]
pub use self::stream::AsyncFdStream;
pub use self::stream::{
    AsyncReadStream, AsyncWriteStream, HostInputStream, HostOutputStream, StreamState,
    TableStreamExt,
};
pub use self::table::{Table, TableError};
pub use cap_fs_ext::SystemTimeSpec;
pub use cap_rand::RngCore;
