#!/bin/bash -v

RELEASE_DIR="release$(date +%F-%H-%M-%S)"

rustfmt --check src/**/*.rs &&
    cargo test &&
    cargo build --release &&
    mkdir -p ${RELEASE_DIR} &&
    cp -r ./CHANGELOG.md ./commands.md ./shell-completion.md example/ img/ ./README.md ./LICENSE target/release/rtw ${RELEASE_DIR}
