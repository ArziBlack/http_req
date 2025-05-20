use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use zip::ZipArchive;

pub fn unzip_file(zip_path: &str, extract_to: &str) -> io::Result<()> {
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    fs::create_dir_all(extract_to)?;
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => Path::new(extract_to).join(path),
            None => continue,
        };

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
            continue;
        }

        if let Some(parent) = outpath.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut outfile = File::create(&outpath)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        outfile.write_all(&contents)?;
    }
    Ok(())
}