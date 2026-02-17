// Copyright (C) <2026> <Bogdan Yachmenv>
// SPDX-License-Identifier: GPL-3.0/

use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;


lazy_static! {
    static ref ALIASES: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

pub fn add(name: &str, command: &str) {
    ALIASES.lock().unwrap().insert(name.to_string(), command.to_string());
}

pub fn get(name: &str) -> Option<String> {
    ALIASES.lock().unwrap().get(name).cloned()
}

pub fn remove(name: &str) {
    ALIASES.lock().unwrap().remove(name);
}

pub fn list() -> Vec<(String, String)> {
    ALIASES.lock().unwrap()
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}

pub fn clear() {
    ALIASES.lock().unwrap().clear();
}
