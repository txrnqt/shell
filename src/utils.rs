use std::fs;
use chrono::prelude::*;

use anyhow::Ok;

use crate::errors::CrateResult;

pub fn pwd() -> CrateResult<String> {
    let current_dir = std::env::current_dir()?;
    Ok(current_dir.display().to_string())
}

pub fn ls() -> CrateResult<()> {
    let entries = fs::read_dir(".")?;

    for entry in entries {
        let entry = entry?;
        println!("{}", entry.file_name().to_string_lossy());
    }

    Ok(())
}

pub fn cat(path: &str) -> CrateResult<String> {
    let pwd = pwd()?;
    let joined_path = std::path::Path::new(&pwd).join(path);
    let contents = fs::read_to_string(joined_path)?;

    Ok(contents)
}

pub fn cd(path: &str) -> CrateResult<()> {
    let _ = std::env::set_current_dir(path)?;

    Ok(())
}

pub fn rm(path: &str) -> CrateResult<()> {
    let is_dir = fs::metadata(path).unwrap();
    
    if is_dir.is_dir() {
        let _ = fs::remove_dir(path)?;
    } else {
        let _ = fs::remove_file(path)?;
    }
     
    Ok(())
}

pub fn touch(path: &str) -> CrateResult<()> {
    let _ = fs::File::create(path)?;

    Ok(())
}

pub fn date() -> CrateResult<String> {
    let date = Local::now().to_string();

    Ok(date)
}

pub fn mkdir(path: &str) -> CrateResult<()> {
    let _ = fs::create_dir(path)?;

    Ok(())
}