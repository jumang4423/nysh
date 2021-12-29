use git2::Repository;
use std::env::args;
use std::fs::read_to_string;
use std::io;

pub fn check_nysh_lib() -> Option<std::path::PathBuf> {
    // check the ~/.nysh is exsists
    let _nysh_path = match dirs::home_dir() {
        Some(path) => path.join(".nysh"),
        None => return None,
    };
    // is _nysh_path exsists
    if !_nysh_path.exists() {
        return Some(_nysh_path);
    } else {
        return None;
    }
}

pub fn auto_install_lib() -> Result<(), String> {
  // auto installing nylang lib
  println!("{}", "-> started auto install!");
  println!("{}", "-> fetching nysh into ~/.nysh");
  let url = "https://github.com/jumang4423/nysh.git";
  let _cache_path = match dirs::home_dir() {
    Some(path) => path.join(".nysh"),
    None => return Err("Could not get home directory".to_string()),
  };
  match Repository::clone(url, _cache_path) {
    Err(e) => return Err(format!("failed to fetch: {:?}", e).to_string()),
    _ => (),
  };

  Ok(())
}