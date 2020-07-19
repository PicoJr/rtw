#!/bin/bash -v

RELEASE_DIR="release$(date +%F-%H-%M-%S)"

rustfmt --check src/**/*.rs &&
    cargo test &&
    docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder cargo build --release &&
    mkdir -p ${RELEASE_DIR} &&
    cp -r ./CHANGELOG.md ./commands.md ./shell-completion.md \
        example/ img/ ./README.md ./LICENSE ${RELEASE_DIR} &&
    cp target/x86_64-unknown-linux-musl/release/rtw ${RELEASE_DIR}/rtw-x86_64-unknown-linux-musl

