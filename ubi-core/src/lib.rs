#[macro_use]
extern crate log;

pub mod diff;
pub mod smali;

#[derive(Debug)]
pub struct UbiArgs {
    pub baksmali_path: String,
    pub dx_path: String,
    pub mod_dir: String,
    pub disass_dir: String,
    pub no_diff: bool,
    pub ignore_default_constructors: bool,
    pub ignore_object_super: bool,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
