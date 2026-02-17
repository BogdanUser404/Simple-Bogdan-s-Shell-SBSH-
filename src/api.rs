// Copyright (C) <2026> <Bogdan Yachmenv>
// SPDX-License-Identifier: GPL-3.0/

use std::env;
use libc::option;
use whoami::username;
use iridescent::{Styled, Rgb};
use git2::Repository;
use crate::cmd_runner;
use crate::cmd_runner::aliases;
use crate::rhai_api::{self, init_rhai};
use std::fs;
use std::io::Write;
use std::path::Path;
use chrono::Local;
use std::time::Instant;
use std::thread::sleep;
use std::time::Duration;
use once_cell::sync::Lazy;
use std::sync::Mutex;
static GLOBAL_TIMER: Lazy<Mutex<Option<Instant>>> = Lazy::new(|| Mutex::new(None));


//Geting values
pub fn get_current_dir() -> String {
    env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default()
}


pub fn get_user() -> String {
    username()
}
//Aliases
pub fn alias_add(name: &str, command: &str) {
    aliases::add(name, command);
}

pub fn alias_get(name: &str) -> Option<String> {
    aliases::get(name)
}

pub fn alias_remove(name: &str) {
    aliases::remove(name);
}

pub fn alias_list() {
    aliases::list();
}

pub fn alias_clear() {
    aliases::clear();
}

//Variables
pub fn set_var(name: String, value: String) {
    unsafe {env::set_var(name, value);}
}

pub fn get_var(name: String) -> Option<String>{
    env::var(name).ok()
}
pub unsafe fn del_var(name: String){
    env::remove_var(name);
}


pub fn run_command(command: String){
    cmd_runner::handle_builtin(&command);
}
//Fromating
pub fn set_color(text: String, r: i64, g: i64, b: i64) -> String {
    text.foreground(&[r as u8, g as u8, b as u8]).to_string()
}


pub fn set_bold(text: String) -> String {
    text.bold().to_string()
}
//Git
pub fn is_git_repo() -> bool {
    env::current_dir()
        .ok()
        .and_then(|dir| Repository::discover(dir).ok())
        .is_some()
}

pub fn get_git_branch() -> String {
    let dir = match env::current_dir() {
        Ok(d) => d,
        Err(_) => return String::new(),
    };
    let repo = match Repository::discover(dir) {
        Ok(r) => r,
        Err(_) => return String::new(),
    };
    let head = match repo.head() {
        Ok(h) => h,
        Err(_) => return String::new(),
    };
    head.shorthand().unwrap_or("").to_string()
}

pub fn git_is_dirty() -> bool {
    let dir = match env::current_dir() {
        Ok(d) => d,
        Err(_) => return false,
    };
    let repo = match Repository::discover(dir) {
        Ok(r) => r,
        Err(_) => return false,
    };
    let statuses = match repo.statuses(None) {
        Ok(s) => s,
        Err(_) => return false,
    };
    statuses.iter().any(|entry| entry.status() != git2::Status::CURRENT)
}

pub fn git_ahead_behind() -> (usize, usize) {
    let dir = match env::current_dir() {
        Ok(d) => d,
        Err(_) => return (0, 0),
    };
    let repo = match Repository::discover(dir) {
        Ok(r) => r,
        Err(_) => return (0, 0),
    };

    let head = match repo.head() {
        Ok(h) => h,
        Err(_) => return (0, 0),
    };

    let local_oid = match head.target() {
        Some(oid) => oid,
        None => return (0, 0),
    };

    let branch_name = match head.shorthand() {
        Some(name) => name,
        None => return (0, 0),
    };

    let local_branch = match repo.find_branch(branch_name, git2::BranchType::Local) {
        Ok(b) => b,
        Err(_) => return (0, 0),
    };

    let upstream = match local_branch.upstream() {
        Ok(u) => u,
        Err(_) => return (0, 0),
    };

    let upstream_oid = match upstream.get().target() {
        Some(oid) => oid,
        None => return (0, 0),
    };

    match repo.graph_ahead_behind(local_oid, upstream_oid) {
        Ok((ahead, behind)) => (ahead as usize, behind as usize),
        Err(_) => (0, 0),
    }
}
pub fn load_plugin(path: String){
    let engine = init_rhai();
    engine.run_file(path.into());
}

//File API

pub fn read_file(path: String) -> String {
    fs::read_to_string(&path).unwrap_or_else(|e| {
        eprintln!("Error reading file {}: {}", path, e);
        String::new()
    })
}

pub fn write_file(path: String, content: String) {
    if let Err(e) = fs::write(&path, &content) {
        eprintln!("Error writing file {}: {}", path, e);
    }
}
//File info get
pub fn get_file(path_str: &str) -> Option<String> { //Using the &str type instead of String is forced, replacing the type will break the code
    let path = Path::new(path_str);
    path.file_name().map(|name| name.to_string_lossy().to_string())
}
pub fn is_dir(path_str: &str) -> bool {
    let path = Path::new(path_str);
    if let Ok(metadata) = fs::metadata(path) {
        metadata.is_dir()
    } else {
        false
    }
}
pub fn is_file(path_str: &str) -> bool {
    let path = Path::new(path_str);
    if let Ok(metadata) = fs::metadata(path) {
        if metadata.is_dir() == true{
            false
        }
        else {
            true
        }
    } else {
        false
    }
}

//Time features

pub fn get_current_time() -> String{
    let now = Local::now();
    now.format("%H:%M:%S").to_string()
}

pub fn start_timer() {
    let mut timer = GLOBAL_TIMER.lock().unwrap();
    *timer = Some(Instant::now());
}

pub fn stop_timer() -> f64 {
    let timer = GLOBAL_TIMER.lock().unwrap();
    if let Some(start_time) = *timer {
        start_time.elapsed().as_secs_f64()
    } else {
        0.0 // Возвращаем 0, если забыли вызвать start_timer
    }
}