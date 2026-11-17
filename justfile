# Dodai Tech website. Scripts are Nushell: `nu scripts/...`.

default:
    @just --list

# Regenerate every page, the stylesheet and the showcase pictures.
gen:
    cd gen && CARGO_INCREMENTAL=0 cargo run -q --release

# Rebuild the browser showcase (wasm) from a dodai-ui checkout.
showcase dodai_ui="~/Projects/dodai-ui":
    nu scripts/build-showcase.nu --dodai-ui {{dodai_ui}}

# Serve the site at http://127.0.0.1:8765
serve:
    python3 -m http.server 8765 --bind 127.0.0.1

# Script tests.
test:
    nu scripts/tests/run.nu

# Dry run of the Cloudflare deploy.
check: test
    nu scripts/deploy.nu --dry-run

# Upload the committed site (needs CLOUDFLARE_API_TOKEN and CLOUDFLARE_ACCOUNT_ID).
deploy:
    nu scripts/deploy.nu
