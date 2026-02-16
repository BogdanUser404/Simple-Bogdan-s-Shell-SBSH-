// Copyright (C) <2026> <Bogdan Yachmenv>
// SPDX-License-Identifier: GPL-3.0/

mod aliases;
use std::env;
mod cmd_runner;

pub fn get_curent_dir() -> String{
    current_dir()
}
pub fn get_user(){
    username()
}

pub fn alias_add(name: String, command: String){
    alias::add(name,command)
}
pub fn alias_get(name: String){
    alias::get(name)
}
pub fn alias_remove(name:String){
    alias::remove(name);
}
pub fn alias_list(){
    alias::list();
}
pub fn alias_clear(){
    alias::clear;
}

pub fn get_var(name: String) -> String{
    match env::var(name) {
        Ok(val) => println!("Значение: {}", val),
        Err(e) => println!("Ошибка: {}", e),
    }
    val
}
pub fn set_var(name:String, value:String){
    env::set_var(name, value);
}

pub fn run_command(command: &str){
    cmd_runner::handle_builtin(command);
}