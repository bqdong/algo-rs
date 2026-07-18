_default:
    just -l

alias f := fmt
alias t := test
alias l := lint

fmt:
    just --fmt --unstable
    cargo fmt

lint:
    cargo clippy

fix:
    cargo clippy --fix --allow-dirty

test:
    cargo t

run-hook:
    prek run --show-diff-on-failure --color=always --all-files
