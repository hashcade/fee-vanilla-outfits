//! Original character battle clothing, without custom save data.

pub mod catalog;
#[cfg(target_os = "switch")]
mod native;
#[cfg(target_os = "switch")]
pub(crate) mod plugin;
pub mod policy;
pub mod profile;
pub mod scope;
