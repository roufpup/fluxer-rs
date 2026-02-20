#!/bin/bash
set -e

sudo apt-get update && sudo apt-get install -y clang
cargo install --locked wild-linker

code-server --install-extension rust-lang.rust-analyzer