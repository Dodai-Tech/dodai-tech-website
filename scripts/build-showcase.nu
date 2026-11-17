#!/usr/bin/env nu
# Builds the Dodai showcase for the browser into static/design-system/showcase/ (committed):
#   nu scripts/build-showcase.nu [--dodai-ui ~/Projects/dodai-ui]
def main [--dodai-ui: string = "~/Projects/dodai-ui"] {
    let src = ($dodai_ui | path expand)
    let out = ($env.FILE_PWD | path dirname | path join static design-system showcase)
    with-env { CARGO_INCREMENTAL: "0" } {
        cargo build --release --target wasm32-unknown-unknown -p dodai-showcase-web --manifest-path ($src | path join Cargo.toml)
    }
    cp ($src | path join target wasm32-unknown-unknown release dodai_showcase_web.wasm) ($out | path join showcase.wasm)
    cp ($src | path join crates dodai-topcoat examples topcoat_showcase app.js) ($out | path join app.js)
}
