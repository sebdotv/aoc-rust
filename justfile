#!/usr/bin/env just --justfile

default:
    just --list

init year day:
    mkdir --parents data/year{{ year }}/day{{ day }}
    cd data/year{{ year }}/day{{ day }} && aoc download --overwrite --year {{ year }} --day {{ day }}
    cp --no-clobber ./src/template.rs ./src/year{{ year }}/day{{ day }}.rs

init-today:
    just --justfile {{ justfile() }} init {{ datetime("%Y") }} {{ datetime("%d") }}

run year day:
    cargo run --release -- --year {{ year }} --day {{ day }}

run-today:
    just --justfile {{ justfile() }} run {{ datetime("%Y") }} {{ datetime("%d") }}
