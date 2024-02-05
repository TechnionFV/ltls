# LTLS

This repository is for running LTLS seamlessly.

## Steps To Run:

### Step 1: Get Rust

To run this, first rust must be installed on your machine:
https://www.rust-lang.org/learn/get-started

### Step 2: Git Access

The sole dependency of this repo is https://github.com/TechnionFV/rust-formal-verification.git

Make sure you can access it since it is a private repo, this can be accomplished by asking me for access.

After getting access you should be cache your github credentials in your terminal:
https://docs.github.com/en/get-started/getting-started-with-git/caching-your-github-credentials-in-git

You can also cache your credentials in an unsecure manner by running 
```
git config --global credential.helper store
git pull
```
Then inserting your username and a one-time-password that can be requested in 
https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-personal-access-token-classic

### Step 3: Build

The best performance is reached when running in `release` mode, so the command to run this program would be:
```
cargo run --release <AIGER file>
```

This command fetches all the dependencies and compiles the code, if step 2 is not done (you were not granted access OR you did not cache your credentials) this command will fail.

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

## Using different C++ compilers and linkers

To compile the project you need to have a C++ compiler in order to compile CaDiCal.
To set the compiler certain environment variables must be set before trying to compile.

The C++ standard library may be linked to the crate target. 
By default it's:
1. `libc++` for macOS, FreeBSD, and OpenBSD
2. `libc++_shared` for Android, nothing for MSVC
3. `libstdc++` for anything else. 
It can be changed by setting the `CXXSTDLIB` environment variable.

### Using c++

Run these commands in order.
```
cargo clean
unset CRATE_CC_NO_DEFAULTS
unset CXXFLAGS
unset CXXSTDLIB
unset RUSTFLAGS
export CXX=/usr/bin/g++
cargo test
```

### Using Clang

First you should get the C++ library that works with Clang using one of these commands (not sure which specific one, but the first should work):
```
sudo apt install libc++-dev
sudo apt install libc++abi-dev
sudo apt install libstdc++-dev
```

Then run these commands in order:
```
cargo clean
unset CRATE_CC_NO_DEFAULTS
unset CXXFLAGS
unset CXXSTDLIB
unset RUSTFLAGS
export CXX=clang++
cargo test
```

### Using LLD

First get the linker using:
```
sudo apt install lld
```

Then run these commands in order:
```
cargo clean
unset CRATE_CC_NO_DEFAULTS
unset CXXFLAGS
unset CXXSTDLIB
export RUSTFLAGS="-C link-arg=-fuse-ld=lld"
export CXX=g++
cargo test
```

### Using g++ for linking

There are situations, when running with a different g++ compiler than the one that `PATH` points to where the C++ standard library would not be found.
To fix this, you need to tell rust compiler to link with the same version directly.

Run these commands in order:
```
cargo clean
unset CRATE_CC_NO_DEFAULTS
unset CXXFLAGS
unset CXXSTDLIB
export RUSTFLAGS="-C linker=g++"
export CXX=g++
cargo test
```


