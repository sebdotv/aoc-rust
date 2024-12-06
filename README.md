# aoc-rust

### download & prepare

```shell
just init-today
```

or:

```shell
just init-day [year] [date]
```

### solve all challenges

```shell
cargo run --quiet --release
```

### clippy

```shell
cargo clippy --all-targets --all-features
```

### benchmark

```shell
cargo bench --benches year2023::day17::part2
```

### profiling

```shell
cargo flamegraph -- --latest --part 2 --only input
```

## Development

### Dependencies

- `anyhow`: idiomatic error handling
- `chrono`: date & time
- `clap`: command line arguments
- `colored`: terminal colors
- `hex`: hexadecimal
- `indexmap`: ordered hashmap
- `itertools`: iterator tools
- `lazy_static`: lazy statics
- `nom`: parser combinators
- `nom_locate`: special input type for `nom` to locate tokens
- `num_enum`: enum from number
- `pathfinding`: pathfinding
- `polyfit-rs`: polynomial fitting
- `regex`: regular expressions
- `strum`: enum traits
- `strum_macros`: enum traits
- `thiserror`: derive Error from enum

Development (test) dependencies:

- `criterion`: benchmarking
- `indoc`: multiline strings