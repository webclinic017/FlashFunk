#![allow(clippy::mutex_atomic)]
#![allow(clippy::type_complexity)]
#![allow(
dead_code,
unused_variables,
non_camel_case_types,
non_snake_case,
non_upper_case_globals,
unused_imports
)]

/// In this crate, it provides the data_type and constants.
/// Also, provide the most useful interface like ctp or ctp_mini
///
#[macro_use]
extern crate bitflags;

use std::env::var;
use std::path::PathBuf;

pub mod constant;
pub mod context;
pub mod data_type;
pub mod interface;

#[cfg(feature = "ctp")]
mod ctp;


#[cfg(feature = "ctpmini")]
mod ctpmini;

#[cfg(feature = "ctp")]
pub use ctp::mdapi::CtpMdApi;
#[cfg(feature = "ctp")]
pub use ctp::tdapi::CtpTdApi;
use std::fs;

pub mod types;
pub mod util;
mod c_func;

#[cfg(not(target_os = "windows"))]
fn os_path(target: &str) -> PathBuf {
    let path = PathBuf::from(format!(
        "{}",
        var("HOME").unwrap()
    ));
    let path = path.join(".HFQ");
    if !path.exists() {
        fs::create_dir(path.clone());
    }
    let p = path.join(target);
    if !p.exists() {
        fs::create_dir(p.clone());
    }
    p
}

#[cfg(target_os = "windows")]
fn os_path(target: &str) -> PathBuf {
    let path = PathBuf::from(format!(
        "{}{}",
        var("HOMEDRIVE").unwrap(),
        var("HOMEPATH").unwrap()
    ));
    let path = path.join(".HFQ");
    if !path.exists() {
        fs::create_dir(path.clone());
    }
    let p = path.join(target);
    if !p.exists() {
        fs::create_dir(p.clone());
    }
    p
}

fn get_interface_path(interface: &str) -> PathBuf {
    let home = os_path(interface);
    home.join("bindings.rs")
}
