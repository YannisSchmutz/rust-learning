- Build system
- and package manager
- Built in to rust
	- (Unlike other low-level programming languages where this can be a pain-point)


### Create a cargo package

```bash
cargo new hello_cargo
```

Creates several things
- Cargo.toml
	- Configuration file for this package
	- Some metadata
	- dependencies
- src foler
- in some? cases a .gitignore

### Build
In package main directory.
Execute if you add new dependencies. Otherwise "run" seems to be enough.

```bash
cargo build
```

Creates
- Cargo.lock - exact dependencies
- target/
	- debug/ - contains, among others, our executable.


### Run
```bash
cargo run
```
Run seems to build the project as well.

### Check for errors only
```bash
cargo check
```
Checks for errors without execution - a lot faster than build / run.
