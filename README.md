# SBSH – Simple Bogdan's Shell

SBSH is a minimal Unix‑like shell written in Rust. It supports executing external programs, built‑in commands, environment variable manipulation, basic arithmetic expressions, and command separation with semicolons.

Full documentation is available in the [`/doc`](doc/) directory.
The project's philosophy is brief:
"Do what you want, however you want, but be careful."
## License

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program.

## Using libraries

This project uses the following libraries (all are licensed under permissive terms compatible with GPLv3):

- **libc** (version 0.2.182) - license: MIT OR Apache-2.0  
  Author: The Rust Project Developers

- **shlex** (version 1.3) - license: MIT  
  Author: comex <comexk@gmail.com>

- **rustyline** (version 17.0.2) - license: MIT  
  Author: Katsu Kawakami

- **rhai** (version 1.24.0) - license: MIT OR Apache-2.0  
  Author: Rhai Foundation

- **whoami** (version 0.5) - license: MIT OR Apache-2.0  
  Author: Artyom Pavlov [and others]

- **lazy_static** (version 1.5.0) - license: MIT OR Apache-2.0  
  Author: Marvin Löbel

- **git2** (version 0.20) - license: MIT OR Apache-2.0  
  Author: Alex Crichton and contributors

- **iridescent** (version 0.2.1) - license: MIT  
  Author: Owen G. De Long <owen.g.delong@gmail.com>

- **shh** (version 1.0) - license: MIT OR Apache-2.0  
  Author: Steven Roose