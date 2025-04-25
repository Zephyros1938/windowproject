use std::ffi::CString;
use std::{env, ffi::OsString};
use std::{fs::File, io::Read};
pub mod cube;
pub mod mesh;
pub mod model;

use crate::cstr;

pub static HOME: std::sync::LazyLock<OsString> = std::sync::LazyLock::new(|| {
    env::var_os("HOME")
        .ok_or("The HOME environment variable is not set")
        .unwrap()
});

#[allow(dead_code)]
pub fn get_assets_dir() -> std::path::PathBuf {
    let assets_dir = std::path::Path::new(&HOME.as_os_str()).join(concat!(
        ".local/share/",
        env!("CARGO_PKG_NAME"),
        "/assets"
    ));
    assets_dir
}

#[allow(dead_code)]
pub fn get_asset(path: &str) -> Result<std::fs::File, String> {
    let assets_dir = get_assets_dir();
    let asset_path = assets_dir.join(path);
    if asset_path.is_file() {
        Ok(File::open(&asset_path).unwrap())
    } else {
        Err(format!("Asset not found: {}", asset_path.display()))
    }
}
#[allow(dead_code)]
pub fn get_asset_path(path: &str) -> Result<String, String> {
    let assets_dir = get_assets_dir();
    let asset_path = assets_dir.join(path);
    if asset_path.is_file() {
        Ok(asset_path
            .as_path()
            .to_str()
            .expect("Could not convert asset path to string.")
            .to_string())
    } else {
        Err(format!("Asset not found: {}", asset_path.display()))
    }
}

#[allow(dead_code)]
pub fn get_asset_path_cstr(path: &str) -> Result<CString, String> {
    let assets_dir = get_assets_dir();
    let asset_path = assets_dir.join(path);
    if asset_path.is_file() {
        let asset_str = asset_path
            .as_path()
            .to_str()
            .expect("Could not convert asset path to string.");
        Ok(CString::new(asset_str).unwrap())
    } else {
        Err(format!("Asset not found: {}", asset_path.display()))
    }
}

#[allow(dead_code)]
#[deprecated = "This function is unstable and as of 4/24/2025 9:30PM does not work."]
pub fn read_asset_to_cstr(path: &str) -> std::ffi::CString {
    let mut s = String::new();
    let _ = get_asset(path).unwrap().read_to_string(&mut s);
    cstr!(s)
}
