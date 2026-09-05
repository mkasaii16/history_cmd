use std::{env, error::Error, io, path::PathBuf};

pub fn history_path() -> Result<PathBuf, Box<dyn Error>> {
    if let Some(path) = env::var_os("HISTFILE") {
        return Ok(PathBuf::from(path));
    }

    let home = env::var_os("HOME")
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "HOME is not defined"))?;

    let shell = env::var("SHELL").unwrap_or_default();
    let filename = if shell.ends_with("bash") {
        ".bash_history"
    } else {
        ".zsh_history"
    };

    Ok(PathBuf::from(home).join(filename))
}
