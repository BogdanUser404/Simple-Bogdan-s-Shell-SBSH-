# SBSH – Simple Bogdan's Shell

SBSH is a minimal Unix‑like shell written in Rust. It supports executing external programs, built‑in commands, environment variable manipulation, basic arithmetic expressions, and command separation with semicolons.

## Building

Ensure Rust (stable) is installed. Clone the repository and build:

```bash
git clone <repository-url>
cd ESH
cargo build --release
The executable will be placed in target/release/esh.

Usage
Start the shell by running the executable. The prompt is >>.
Commands can be entered one per line or separated by ; (semicolon).
Lines beginning with # are ignored as comments.
```
Example:
```
text
>> cd /tmp
>> print "Current directory:" ; pwd
>> var msg = "Hello, world!"
>> print $msg
Built‑in Commands
All built‑ins are case‑sensitive.

print – output text and variables
Syntax: print [arguments...] [;]
```
Arguments are separated by whitespace.

If an argument starts with $, it is treated as an environment variable name and its value is substituted.

Otherwise the argument is printed literally.

A semicolon terminates the command.

No newline is added automatically; use print ; to output a blank line.

Examples:
```
text
print Hello world
print "The value is" $VAR
print $HOME ;
print "Line 1" ; print "Line 2"
cd – change current directory
Syntax: cd [directory]
```
Changes the current working directory to directory.

If directory is omitted, changes to $HOME.

On error, an error message is printed.

Examples:
```
text
cd /usr/local
cd
cd ..
var – manage environment variables
Syntax: var [name=value | del name | match ...]
```
Without arguments, lists all environment variables.
```
var name=value (with or without spaces around =) sets variable name to value.

var del name removes the variable name.

var match – evaluates a simple arithmetic expression and stores the result in a variable.

var match – arithmetic expressions
Syntax: var match target = left operand right ;
```
target – name of the variable to store the result.

left and right may be numbers or names of existing numeric variables (no $ needed).

operand is one of +, -, *, /, ^ (exponentiation).

Spaces are flexible; the expression can be written with or without spaces around the operator and =.

A semicolon is required (may be preceded/followed by spaces).

If a variable does not exist or its value is not a valid number, the shell panics with a message.

Examples:
```
text
var x = 10
var y = 20
var match sum = x + y ;
print $sum
var match result = 2.5 * 4 ;
var match a = 100 / 3 ;
var match b = 2 ^ 8
exit – terminate the shell
Syntax: exit [code]
```
Terminates the shell process with the given exit code (default 0).

Examples:
```
text
exit
exit 1
clr – clear the terminal screen
Syntax: clr

Sends the ANSI escape sequence to clear the screen and move the cursor to the top‑left corner.

External Programs
If a command is not a built‑in, SBSH attempts to execute it as an external program using the system PATH (via execvp). The shell waits for the program to finish and prints its exit code if it is non‑zero.
```
Example:
```
text
>> ls -l
>> grep "error" log.txt
>> /bin/echo hello
Environment Variables
All built‑ins that read or write variables operate on the shell’s environment.
```
Variables set with var are inherited by child processes.

Use $NAME inside print to substitute the value.

Command Separator
Multiple commands can be written on one line by separating them with a semicolon ;.
Example:
```
text
cd /tmp ; pwd ; ls
Comments
Any line starting with # is ignored.
```
License
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program.
