use std::{ffi::OsStr, path::PathBuf};

pub struct AppArgs {
    pub input: PathBuf,
    pub output: PathBuf,
}

pub fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();
    let args = AppArgs {
        input: pargs
            .value_from_os_str("--input", |s: &OsStr| -> Result<PathBuf, &'static str> {
                Ok(s.into())
            })?,
        output: pargs
            .value_from_os_str("--output", |s: &OsStr| -> Result<PathBuf, &'static str> {
                Ok(s.into())
            })?,
    };
    Ok(args)
}
