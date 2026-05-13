#!/bin/bash
set -e
cargo clean
rustup run stable cargo test -p adapter database::model::auth --no-run
