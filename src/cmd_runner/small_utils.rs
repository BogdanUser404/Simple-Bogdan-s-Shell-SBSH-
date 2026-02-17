// Copyright (C) <2026> <Bogdan Yachmenv>
// SPDX-License-Identifier: GPL-3.0/

use std::io;
use std::io::Write;
use std::env;
use std::path::Path;
use crate::GLOBAL_AST;
use crate::GLOBAL_ENGINE;


pub fn clr(){
    print!("\x1B[2J\x1B[H");
    std::io::stdout().flush().unwrap();
}
pub fn cd(args: Vec<String>) {
    // Запоминаем старую директорию как строку
    let old_dir = env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();

    let target = if args.len() >= 2 {
        args[1].clone()
    } else {
        env::var("HOME").unwrap_or_else(|_| "/".to_string())
    };

    if let Err(e) = env::set_current_dir(Path::new(&target)) {
        eprintln!("cd: {}: {}", target, e);
        return;
    }

    // Получаем новую директорию после смены
    let new_dir = env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or(target);

    // Используем глобальные движок и AST (thread-local)
    crate::GLOBAL_ENGINE.with(|eng| {
        crate::GLOBAL_AST.with(|cell| {
            if let Some(ast) = cell.borrow().as_ref() {
                let engine = eng.borrow();
                let mut scope = rhai::Scope::new();
                if let Err(e) = engine.call_fn::<()>(&mut scope, ast, "on_cd", (old_dir, new_dir)) {
                    if !e.to_string().contains("Function not found") {
                        eprintln!("Error in on_cd hook: {}", e);
                    }
                }
            }
        });
    });
}
pub fn exit(args: Vec<String>){
    //Exit hook reslisation
    crate::GLOBAL_ENGINE.with(|eng| {
    crate::GLOBAL_AST.with(|cell| {
        if let Some(ast) = cell.borrow().as_ref() {
            let engine = eng.borrow();
            let mut scope = rhai::Scope::new();
            if let Err(e) = engine.call_fn::<()>(&mut scope, ast, "on_exit", (args.clone())) {
                if !e.to_string().contains("Function not found") {
                        eprintln!("Error in on_cd hook: {}", e);
                    }
                }
            }
        });
    });
    //Exit
    let code = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    std::process::exit(code);
}
