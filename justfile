
devsetup:
    cp dev/hooks/* .git/hooks

fmt:
    cargo +nightly fmt --all

chk:
    cargo check

test:
    cargo test

lint:
    cargo clippy
