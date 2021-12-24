#[macro_use]
extern crate log;
extern crate yaml_rust;

pub mod diff;
pub mod smali;
pub mod ubignore;

#[derive(Debug)]
pub struct UbiArgs {
    pub baksmali_path: String,
    pub dx_path: String,
    pub mod_dir: String,
    pub disass_dir: String,
    pub no_diff: bool,
    pub ignore_default_constructors: bool,
    pub ignore_object_super: bool,
    pub ubignore: Option<ubignore::UbiIgnore>,
}
