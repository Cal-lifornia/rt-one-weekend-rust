#!/usr/bin/env sh
cargo build --target x86_64-pc-windows-gnu &&
cp target/x86_64-pc-windows-msvc/debug/rt-one-weekend.exe . &&
exec ./rt-one-weekend.exe "$@"
