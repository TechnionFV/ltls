# LTLS

This repository is for running LTLS seamlessly.

## Step 1: Get Rust

To run this, first rust must be installed on your machine:
https://www.rust-lang.org/learn/get-started

## Step 2: Git Access

The sole dependency of this repo is https://github.com/TechnionFV/rust-formal-verification.git

Make sure you can access it since it is a private repo.

## Step 3: Run

The best performance is reached when running in `release` mode, so the command to run this program would be:
```
cargo run --release <AIGER file>
```

## Notes: 

You might need to have Clang on your machine.