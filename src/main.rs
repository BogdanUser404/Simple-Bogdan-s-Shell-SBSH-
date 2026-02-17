// Copyright (C) <2026> <Bogdan Yachmenv>
// SPDX-License-Identifier: GPL-3.0

use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
mod api;
mod rhai_api;
pub mod cmd_runner;

fn main() {
    let engine = rhai_api::init_rhai();
    let home = api::get_var("HOME".to_string()).unwrap_or_default();
    let config = home.clone() + "/.sbshrc.rhai";
    let history = home + "/.sbsh_history.txt";

    let ast = engine.compile_file(config.into())
        .expect("Failed to compile configuration file");
    engine.run_ast(&ast)
        .expect("Failed to execute configuration file");

    let mut ps1 = api::get_var("PS1".to_string())
        .expect("No PS1 variable set. Check your .sbshrc.rhai");

    let mut rl = DefaultEditor::new()
        .expect("Failed to create rustyline editor");
    let _ = rl.load_history(&history);

    loop {
        let mut scope = ::rhai::Scope::new();

        // Repeat hook
        if let Err(e) = engine.call_fn::<()>(&mut scope, &ast, "repeat", ()) {
            if !e.to_string().contains("Function not found") {
                eprintln!("Error in on_input hook: {}", e);
            }
        }

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

        //on_input hook
        if let Err(e) = engine.call_fn::<()>(&mut scope, &ast, "on_input", (line.clone(),)) {
            if !e.to_string().contains("Function not found") {
                eprintln!("Error in on_input hook: {}", e);
            }
            if e.to_string().contains("Function not found"){
                api::run_command(line);
            }
        }
    }

    rl.save_history(&history).ok();
}