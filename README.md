# aoc-rust

### download & prepare

```shell
YEAR=2023
DAY=12
(mkdir --parents data/year$YEAR/day$DAY && cd data/year$YEAR/day$DAY && aoc download --overwrite --year $YEAR --day $DAY)
cp --no-clobber ./src/template.rs ./src/year$YEAR/day$DAY.rs
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
cargo bench --benches year2023::day12::part2
```
