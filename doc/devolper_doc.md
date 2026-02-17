# Technical Documentation for SBSH

This document is created to simplify understanding of the project for new contributors and those who wish to study its source code.

## Project Structure

- `/src/main.rs` – Main file; handles configuration processing, startup, and input retrieval.
- `/src/cmd_runner.rs` – Main file of the `cmd_runner` module; handles command processing and built‑in commands.
- `/src/api.rs` – API that glues libraries into a single interface to simplify refactoring and code expansion.
- `/src/rhai_api.rs` – API for working with Rhai configurations; contains the Rhai engine initializer function and registration of functions for Rhai configs.
- `/src/cmd_runner/print.es` – Implementation of the `print` built‑in command.
- `/src/cmd_runner/var.rs` – Implementation of the `var` built‑in command.
- `/src/cmd_runner/aliases.rs` – Implementation of aliases.
- `/src/cmd_runner/small_utils.rs` – Implementation of three small commands: `cd`, `exit`, and `clr`.

## Project Conventions

We use the following naming conventions:
- `my_var`
- `my_func`
- `MY_CONST`
- `my_module`
- `MyStruct`
- `MY_GLOBAL_VAR`

Comments are written in English.  
Use comments in places that are difficult to read without them.  
Do not overuse pointers.  
Use `String` instead of `&str` in function arguments and return values.