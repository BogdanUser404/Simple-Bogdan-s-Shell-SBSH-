// Copyright (C) <2026> <Bogdan Yachmenv>
// SPDX-License-Identifier: GPL-3.0/

///Rhai integration
use rhai::{Engine, Scope};
use crate::api;

pub fn init_rhai() -> Engine{
    let mut  engine = Engine::new();
    engine.register_fn("get_user", api::get_user);
    engine.register_fn("get_current_dir", api::get_current_dir);
    engine.register_fn("get_var", api::get_var);
    engine.register_fn("set_var", api::set_var);
    engine.register_fn("alias_add", api::alias_add);
    engine.register_fn("alias_list", api::alias_list);
    engine.register_fn("alias_remove", api::alias_remove);
    engine.register_fn("alias_get", api::alias_get);
    engine.register_fn("alias_clear", api::alias_clear);
    engine.register_fn("set_color", api::set_color);
    engine.register_fn("set_bold", api::set_bold);
    engine.register_fn("system", api::run_command);
    engine.register_fn("is_git_repo", api::is_git_repo);
    engine.register_fn("get_git_branch", api::get_git_branch);
    engine.register_fn("git_is_dirty", api::git_is_dirty);
    engine.register_fn("load_plugin", api::load_plugin);
    engine.register_fn("git_ahead_behind", api::git_ahead_behind);
    //Returned engine for work in main func
    engine
}