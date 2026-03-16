use std::fs;
use std::io;
use std::path::Path;

pub fn unpack_archive(archive: &Path, destination: &Path) -> io::Result<()> {
    fs::create_dir_all(destination)?;
    let name = archive
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default();

    if name.ends_with(".zip") {
        unpack_zip(archive, destination)?;
        return Ok(());
    }
    if name.ends_with(".tar.gz") || name.ends_with(".tgz") {
        unpack_tar_gz(archive, destination)?;
        return Ok(());
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        format!("unsupported archive format `{}`", archive.display()),
    ))
}

fn unpack_zip(archive: &Path, destination: &Path) -> io::Result<()> {
    let file = fs::File::open(archive)?;
    let mut zip = zip::ZipArchive::new(file)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;

    for index in 0..zip.len() {
        let mut entry = zip
            .by_index(index)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        let path = destination.join(entry.mangled_name());
        if entry.is_dir() {
            fs::create_dir_all(&path)?;
            continue;
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut outfile = fs::File::create(&path)?;
        io::copy(&mut entry, &mut outfile)?;
    }

    Ok(())
}

fn unpack_tar_gz(archive: &Path, destination: &Path) -> io::Result<()> {
    let file = fs::File::open(archive)?;
    let decoder = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);
    archive.unpack(destination)?;
    Ok(())
}
