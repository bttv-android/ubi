use std::fs::File;
use std::path::Path;
use zip::ZipArchive;

mod err;

use err::*;

pub fn prepare_mock_aar(
    aar_path: impl AsRef<Path>,
    dx_path: &str,
    baksmali_path: &str,
) -> PrepareAARResult<()> {
    let classes_jar_path = extract_classes_jar(aar_path)?;
    let dex_path = run_dx(dx_path, classes_jar_path)?;
    let smali_dir_path = run_baksmali(baksmali_path, dex_path)?;
    Ok(())
}

fn extract_classes_jar(aar_path: impl AsRef<Path>) -> PrepareAARResult<&'static str> {
    let aar_file = File::open(aar_path)?;
    let mut archive = ZipArchive::new(aar_file)?;
    let mut classes = archive.by_name("classes.jar")?;

    let out_path = "/tmp/bttv-ubi-classes.jar";
    let mut outfile = File::create(out_path)?;

    std::io::copy(&mut classes, &mut outfile)?;

    Ok(out_path)
}

fn run_dx(dx_path: &str, classes_jar_path: &str) -> PrepareAARResult<&'static str> {
    let out_file = "/tmp/bttv-ubi-dx.dex";
    let output = std::process::Command::new("sh")
        .arg(dx_path)
        .arg("--dex")
        .arg(format!("--output={}", out_file))
        .arg(classes_jar_path)
        .output()?;

    if output.status.success() {
        Ok(out_file)
    } else {
        Err(PrepareAARError::DXErr(output))
    }
}

fn run_baksmali(baksmali_path: &str, dex_path: &str) -> PrepareAARResult<&'static str> {
    let out_dir = "/tmp/bttv-ubi-smali";
    let output = std::process::Command::new("java")
        .arg("-jar")
        .arg(baksmali_path)
        .arg("d")
        .arg(dex_path)
        .arg("-o")
        .arg(out_dir)
        .output()?;

    if output.status.success() {
        Ok(out_dir)
    } else {
        Err(PrepareAARError::DXErr(output))
    }
}
