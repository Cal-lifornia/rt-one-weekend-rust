default:
  @just --list


# Run 'cargo run' on the project
run *ARGS:
  cargo run {{ARGS}}

build:
  cargo build

# Run criterion benchmark with flamegraph on the supplied benchmark
bench BENCHMARK:
  cargo flamegraph --root --bench {{BENCHMARK}} -o target/flamegraph_{{BENCHMARK}}.svg -- --bench

wrun *FEATURES:
  #!/usr/bin/env sh
  cargo build --target x86_64-pc-windows-gnu --features {{FEATURES}} &&
  exec target/x86_64-pc-windows-gnu/debug/rt-one-weekend.exe "$@"
