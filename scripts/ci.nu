#!/usr/bin/env nu
# The pull-request gate: format, lints, Rust and script tests, and a build of the site.
#   nu scripts/ci.nu
use lib.nu sh

# The cargo commands of the gate, in order.
export def gate []: nothing -> list<list<string>> {
    [
        [cargo fmt --all -- --check]
        [cargo clippy --all-targets --locked -- -D warnings]
        [cargo test --locked]
        [cargo run --release --locked]
    ]
}

def main [] {
    for c in (gate) { print $"==> ($c | str join ' ')"; sh $c }
    print "==> nu scripts/tests/run.nu"
    sh [nu scripts/tests/run.nu]
    print "==> nu scripts/deploy.nu --dry-run"
    sh [nu scripts/deploy.nu --dry-run]
}
