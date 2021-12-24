#[macro_use]
extern crate log;
extern crate clap;
extern crate env_logger;
extern crate ubi_core;
extern crate walkdir;
extern crate zip;

mod args;
mod git;
mod mod_dir;

use std::env;
use std::path::{Path, PathBuf};
use ubi_core::{diff, smali};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    env_logger::init();

    trace!("Logger initialized, getting args");

    let args = args::parse_args();
    debug!("{:#?}", args);

    let mod_smali = mod_dir::handle_mod_dir(&args);

    let mut smali_files = get_all_smali_files(mod_smali.to_string());
    trace!("smali files: {:#?}", smali_files);

    if !args.no_diff {
        let filter = git::diff(&args.disass_dir);
        trace!("changed files: {:#?}", filter);
        smali_files = smali_files
            .into_iter()
            .filter(|path| filter.contains(path))
            .collect();
        trace!("smali files: {:#?}", smali_files);
    }

    let mod_base = std::path::Path::new(mod_smali);
    let disass_base = std::path::Path::new(&args.disass_dir);

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

            if !diff::print_diff(&args, rel_path, smali_mod.unwrap(), smali_disass.unwrap()) {
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
