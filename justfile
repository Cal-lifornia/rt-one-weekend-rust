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
