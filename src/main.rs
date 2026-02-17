// Copyright (C) <2026> <Bogdan Yachmenv>
// SPDX-License-Identifier: GPL-3.0

use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
mod api;
mod rhai_api;
use std::cell::RefCell;
use rhai::{Engine, AST};
pub mod cmd_runner;

thread_local! {
    pub static GLOBAL_ENGINE: RefCell<Engine> = RefCell::new(rhai_api::init_rhai());
    pub static GLOBAL_AST: RefCell<Option<AST>> = RefCell::new(None);
}

fn main() {
    let home = api::get_var("HOME".to_string()).unwrap_or_default();
    let config = home.clone() + "/.sbshrc.rhai";
    let history = home + "/.sbsh_history.txt";

    // Compiling config – если ошибка, продолжаем без AST
    let ast = GLOBAL_ENGINE.with(|eng| {
        match eng.borrow().compile_file(config.into()) {
            Ok(ast) => Some(ast),
            Err(e) => {
                eprintln!("Warning: Failed to compile config: {}. Using default settings.", e);
                None
            }
        }
    });

    // Если компиляция удалась, сохраняем AST и выполняем
    if let Some(ref ast) = ast {
        GLOBAL_AST.with(|cell| *cell.borrow_mut() = Some(ast.clone()));

        // Running config – если ошибка, просто предупреждаем
        GLOBAL_ENGINE.with(|eng| {
            if let Err(e) = eng.borrow().run_ast(ast) {
                eprintln!("Warning: Failed to execute config: {}. Continuing with defaults.", e);
            }
        });
    } else {
        // Если конфиг не скомпилировался, AST остаётся None
    }

    // Получаем PS1 или ставим запасной промпт
    let ps1_default = "Fix config>> ".to_string();
    let ps1 = api::get_var("PS1".to_string()).unwrap_or_else(|| {
        eprintln!("Warning: PS1 not set. Using default prompt: {}", ps1_default);
        ps1_default.clone()
    });

    let mut rl = match DefaultEditor::new() {
        Ok(editor) => editor,
        Err(e) => {
            eprintln!("Warning: Failed to create rustyline editor: {}. Using minimal input.", e);
            // В этом случае мы не можем использовать rustyline, нужно либо паниковать, либо выйти.
            // Для простоты выходим с ошибкой, но можно попытаться использовать stdin/stdout напрямую.
            // Однако для демонстрации заменим на панику с предупреждением.
            panic!("Cannot proceed without rustyline");
        }
    };
    let _ = rl.load_history(&history);

    loop {
        let mut scope = ::rhai::Scope::new();

        // Run hook repeat
        GLOBAL_ENGINE.with(|eng| {
            GLOBAL_AST.with(|cell| {
                if let Some(ast) = cell.borrow().as_ref() {
                    if let Err(e) = eng.borrow().call_fn::<()>(&mut scope, ast, "repeat", ()) {
                        if !e.to_string().contains("Function not found") {
                            eprintln!("Error in repeat hook: {}", e);
                        }
                    }
                }
            });
        });

        // Получаем актуальный PS1 (может измениться в хуке)
        let ps1 = api::get_var("PS1".to_string()).unwrap_or_else(|| {
            eprintln!("Warning: PS1 became unset, using default.");
            "Fix config>> ".to_string()
        });

        let line = match rl.readline(&ps1) {
            Ok(line) => line,
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => break,
            Err(err) => {
                eprintln!("Readline error: {}", err);
                break;
            }
        };
        rl.add_history_entry(&line).ok();

        let mut scope = ::rhai::Scope::new();

        // Run on_input hook
        GLOBAL_ENGINE.with(|eng| {
            GLOBAL_AST.with(|cell| {
                if let Some(ast) = cell.borrow().as_ref() {
                    if let Err(e) = eng.borrow().call_fn::<()>(&mut scope, ast, "on_input", (line.clone(),)) {
                        if !e.to_string().contains("Function not found") {
                            eprintln!("Error in on_input hook: {}", e);
                        }
                    }
                }
            });
        });

        // Executing command
        api::run_command(line);
    }

    rl.save_history(&history).ok();
}