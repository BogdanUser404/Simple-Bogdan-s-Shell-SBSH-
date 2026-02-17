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

    //Compiling config
    let ast = GLOBAL_ENGINE.with(|eng| {
        eng.borrow()
            .compile_file(config.into())
            .expect("Failed to compile configuration file")
    });


    GLOBAL_AST.with(|cell| *cell.borrow_mut() = Some(ast.clone()));

    //Runing confog
    GLOBAL_ENGINE.with(|eng| {
        eng.borrow()
            .run_ast(&ast)
            .expect("Failed to execute configuration file")
    });

    //Get prompt
    let _ = api::get_var("PS1".to_string())
        .expect("No PS1 variable set. Check your .sbshrc.rhai");

    let mut rl = DefaultEditor::new()
        .expect("Failed to create rustyline editor");
    let _ = rl.load_history(&history);

    loop {
        let mut scope = ::rhai::Scope::new();

        //Run hook repeat
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
        //Get prompr
        let ps1 = api::get_var("PS1".to_string())
            .expect("Non PS1 variable, check the .sbshrc and plugins");

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

        //Runig on output
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