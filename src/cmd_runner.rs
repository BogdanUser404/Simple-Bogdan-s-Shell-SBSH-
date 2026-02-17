// Copyright (C) <2026> <Bogdan Yachmenv>
// SPDX-License-Identifier: GPL-3.0/

use libc::{WEXITSTATUS, WIFEXITED, waitpid, fork, execvp};
use shlex::split;
use std::ffi::CString;
use std::ptr;
use std::env;
use std::path::Path;
use std::io::{self, Write};
pub mod aliases;
mod print;
mod small_utils;
mod var;

pub fn handle_builtin(line: &str) {
    const MAX_EXPANSION: u32 = 10;  // защита от циклических алиасов
    let mut current_line = line.to_string();
    let mut expansion_count = 0;

    loop {
        let args: Vec<String> = match split(&current_line) {
            Some(a) => a,
            None => {
                eprintln!("Command parsing error");
                return;
            }
        };
        if args.is_empty() {
            return;
        }

        if let Some(alias_cmd) = aliases::get(&args[0]) {
            if expansion_count >= MAX_EXPANSION {
                eprintln!("Alias expansion too deep (possible cycle)");
                return;
            }
            let rest = if args.len() > 1 {
                format!(" {}", args[1..].join(" "))
            } else {
                String::new()
            };
            current_line = format!("{}{}", alias_cmd, rest);
            expansion_count += 1;
            continue;
        }

        match args[0].as_str() {
            "print" => {
                print::print(args);
            }
            "cd" => {
                small_utils::cd(args);
            }
            "var" => {
                var::handle_var(args);
            }
            "exit" => {
                small_utils::exit(args);
            }
            "clr" => {
                small_utils::clr();
            }
            _ => {
                system_run(args);
            }
        };
        break;
    }
}

fn system_run(args: Vec<String>) {
    unsafe {
        let pid = fork();
        match pid {
            -1 => panic!("Fork Failed!"),
            0 => {
                let c_args: Vec<CString> = args.iter()
                    .map(|arg| CString::new(arg.as_str()).expect("CString::new failed"))
                    .collect();

                let mut argv_ptrs: Vec<*const libc::c_char> = c_args.iter()
                    .map(|arg| arg.as_ptr())
                    .collect();

                argv_ptrs.push(ptr::null());
                execvp(argv_ptrs[0], argv_ptrs.as_ptr());
                panic!("execvp failed!");
            }
            child_pid => {
                let mut status: i32 = 0;
                let ret = waitpid(child_pid, &mut status, 0);
                if ret == -1 {
                    eprintln!("waitpid error");
                } else if WIFEXITED(status) {
                    let code = WEXITSTATUS(status);
                    if code != 0 {
                        eprintln!("the program terminated with code {}", code);
                    }
                }
            }
        }
    }
}