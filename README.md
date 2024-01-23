# LTLS

This repository is for running LTLS seamlessly.

## Steps To Run:

### Step 1: Get Rust

To run this, first rust must be installed on your machine:
https://www.rust-lang.org/learn/get-started

### Step 2: Git Access

The sole dependency of this repo is https://github.com/TechnionFV/rust-formal-verification.git

Make sure you can access it since it is a private repo.

### Step 3: Run

The best performance is reached when running in `release` mode, so the command to run this program would be:
```
cargo run --release <AIGER file>
```

## Notes: 

### Compiling C++

This rust program depends on other rust programs that may depend on C++ code.
So you might need to have Clang on your machine.

### Running offline

Here is some information I found about downloading the rust compiler and installing offline:
https://forge.rust-lang.org/infra/other-installation-methods.html
Furthermore, some information that might be helpful in the case when you want to use cargo offline:
https://www.reddit.com/r/rust/comments/137hmah/rust_offline/
More specifically, running with 'cargo run --release --offline'

### Compiling And Running Elsewhere

Another solution is to compile the program anywhere, then send the executable to where it needs to go.
To compile the executable run:
```
cargo build --release
```

Then you'll find the executable under `./target/release/ltls`

## Using different C++ compilers

To compile the project you need to have a C++ compiler in order to compile CaDiCal.
To set the compiler certain environment variables must be set before trying to compile.

The C++ standard library may be linked to the crate target. 
By default it's:
1. `libc++` for macOS, FreeBSD, and OpenBSD
2. `libc++_shared` for Android, nothing for MSVC
3. `libstdc++` for anything else. 
It can be changed by setting the `CXXSTDLIB` environment variable.

### Using g++

Run these commands in order.
```
cargo clean
unset CRATE_CC_NO_DEFAULTS
unset CXXFLAGS
unset CXXSTDLIB
export CXX=/usr/bin/g++
cargo test
```



