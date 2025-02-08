#!/bin/bash

# Update the dependencies
cargo update

# Run the project
cargo lcheck --bin rem-borrower
cargo build --bin rem-borrower