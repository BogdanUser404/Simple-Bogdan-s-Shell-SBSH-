// Copyright (C) <2026> <Bogdan Yachmenv>
// SPDX-License-Identifier: GPL-3.0/

///Rhai integration
use rhai::{Engine, Scope};
mod api;

pub fn init_rhai() -> engine{
    let engine = Engine::new();
    engine.register_fn("get_user", api::get_user);
    engine.register_fn("get_current_dir", api::get_current_dir);
    engine.register_fn("get_var", api::get_var);
    engine.register_fn("set_var", api::set_var);
    engine.register_fn("alias_add", api::alias_add);
    engine.register_fn("alias_list", api::alias_list);
    engine.register_fn("alias_remove", api::alias_remove);
    engine.register_fn("alias_get", api::alias_get);
    engine.register_fn("alias_clear", api::alias_clear);
    engine.register_fn("system", api::run_command);
    //Returned engine for work in main func
    engine
}