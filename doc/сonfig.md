# Configuration and Addons

For configuration and addons, the Rhai scripting language is used.

This documentation is dedicated exclusively to the hooks and functions added to Rhai for working with SBSH and environment variables. Documentation for the original Rhai can be found on their website (link: https://rhai.rs/).

## Hooks

SBSH uses two hooks: `on_input` and `repeat`. Hooks are ordinary functions in Rhai.  
`on_input` is called after input and receives the input as an argument. `repeat` is called first in the main loop; its main purpose is to be a single-threaded analog of an infinite loop and update the shell state, for example as below. Note that the hook executes after checking for the existence of the `PS1` environment variable, so logic for obtaining the `PS1` environment variable before the hook execution is necessary.

Example code for the `repeat` hook in practice:
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

The `on_input` hook is called after the user enters text and takes the user input as an argument. After the hook is called, the standard command processing does not run, and commands must be handled manually. This hook is used for advanced aliases and auto-completion.

Example code below:
```
// File .sbshrc.rhai – example of using the on_input hook

// Command logging function (appends the line to a file)
fn log_command(cmd) {
    // Open log file in the home directory
    let log_path = get_var("HOME") + "/.sbsh_history.log";
    // Append command and time (can add timestamp)
    let timestamp = "[" + get_current_time() + "] "; // if time function exists
    let entry = timestamp + cmd + "\n";
    // Here we need a function to write to a file, e.g., write_file
    // Assume such a function exists in the API (can be added)
    // write_file(log_path, entry, "append");
    // For demonstration, just print to terminal
    print("Logging: " + cmd);
}

fn on_input(line) {
    // Log all commands
    log_command(line);

    // Auto-replace 'g' with 'git'
    if line == "g" {
        let (out, err, code) = run_command("git status");
        print(out + err);
        return; // command handled
    }
    if line.starts_with("g ") {
        let git_cmd = "git " + line.slice(2);
        let (out, err, code) = run_command(git_cmd);
        print(out + err);
        return;
    }

    // Warning before a dangerous command
    if line.contains("rm -rf /") {
        print("⚠️  This will delete the entire system! Type 'yes' to confirm.");
        // Can request confirmation via another mechanism, but here just output warning
        // and do not execute the command
        return;
    }

    // If nothing special, execute the command as is
    let (out, err, code) = run_command(line);
    print(out + err);
}
```

## SBSH-Specific Functions

SBSH adds a fair number of unique functions for working with SBSH and the system.  
The functions are grouped by category.

### Value Retrieval

| Function | Description |
|----------|-------------|
| `get_user()` | Returns the username. |
| `get_current_dir()` | Returns the current directory. |

### Environment Variables

| Function | Description |
|----------|-------------|
| `get_var(name)` | Returns the value of a variable. |
| `set_var(name, value)` | Creates an environment variable (if absent) and moves the value into it. |

### Aliases

| Function | Description |
|----------|-------------|
| `alias_add(name, replacement)` | Creates an alias for a command. |
| `alias_get(name)` | Returns what the alias replaces (if the alias exists); if the alias is absent, returns `none`. |
| `alias_list()` | Returns a list of all aliases. |
| `alias_remove(name)` | Deletes an alias by name. |
| `alias_clear()` | Deletes all aliases. |

### String Formatting

| Function | Description |
|----------|-------------|
| `set_color(r, g, b)` | Applies an RGB color to the string it is called on. |
| `set_bold(true/false)` | Makes the string it is applied to bold/non-bold. |

### Git Integration

| Function | Description |
|----------|-------------|
| `is_git_repo()` | Returns `true`/`false` depending on whether the current directory is a git repository. |
| `get_git_branch()` | Returns the git branch of the project in the current directory. |
| `git_is_dirty()` | Checks if there are uncommitted changes in the repository; returns `true`/`false`. |
| `git_ahead_behind()` | Returns the number of uncommitted changes. |

### Miscellaneous

| Function | Description |
|----------|-------------|
| `load_plugin(path/to/file.rhai)` | Runs the specified Rhai script on another Rhai engine. |
| `system(command)` | Executes a command. |

## A Number of Technical Features

SBSH is a single-threaded program, therefore:
- Infinite loops block the program; instead, use the `repeat` hook or provide a way to exit them.
- Long calculations in the `repeat` hook and the functions/files it calls are not recommended; avoid complex computations.
- Without the `.sbshrc.rhai` file in the home directory and the `PS1` variable declared in it, SBSH will not start.