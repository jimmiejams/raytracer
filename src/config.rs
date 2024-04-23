use std::ffi::OsStr;
use std::path::Path;

pub struct Config<'a> {
    pub output_filename: &'a Path,
}

impl Config<'_> {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough command line arguments")
        }
        let output_filename = Path::new(args.get(1).unwrap());
        if output_filename.extension() != Some(OsStr::new("exr")) {
            return Err("output filename must end in '.exr'")
        }
        Ok(Config {
            output_filename: output_filename,
        })
    }
}