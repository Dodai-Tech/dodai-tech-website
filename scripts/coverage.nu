#!/usr/bin/env nu
# Test coverage of the site generator, with a floor.
#   nu scripts/coverage.nu [--min 95] [--html]
# Needs `cargo install cargo-llvm-cov` and `rustup component add llvm-tools-preview`.
# `main.rs` (argument handling) and `shaders.rs` (needs a GPU and ffmpeg, so it only runs on a
# developer machine) are left out of the measure.
use lib.nu sh

# Files that the measure leaves out.
export const IGNORED = '(main|shaders)\.rs$'

# The cargo-llvm-cov command line.
export def command [min: int, html: bool]: nothing -> list<string> {
    [cargo llvm-cov --release --locked --ignore-filename-regex $IGNORED --fail-under-lines ($min | into string)]
    | append (if $html { [--html] } else { [--summary-only] })
}

def main [--min: int = 95, --html] {
    sh (command $min $html)
}
