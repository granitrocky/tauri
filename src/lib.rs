#![allow(clippy::single_component_path_imports)]

pub use tauri_core::*;

#[cfg(feature = "dynamic")]
#[allow(unused_imports)]
use tauri_dylib;
