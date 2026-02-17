# Configuration and Addons

For configuration and addons, the Rhai scripting language is used.

This documentation is dedicated exclusively to the hooks and functions added to Rhai for working with SBSH and environment variables. Documentation for the original Rhai can be found on their website (link: https://rhai.rs/).

## Hooks

SBSH uses several hooks. Hooks are ordinary functions in Rhai.  
- `repeat` – called first in the main loop; its main purpose is to be a single-threaded analog of an infinite loop and update the shell state.  
- `on_input` – called after user input, receives the input as an argument. After this hook, standard command processing does **not** run; commands must be handled manually.  
- `on_cd` – called after a successful directory change (via the `cd` built‑in).  
- `on_exit` – called just before the shell terminates (by `exit` command or signal).

**Note about `repeat`:** The hook executes after checking for the existence of the `PS1` environment variable, so logic for obtaining `PS1` before the hook execution is necessary.

### Example of `repeat`
```
fn repeat() {
    // Get values
    let user = get_user().set_color(76, 181, 148);
    let dir  = get_current_dir().set_color(76, 140, 239);
    // Generate prompt and set it in the PS1 environment variable
    let ps1 = user + " " + dir + " >> ".set_color(76, 239, 100);
    set_var("PS1", ps1);
}
```

### Example of `on_input`
```
// File .sbshrc.rhai – example of using the on_input hook

fn log_command(cmd) {
    let log_path = get_var("HOME") + "/.sbsh_history.log";
    let timestamp = "[" + get_time("%Y-%m-%d %H:%M:%S") + "] ";
    let entry = timestamp + cmd + "\n";
    write_file(log_path, entry, "append");
}

fn on_input(line) {
    log_command(line);

    // Auto-replace 'g' with 'git'
    if line == "g" {
        let (out, err, code) = run_command("git status");
        print(out + err);
        return;
    }
    if line.starts_with("g ") {
        let git_cmd = "git " + line.slice(2);
        let (out, err, code) = run_command(git_cmd);
        print(out + err);
        return;
    }

    // If nothing special, execute the command as is
    let (out, err, code) = run_command(line);
    print(out + err);
}
```

### New hooks: `on_cd` and `on_exit`

#### `on_cd(old_path, new_path)`
Called after a successful directory change. It receives two strings: the previous working directory and the new one. Useful for updating environment variables, loading project‑specific settings, or logging.

**Example:**
```
fn on_cd(old, new) {
    // Activate Python virtual environment if present
    let venv = new + "/venv/bin/activate";
    if is_file(venv) {
        // Assuming a hypothetical `source` function; here we just print
        print("Activate venv in " + new);
    }
    // Update prompt with git branch
    if is_git_repo() {
        set_var("PS1", "(" + get_git_branch() + ") $ ");
    }
}
```

#### `on_exit(exit_code)`
Called just before the shell terminates, with the exit code that will be returned to the parent process. Useful for cleanup tasks, saving state, or logging session end.

**Example:**
```
fn on_exit(code) {
    let log = get_var("HOME") + "/.sbsh_exit.log";
    let msg = "Exited with code " + code + " at " + get_time("%c") + "\n";
    write_file(log, msg, "append");
}
```

---

## SBSH-Specific Functions

SBSH adds many unique functions for working with the shell and the system. They are grouped by category.

### Value Retrieval (0.1)
| Function | Description |
|----------|-------------|
| `get_user()` | Returns the username. |
| `get_current_dir()` | Returns the current directory. |

### Environment Variables(0.1 and 0.2)
| Function | Description |
|----------|-------------|
| `get_var(name)` | Returns the value of a variable. |
| `set_var(name, value)` | Creates or updates an environment variable. |
| `del_var(name)` | Removes an environment variable. Added in 0.2| 

### Aliases (0.1)
| Function | Description |
|----------|-------------|
| `alias_add(name, replacement)` | Creates an alias for a command. |
| `alias_get(name)` | Returns what the alias replaces (or `none`). |
| `alias_list()` | Returns a list of all aliases. |
| `alias_remove(name)` | Deletes an alias by name. |
| `alias_clear()` | Deletes all aliases. |

### String Formatting (0.1)
| Function | Description |
|----------|-------------|
| `set_color(r, g, b)` | Applies an RGB color to the string it is called on. |
| `set_bold(true/false)` | Makes the string bold or normal. |

### Git Integration (0.1)
| Function | Description |
|----------|-------------|
| `is_git_repo()` | Returns `true`/`false` if current directory is a git repository. |
| `get_git_branch()` | Returns the current git branch. |
| `git_is_dirty()` | Checks for uncommitted changes. |
| `git_ahead_behind()` | Returns the number of commits ahead/behind. |

### File System (0.2)
| Function | Description |
|----------|-------------|
| `read_file(path) -> String` | Reads the entire contents of a file as a string. |
| `write_file(path, content, mode)` | Writes `content` to a file. `mode` can be `"overwrite"` or `"append"`. |
| `is_file(path) -> Bool` | Returns `true` if the path points to a regular file. |
| `is_dir(path) -> Bool` | Returns `true` if the path points to a directory. |
| `get_file(path) -> String` | Returns the last component of the path (file or directory name). Returns empty string if path ends with `/`. |

### Timing (0.2)
| Function | Description |
|----------|-------------|
| `start_timer(name)` | Starts a timer with the given name. |
| `stop_timer(name) -> Float` | Stops the named timer and returns the elapsed time in seconds (as a floating‑point number). |
| `get_time(format) -> String` | Returns the current local time formatted according to the given `format` string (using `strftime` conventions). |

**Examples with new functions:**
```
// Check if a directory exists before creating it
if !is_dir("/tmp/mydir") {
    write_file("/tmp/mydir/note.txt", "Created at " + get_time("%F"), "overwrite");
}

// Measure how long a command takes
start_timer("mycmd");
run_command("sleep 2");
let elapsed = stop_timer("mycmd");
print("Command took " + elapsed + " seconds");

// Get just the filename from a path
let fname = get_file("/home/user/docs/report.pdf");   // returns "report.pdf"
```

### Miscellaneous (0.1)
| Function | Description |
|----------|-------------|
| `load_plugin(path)` | Runs the specified Rhai script on another Rhai engine. |
| `system(command)` | Executes a command (alias for `run_command`). |

---

## Technical Features

SBSH is a single‑threaded program, therefore:
- Infinite loops block the program; instead, use the `repeat` hook or provide a way to exit them.
- Long calculations in the `repeat` hook and the functions/files it calls are not recommended; avoid complex computations.
- Without the `.sbshrc.rhai` file in the home directory and the `PS1` variable declared in it, SBSH will not start.