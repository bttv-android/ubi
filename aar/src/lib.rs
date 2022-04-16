use std::fs::File;
use std::path::Path;
use zip::ZipArchive;

mod err;

use err::PrepareAARError;

pub fn prepare_mock_aar(aar_path: impl AsRef<Path>) -> Result<(), PrepareAARError> {
    let _classes_jar_path = extract_classes_jar(aar_path)?;
    Ok(())
}

fn extract_classes_jar(aar_path: impl AsRef<Path>) -> Result<&'static str, PrepareAARError> {
    let aar_file = File::open(aar_path)?;
    let mut archive = ZipArchive::new(aar_file)?;
    let mut classes = archive.by_name("classes.jar")?;

    let out_path = "/tmp/bttv-ubi-classes.jar";
    let mut outfile = File::create(out_path)?;

    std::io::copy(&mut classes, &mut outfile)?;

    Ok(out_path)
}
