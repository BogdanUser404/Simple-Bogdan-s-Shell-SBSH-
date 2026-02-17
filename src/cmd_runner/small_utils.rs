// Copyright (C) <2026> <Bogdan Yachmenv>
// SPDX-License-Identifier: GPL-3.0/

use std::io;
use std::io::Write;
use std::env;
use std::path::Path;

pub fn clr(){
    print!("\x1B[2J\x1B[H");
    std::io::stdout().flush().unwrap();
}
pub fn cd(args: Vec<String>) {
    let target = if args.len() >= 2 {
        args[1].clone()
    } else {
        env::var("HOME").unwrap_or_else(|_| "/".to_string())
    };
    if let Err(e) = env::set_current_dir(Path::new(&target)) {
        eprintln!("cd: {}: {}", target, e);
    }
}
pub fn exit(args: Vec<String>){
    let code = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    std::process::exit(code);
}
