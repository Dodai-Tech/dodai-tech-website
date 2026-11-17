# Dodai Tech website. Scripting is Nushell: `nu scripts/...`.

default:
    @just --list

# Build the site into public/.
build:
    cargo run --release

# The pull-request gate: fmt, clippy, tests, build, dry-run deploy.
check:
    nu scripts/ci.nu

# Preview public/ at http://127.0.0.1:8765
serve: build
    cd public && python3 -m http.server 8765 --bind 127.0.0.1

# Supply chain: advisories, licences, bans, sources.
audit:
    cargo audit
    cargo deny check

# Fuzz a target (needs nightly + cargo-fuzz): just fuzz chart_points
fuzz target="chart_points" secs="60":
    cargo +nightly fuzz run {{target}} -- -max_total_time={{secs}}

# Rebuild the browser showcase (wasm) from a dodai-ui checkout; commit the result.
showcase dodai_ui="~/Projects/dodai-ui":
    nu scripts/build-showcase.nu --dodai-ui {{dodai_ui}}

# Render the showcase shader pictures (GPU and ffmpeg); commit the result.
shaders:
    cargo run --release -- shaders

# Upload to Cloudflare (needs CLOUDFLARE_API_TOKEN and CLOUDFLARE_ACCOUNT_ID).
deploy:
    nu scripts/deploy.nu --build
