# SBSH Built-in Commands

SBSH (Simple Bogdan's Shell) provides several built-in commands that are executed directly by the shell without launching an external program. This document describes their syntax, behavior, and examples.

All built‑ins are **case‑sensitive**.

---

## `print` – output text and variables

**Syntax:**  
```
print [arguments...] [;]
```

- Arguments are separated by whitespace.
- If an argument starts with `$`, it is treated as an environment variable name and its value is substituted.
- Otherwise the argument is printed literally.
- A semicolon `;` terminates the command (optional if it is the last command on the line).
- No newline is added automatically; use `print ;` to output a blank line.

**Examples:**
```
print Hello world
print "The value is" $VAR
print $HOME ;
print "Line 1" ; print "Line 2"
```

---

## `cd` – change current directory

**Syntax:**  
```
cd [directory]
```

- Changes the current working directory to `directory`.
- If `directory` is omitted, changes to `$HOME`.
- On error, an error message is printed.

**Examples:**
```
cd /usr/local
cd
cd ..
```

---

## `var` – manage environment variables

**Syntax:**  
```
var [name=value | del name | match ...]
```

- Without arguments, lists all environment variables (one per line, in `name=value` format).

### Setting a variable
```
var name=value
```
Spaces around `=` are allowed (e.g., `var name = value`). Sets the variable `name` to `value`.

### Deleting a variable
```
var del name
```
Removes the variable `name`.

### Arithmetic expressions with `var match`
```
var match target = left operand right ;
```
- `target` – name of the variable to store the result.
- `left` and `right` may be numbers or names of existing numeric variables (no `$` needed).
- `operand` is one of `+`, `-`, `*`, `/`, `^` (exponentiation).
- Spaces are flexible; the expression can be written with or without spaces around the operator and `=`.
- A semicolon `;` is **required** (may have spaces before/after).
- If a variable used in `left` or `right` does not exist or its value is not a valid number, the shell panics with an error message.

**Examples:**
```
var x = 10
var y = 20
var match sum = x + y ;
print $sum
var match result = 2.5 * 4 ;
var match a = 100 / 3 ;
var match b = 2 ^ 8
```

---

## `exit` – terminate the shell

**Syntax:**  
```
exit [code]
```
- Terminates the shell process with the given exit `code` (default `0`).

**Examples:**
```
exit
exit 1
```

---

## `clr` – clear the terminal screen

**Syntax:**  
```
clr
```
- Sends the ANSI escape sequence to clear the screen and move the cursor to the top‑left corner.

---

## Environment Variables

- All built‑ins that read or write variables operate on the shell’s environment.
- Variables set with `var` are inherited by child processes.
- Use `$NAME` inside `print` to substitute the value.

---

## Command Separator

Multiple commands can be written on one line by separating them with a semicolon `;`.

**Example:**
```
cd /tmp ; pwd ; ls
```

---

## Comments

Any line starting with `#` is ignored.

**Example:**
```
# This is a comment
print "Hello"   # this is not a comment (only line‑starting #)
```

---

## External Programs

If a command is not a built‑in, SBSH attempts to execute it as an external program using the system `PATH` (via `execvp`). The shell waits for the program to finish and prints its exit code if it is non‑zero.

**Example:**
```
>> ls -l
>> grep "error" log.txt
>> /bin/echo hello
```

---

*For configuration and scripting with Rhai, see [config.md](config.md).*