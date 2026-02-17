// Copyright (C) <2026> <Bogdan Yachmenv>
// SPDX-License-Identifier: GPL-3.0/

//Var realisation for SBSH
use std::env;


unsafe fn set_var_unsafe(key: &str, value: &str) {
    env::set_var(key, value);
}

unsafe fn remove_var_unsafe(key: &str) {
    env::remove_var(key);
}

/// Var builtin 
pub fn handle_var(args: Vec<String>) {
    if args.is_empty() {
        for (key, value) in env::vars() {
            println!("{}={}", key, value);
        }
        return;
    }

    match args[1].as_str() {
        "match" => {
            if args.len() < 3 {
                eprintln!("var match: too few arguments");
                return;
            }
            
            unsafe {
                var_match(&args[1..]);
            }
        }
        "del" => {
            if args.len() < 3 {
                eprintln!("var del: missing variable name");
                return;
            }
            unsafe {
                remove_var_unsafe(&args[2]);
            }
        }
        _ => {
            if args.len() >= 3 && args[2] == "=" {
            let name = &args[1];
            let value = args[3..].join(" ");
            unsafe { set_var_unsafe(name, &value); }
        }
        else if let Some((key, value)) = args[0].split_once('=') {
            unsafe { set_var_unsafe(key, value); }
        }
        else {
            eprintln!("var: unknown subcommand or invalid format. Use: var key=value, var del NAME, or var match ...");
        }
}
    }
}

///Module for match operations
unsafe fn var_match(tokens: &[String]) {
    
    let mut end = tokens.len();
    for (i, token) in tokens.iter().enumerate() {
        if token == ";" {
            end = i;
            break;
        }
    }
    let tokens = &tokens[..end];

    let (var_name, expr_tokens) = extract_assignment(tokens)
        .unwrap_or_else(|| panic!("var match: expected 'variable = expression'"));
    let expr_str = expr_tokens.join(" ");
    let (left, op, right) = parse_expression(&expr_str)
        .unwrap_or_else(|| panic!("var match: invalid expression: '{}'", expr_str));

    let left_val = get_value(&left);
    let right_val = get_value(&right);

    let result = match op {
        '+' => left_val + right_val,
        '-' => left_val - right_val,
        '*' => left_val * right_val,
        '/' => left_val / right_val,
        '^' => left_val.powf(right_val),
        _ => unreachable!(),
    };

    set_var_unsafe(&var_name, &result.to_string());
}

///Tokenesator
fn extract_assignment(tokens: &[String]) -> Option<(String, Vec<String>)> {
    if tokens.is_empty() {
        return None;
    }

    for (i, token) in tokens.iter().enumerate() {
        if token == "=" {
            if i == 0 {
                return None;
            }
            let name = tokens[0].clone();
            let expr_tokens = tokens[i+1..].to_vec();
            return Some((name, expr_tokens));
        }
    }

    for (i, token) in tokens.iter().enumerate() {
        if let Some(pos) = token.find('=') {
            let name = token[..pos].to_string();
            let rest = token[pos+1..].to_string();
            let mut expr_tokens = Vec::new();
            if !rest.is_empty() {
                expr_tokens.push(rest);
            }
            
            expr_tokens.extend_from_slice(&tokens[i+1..]);
            return Some((name, expr_tokens));
        }
    }

    
    None
}


fn parse_expression(expr: &str) -> Option<(String, char, String)> {
    let expr = expr.trim();
    if expr.is_empty() {
        return None;
    }

    
    let operators = ['+', '-', '*', '/', '^'];


    let bytes = expr.as_bytes();
    for (i, &c) in bytes.iter().enumerate() {
        if operators.contains(&(c as char)) {
            // Проверяем, что это не первый и не последний символ
            if i > 0 && i < expr.len() - 1 {
                let left = expr[..i].trim().to_string();
                let right = expr[i+1..].trim().to_string();
                if !left.is_empty() && !right.is_empty() {
                    return Some((left, c as char, right));
                }
            }
        }
    }

    for (i, &c) in bytes.iter().enumerate() {
        if operators.contains(&(c as char)) && i > 0 && i < expr.len() - 1 {
            let left = expr[..i].trim().to_string();
            let right = expr[i+1..].trim().to_string();
            return Some((left, c as char, right));
        }
    }

    
    None
}


fn get_value(token: &str) -> f64 {
    if let Ok(num) = token.parse::<f64>() {
        return num;
    }
    match env::var(token) {
        Ok(val) => val.parse::<f64>().unwrap_or_else(|_| {
            panic!("Number Invalid: variable '{}' contains non-numeric value", token)
        }),
        Err(_) => panic!("Number Invalid: unknown variable '{}'", token),
    }
}