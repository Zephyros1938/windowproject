use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=src/assets/*/**=true");

    let home = env::var_os("HOME").ok_or("The HOME environment variable is not set")?;
    let assets_dir = Path::new(&home.as_os_str()).join(concat!(
        ".local/share/",
        env!("CARGO_PKG_NAME"),
        "/assets"
    ));

    fs::create_dir_all(&assets_dir)?;

    let src_assets_dir = Path::new("src/assets");
    if src_assets_dir.exists() {
        println!("Copying assets to {}", assets_dir.display());

        for entry in fs::read_dir(src_assets_dir)? {
            let entry = entry?;
            let asset_path = entry.path();
            let target_path = assets_dir.join(asset_path.file_name().unwrap());

            if asset_path.is_dir() {
                copy_dir_recursive(&asset_path, &target_path)?;
            } else {
                fs::copy(&asset_path, &target_path)?;
                println!(
                    "Copied file: {} to {}",
                    asset_path.display(),
                    target_path.display()
                );
            }
        }
    } else {
        println!("Assets directory not found: {}", src_assets_dir.display());
    }

    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let target = dst.join(entry.file_name());
        if path.is_dir() {
            println!(
                "Entering directory {} from {}",
                &target.display(),
                &path.display()
            );
            copy_dir_recursive(&path, &target)?;
            println!("Exited directory {}", &path.display());
        } else {
            fs::copy(&path, &target)?;
            println!("Copied file: {} to {}", path.display(), target.display());
        }
    }
    Ok(())
}
