use std::error::Error;
use std::fs;
use walkdir::{DirEntry, WalkDir};
use regex::Regex;

pub struct Config {
    pub cmd: String,
    pub params: Option<String>,
    pub dirname: String
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        args.next();
        let cmd = args.next().unwrap();
        let params_or_dirname = args.next().unwrap();
        match args.next() {
            Some(arg) => Ok(Config{ cmd, params: Some(params_or_dirname), dirname: arg }),
            None => Ok(Config{ cmd, params: None, dirname: params_or_dirname})
        }

        
    }
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    if config.cmd.eq("number") {
        fix_number(&config.dirname, &config.params)?
    } else if config.cmd.eq("space") {
        fix_space(&config.dirname, &config.params)?
    } else if config.cmd.eq("ext") {
        fix_ext(&config.dirname, &config.params)?
    } else {
        return Err("Unknown CMD".into());
    }
    Ok(())
}
fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
fn starts_with_number(entry: &DirEntry) -> bool {
    entry.file_name()
    .to_str()
    .map(|s| s.chars().next().unwrap().is_numeric())
    .unwrap_or(false) 
}
fn fix_number(dirname: &str, params: &Option<String>) -> std::io::Result<()> {
    let number = match params {
        Some(params) => match params.trim().parse() {
            Ok(num) => num,
            Err(_) => 2,
        },
        None => 2
    };
    for entry in WalkDir::new(dirname)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir() && !is_hidden(e) && starts_with_number(e)) {
        let f_name = String::from(entry.file_name().to_string_lossy());

        let re = Regex::new(r"^([0-9]+)\W*(.+)$").unwrap();
        let caps = re.captures(&f_name).unwrap();
        let f_name_prefix = caps.get(1).map_or("", |m| m.as_str());
        let f_name_suffix = caps.get(2).map_or("", |m| m.as_str());
        let o_number: i32 = f_name_prefix.parse().unwrap();
        
        let new_number = format!("{:0alignment$}", o_number, alignment = number);
        let new_f_name = format!("{}.{}", new_number, f_name_suffix);
        let path = entry.into_path();
        let mut new_path = path.clone();
        new_path.set_file_name(new_f_name);
        fs::rename(path, new_path)?;
    }
    Ok(())
}

fn fix_space(dirname: &str, params: &Option<String>) -> std::io::Result<()> {
    let divider = match params {
        Some(params) => params,
        None => "_"
    };
    for entry in WalkDir::new(dirname)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir() && !is_hidden(e)) {
        let f_name = String::from(entry.file_name().to_string_lossy());
        let re = Regex::new(r"(.+?)(\.[^.]*$|$)").unwrap();
        let caps = re.captures(&f_name).unwrap();
        let f_name_without_extension = caps.get(1).map_or("", |m| m.as_str()).trim();
        let parts: Vec<&str> = f_name_without_extension.split_whitespace().collect();
        let new_f_name_without_extension:String = parts.join(divider);
        let extension = caps.get(2).map_or("", |m| m.as_str());
        let new_f_name = format!("{}{}", new_f_name_without_extension, extension);
        let path = entry.into_path();
        let mut new_path = path.clone();
        new_path.set_file_name(new_f_name);
        fs::rename(path, new_path)?;
    }
    Ok(())
}
fn fix_ext(dirname: &str, params: &Option<String>) -> std::io::Result<()> {
    let upper = match params {
        Some(params) => params == "upper",
        None => false
    };
    for entry in WalkDir::new(dirname)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir() && !is_hidden(e)) {
        let f_name = String::from(entry.file_name().to_string_lossy());
        let re = Regex::new(r"(.+?)(\.[^.]*$|$)").unwrap();
        let caps = re.captures(&f_name).unwrap();
        let f_name_without_extension = caps.get(1).map_or("", |m| m.as_str()).trim();
        let extension = caps.get(2).map_or("", |m| m.as_str());
        if !extension.eq("") {
            let new_extension = if upper { extension.to_uppercase() } else { extension.to_lowercase() };
            let new_f_name = format!("{}{}", f_name_without_extension, new_extension);
            let path = entry.into_path();
            let mut new_path = path.clone();
            new_path.set_file_name(new_f_name);
            fs::rename(path, new_path)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fix_filename_number() {
        let dirname = "./src/testdir";
        let params = "2";
        assert_eq!((), fix_number(&dirname, &Some(params.to_string())).unwrap());
    }
    #[test]
    fn fix_filename_space() {
        let dirname = "./src/testdir";
        let params = "-";
        assert_eq!((), fix_space(&dirname, &Some(params.to_string())).unwrap());
    }

    #[test]
    fn fix_filename_ext() {
        let dirname = "./src/testdir";
        let params = "lower";
        assert_eq!((), fix_ext(&dirname, &Some(params.to_string())).unwrap());
    }
}