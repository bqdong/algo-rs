_default:
    just -l

alias f := fmt
alias t := test
alias l := lint

fmt:
    just --fmt --unstable
    cargo fmt -- --config format_code_in_doc_comments=true

lint:
    cargo clippy

fix:
    cargo clippy --fix --allow-dirty --tests

test:
    cargo t
    @echo "\n\033[32mStart to test in release mode!\033[0m\n"
    cargo t --release

run-hook:
    prek run --show-diff-on-failure --color=always --all-files

doc:
    RUSTDOCFLAGS="--html-in-header mathjax.html" cargo doc --no-deps
    # 新版 rustdoc 不生成根跳转页,补一个让根路径自动跳到 crate 页
    printf '<meta http-equiv="refresh" content="0; url=algo_rs/index.html">' > target/doc/index.html

preview:
    uv run python -m http.server 8000 --directory target/doc
