#[macro_use]
extern crate log;
extern crate env_logger;
extern crate git2;
extern crate walkdir;
extern crate zip;

mod git;
mod mod_dir;
mod smali;

use std::env;
use std::process;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    env_logger::init();

    trace!("Logger initialized, getting args");
    let (baksmali_path, dx_path, mod_dir, disass_dir, no_diff) = parse_args();

    let mod_smali = mod_dir::handle_mod_dir(dx_path, baksmali_path, &mod_dir);

    let mut smali_files = get_all_smali_files(mod_smali.to_string());
    trace!("smali files: {:#?}", smali_files);

    if !no_diff {
        let filter = git::diff(&disass_dir);
        trace!("changed files: {:#?}", filter);
        smali_files = smali_files
            .into_iter()
            .filter(|path| filter.contains(path))
            .collect();
        trace!("smali files: {:#?}", smali_files);
    }

    // TODO: smali diff using smali_files
    unimplemented!();
}

/** Returns (mod dir, disass dir) or kills process with error message */
fn parse_args() -> (String, String, String, String, bool) {
    let mut no_diff = false;
    let mut help_page = false;
    let mut mod_dir = None;
    let mut disass_dir = None;
    let mut dx_path = None;
    let mut baksmali_path = None;

    let mut neg = 0;

    for (i, arg) in env::args().enumerate() {
        if arg == "-h" || arg == "--help" {
            help_page = true;
            neg += 1;
        } else if arg == "--no-diff" {
            no_diff = true;
            neg += 1;
        }

        let i = i - neg;
        if i == 1 {
            baksmali_path = Some(arg);
        } else if i == 2 {
            dx_path = Some(arg);
        } else if i == 3 {
            mod_dir = Some(arg);
        } else if i == 4 {
            disass_dir = Some(arg);
        }
    }

    trace!("parse_args: help_page: {}", help_page);
    trace!("parse_args: baksmali_path: {:?}", baksmali_path);
    trace!("parse_args: dx_path: {:?}", dx_path);
    trace!("parse_args: mod_dir: {:?}", mod_dir);
    trace!("parse_args: disass_dir: {:?}", disass_dir);
    trace!("parse_args: no_diff: {:?}", no_diff);

    if help_page
        || mod_dir.is_none()
        || disass_dir.is_none()
        || dx_path.is_none()
        || baksmali_path.is_none()
    {
        println!("bttv-android/ubi {}", VERSION);
        println!("usage: ubi <path/to/baksmali> </path/to/dx> <mod dir>, <disass dir>");
        process::exit(1);
    }
    trace!("parse_args: won't show help");

    trace!("parse_args: will return tuple");
    (
        baksmali_path.unwrap(),
        dx_path.unwrap(),
        mod_dir.unwrap(),
        disass_dir.unwrap(),
        no_diff,
    )
}

fn get_all_smali_files(path: String) -> Vec<String> {
    let mut vec = vec![];
    for entry in walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        let extension = path.extension();
        if extension.is_some() && extension.unwrap() == "smali" {
            match path.to_str() {
                Some(v) => vec.push(
                    v.to_string()
                        .strip_prefix("/tmp/bttv-ubi-smali/")
                        .unwrap()
                        .to_string(),
                ),
                None => warn!("could not convert path to str: {}", path.display()),
            }
        }
    }
    return vec;
}
