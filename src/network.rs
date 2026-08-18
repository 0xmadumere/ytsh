use std::path::PathBuf;
use std::fs;
use std::io::{ self, BufReader };
use crate::config::*;
use crate::utility::*;

fn extract_zip(
    name: &str,
    path: &PathBuf,
    zip_path: &PathBuf
) -> Result<(), Box<dyn std::error::Error>> {
    let binary_name = format!("{}{}", name, std::env::consts::EXE_SUFFIX);

    let file = fs::File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    let mut _found = false;
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        if entry.name().ends_with(&binary_name) {
            let mut dest_file = fs::File::create(path)?;
            io::copy(&mut entry, &mut dest_file)?;
            _found = true;
            break;
        }
    }

    if !_found {
        return Err(format!("{} was not found in the archive", binary_name).into());
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms)?;
    }

    fs::remove_file(zip_path)?;

    Ok(())
}


fn extract_tar_xz(
    name: &str,
    path: &PathBuf,
    archive_path: &PathBuf
) -> Result<(), Box<dyn std::error::Error>> {
    let binary_name = format!("{}{}", name, std::env::consts::EXE_SUFFIX);

    let file = fs::File::open(archive_path)?;
    let decompressed = xz2::read::XzDecoder::new(file);
    let mut archive = tar::Archive::new(decompressed);

    let mut found = false;
    for entry in archive.entries()? {
        let mut entry = entry?;
        let entry_path = entry.path()?.into_owned();

        let matches = entry_path
            .file_name()
            .map(|n| n.to_string_lossy() == binary_name)
            .unwrap_or(false);

        if matches {
            let mut dest_file = fs::File::create(path)?;
            io::copy(&mut entry, &mut dest_file)?;
            found = true;
            break;
        }
    }

    if !found {
        return Err(format!("{} was not found in the archive", binary_name).into());
    }

    chmod_executable(path)?;
    fs::remove_file(archive_path)?;

    Ok(())
}

pub fn download_dep(dep: &Dependency) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(&dep.download_url)?;

    if !response.status().is_success() {
        return Err(format!("HTTP error status: {}", response.status()).into());
    }

    if let Some(parent) = dep.path.parent() {
        fs::create_dir_all(parent)?;
    }

    let is_archive = dep.archive_type != ArchiveType::None;

    let mut output_path = fs::File::create(if is_archive { &dep.zip_path } else { &dep.path })?;
    // let mut output_path = fs::File::create(if dep.is_zip { &dep.zip_path } else { &dep.path })?;

    // Set custom buffer to 256 KiB (256 * 1024 bytes)
    let mut reader = BufReader::with_capacity(256 * 1024, response);
    io::copy(&mut reader, &mut output_path)?;

    // if dep.is_zip {
    //     println!("Extraxting {} from zip.", dep.name);
    //     extract_zip(&dep.name, &dep.path, &dep.zip_path)?;
    //     println!("Extracted {}.", dep.name);
    // } else {
    //     #[cfg(unix)]
    //     {
    //         use std::os::unix::fs::PermissionsExt;
    //         let mut perms = fs::metadata(&dep.path)?.permissions();
    //         perms.set_mode(0o755);
    //         fs::set_permissions(&dep.path, perms)?;
    //     }
    // }
    match dep.archive_type {
        ArchiveType::Zip => {
            println!("Extraxting {} from zip.", dep.name);
            extract_zip(&dep.name, &dep.path, &dep.zip_path)?;
            println!("Extracted {}.", dep.name);
        }
        ArchiveType::TarXz => {
            println!("Extracting {} from tar.xz.", dep.name);
            extract_tar_xz(&dep.name, &dep.path, &dep.zip_path)?;
            println!("Extracted {}.", dep.name);
        }
        ArchiveType::None => {
            if cfg!( target_os = "linux") {
                chmod_executable(&dep.path)?;
            }
        }
    }

    Ok(())
}
