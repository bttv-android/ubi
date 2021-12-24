use std::{fs, path};
use ubi_core::UbiArgs;
use zip::ZipArchive;

pub fn handle_mod_dir(args: &UbiArgs) -> &'static str {
    trace!("handle_mod_dir()");

    let path = path::Path::new(&args.mod_dir);
    let aar_path = path.join("./twitch/build/outputs/aar/twitch-debug.aar");

    let f = match fs::File::open(aar_path) {
        Ok(f) => f,
        Err(e) => panic!(
            "could not open twitch-debug.aar: {}, please run ./buildsource first",
            e
        ),
    };
    let mut archive = match ZipArchive::new(f) {
        Err(e) => panic!("{}", e),
        Ok(a) => a,
    };

    trace!("handle_mod_dir(): loaded archive");

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        if file.name() != "classes.jar" {
            continue;
        }
        let mut outfile = fs::File::create("/tmp/bttv-ubi-classes.jar").unwrap();
        match std::io::copy(&mut file, &mut outfile) {
            Err(e) => panic!("could not extract classes.jar: {}", e),
            _ => (),
        }
        trace!("wrote file");
        break;
    }

    trace!("running dx");
    let output = std::process::Command::new("sh")
        .arg(&args.dx_path)
        .arg("--dex")
        .arg("--output=/tmp/bttv-ubi-dx.dex")
        .arg("/tmp/bttv-ubi-classes.jar")
        .output()
        .expect("");

    debug!("dx output: {:#?}", output);

    if !output.status.success() {
        panic!("dx failed to build classes (see debug logs for more)");
    }

    trace!("running smali");
    let output = std::process::Command::new("java")
        .arg("-jar")
        .arg(&args.baksmali_path)
        .arg("d")
        .arg("/tmp/bttv-ubi-dx.dex")
        .arg("-o")
        .arg("/tmp/bttv-ubi-smali")
        .output()
        .expect("");

    debug!("smali output: {:#?}", output);

    if !output.status.success() {
        panic!("failed to baksmali classes (see debug logs for more)");
    }

    return "/tmp/bttv-ubi-smali";
}
