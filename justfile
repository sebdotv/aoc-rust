#!/usr/bin/env just --justfile

default:
    just --list

init-day year day:
    mkdir --parents data/year{{ year }}/day{{ day }}
    cd data/year{{ year }}/day{{ day }} && aoc download --overwrite --year {{ year }} --day {{ day }}
    mkdir --parents src/year{{ year }}
    cp --no-clobber ./src/template.rs ./src/year{{ year }}/day{{ day }}.rs

init-today:
    just --justfile {{ justfile() }} init-day {{ datetime("%Y") }} {{ datetime("%d") }}

run *extra_args:
    cargo run --quiet --release {{ extra_args }}

run-day year day:
    just --justfile {{ justfile() }} run -- --year {{ year }} --day {{ day }}

run-current-year:
    just --justfile {{ justfile() }} run -- --year {{ datetime("%Y") }}

run-today:
    just --justfile {{ justfile() }} run-day {{ datetime("%Y") }} {{ datetime("%d") }}
