#!/usr/bin/env bash
sed -i "0,/version = \".*\"/s//version = \"$1\"/" Cargo.toml default.nix
cargo update -p projeto-bd --precise $1
