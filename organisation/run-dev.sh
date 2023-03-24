#!/bin/bash

# Source: https://superuser.com/questions/1684538/run-8-commands-in-parallel-in-equalized-tmux-panes

# run peers in split windows
tmux \
split-window \
'RUST_BACKTRACE=1 ORG_PROFILE=peer_1 cargo run --bin organisation; zsh'\; \
split-window \
'RUST_BACKTRACE=1 ORG_PROFILE=peer_2 cargo run --bin organisation; zsh'\;

# run coordinator
RUST_BACKTRACE=1 cargo run --bin coordinator

tmux select-layout main-horizontal
