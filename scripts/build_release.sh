#!/bin/bash

# Update the dependencies
cargo update

# Run the project
cargo lcheck --release --bin rem-borrower
cargo build --release --bin rem-borrower