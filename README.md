# aoc-rust

### download & prepare

```shell
YEAR=2023
DAY=06
(mkdir --parents data/year$YEAR/day$DAY && cd data/year$YEAR/day$DAY && aoc download --overwrite --year $YEAR --day $DAY)
cp ./src/template.rs ./src/year$YEAR/day$DAY.rs
```

### solve all challenges

```shell
cargo run
```

### clippy

```shell
cargo clippy --all-targets --all-features
```

### benchmark

```shell
cargo bench --benches year2023::day05::part2
```
