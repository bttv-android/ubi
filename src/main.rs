#[macro_use]
extern crate log;
extern crate env_logger;
extern crate git2;
extern crate walkdir;
extern crate zip;

mod diff;
mod git;
mod mod_dir;
mod smali;

use std::env;
use std::path::{Path, PathBuf};
use std::process;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    env_logger::init();

    trace!("Logger initialized, getting args");
    let (
        baksmali_path,
        dx_path,
        mod_dir, 
        disass_dir,
        no_diff,
        ignore_default_constructors,
        ignore_object_super
    ) = parse_args();

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

    let mod_base = std::path::Path::new(mod_smali);
    let disass_base = std::path::Path::new(&disass_dir);

    let mut no_diffs_found = 0;
    let mut diffs_found = 0;
    let mut files_not_found = vec![];

    for rel_path in smali_files {
        let mod_path = mod_base.join(&rel_path);
        let disass_path = find_disass_path(&disass_base, &rel_path);
        if disass_path.is_none() {
            files_not_found.push(rel_path);
            continue;
        }
        let disass_path = disass_path.unwrap();

        let smali_mod = smali::parse_file(&mod_path);
        let smali_disass = smali::parse_file(&disass_path);

        if smali_disass.is_err() {
            error!("error ({:?}) {}", disass_path, smali_disass.unwrap_err());
        } else if smali_mod.is_err() {
            error!("error ({:?}): {}", mod_path, smali_mod.unwrap_err());
        } else {
            let smali_mod = smali_mod.unwrap();
            let smali_disass = smali_disass.unwrap();

            if smali_mod.is_none() {
                error!("error ({:?}) {:?}", mod_path, smali_mod);
            } else if smali_disass.is_none() {
                error!("error ({:?}) {:?}", disass_path, smali_disass);
            }

            if !diff::print_diff(rel_path, smali_mod.unwrap(), smali_disass.unwrap(), ignore_default_constructors, ignore_object_super) {
                no_diffs_found += 1;
            } else {
                diffs_found += 1;
            }
        }
    }

    if !files_not_found.is_empty() {
        for rel in files_not_found {
            warn!("not found in disass: {}", rel);
        }
    }
    info!("{} files had diffs", diffs_found);
    info!("{} files were ok", no_diffs_found);
}

/** Returns or kills process with error message */
fn parse_args() -> (String, String, String, String, bool, bool, bool) {
    let mut no_diff = false;
    let mut help_page = false;
    let mut mod_dir = None;
    let mut disass_dir = None;
    let mut dx_path = None;
    let mut baksmali_path = None;
    let mut ignore_default_constructors = false;
    let mut ignore_object_super = false;

    let mut neg = 0;

    for (i, arg) in env::args().enumerate() {
        trace!("{}", arg);
        if i == 0 {
            continue;
        } else if arg == "-h" || arg == "--help" {
            help_page = true;
            neg += 1;
        } else if arg == "--no-diff" {
            no_diff = true;
            neg += 1;
        } else if arg == "--ignore-default-constructors" {
            ignore_default_constructors = true;
            neg += 1;
        } else if arg == "--ignore-object-super" {
            ignore_object_super = true;
            neg += 1;
        } else {
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
    }

    trace!("parse_args: help_page: {}", help_page);
    trace!("parse_args: baksmali_path: {:?}", baksmali_path);
    trace!("parse_args: dx_path: {:?}", dx_path);
    trace!("parse_args: mod_dir: {:?}", mod_dir);
    trace!("parse_args: disass_dir: {:?}", disass_dir);
    trace!("parse_args: no_diff: {:?}", no_diff);
    trace!("parse_args: ignore-default-constructors: {}", ignore_default_constructors);
    trace!("parse_args: ignore-object-super: {}", ignore_object_super);

    if help_page
        || mod_dir.is_none()
        || disass_dir.is_none()
        || dx_path.is_none()
        || baksmali_path.is_none()
    {
        println!("bttv-android/ubi {}", VERSION);
        println!("usage: ubi [options] <path/to/baksmali> </path/to/dx> <mod dir>, <disass dir>");
        println!("options:");
        println!("  --help | -h");
        println!("  --no-diff");
        println!("  --ignore-default-constructors");
        println!("  --ignore-object-super");
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
        ignore_default_constructors,
        ignore_object_super,
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

fn find_disass_path(base: &Path, rel_path: &String) -> Option<PathBuf> {
    let wd = walkdir::WalkDir::new(base)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok());

    for entry in wd {
        let path = entry.path();
        if path.is_dir() {
            let path = entry.path().join(rel_path);
            if path.exists() {
                return Some(path);
            }
        }
    }
    return None;
}
