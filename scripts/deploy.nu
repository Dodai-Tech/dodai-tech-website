#!/usr/bin/env nu
# Deploys the built site (`public/`) to Cloudflare (Workers static assets, project `dodaitech`).
#   nu scripts/deploy.nu [--build] [--dry-run]
# --build runs `cargo run --release` first (CI does). Needs Node and, unless --dry-run,
# CLOUDFLARE_API_TOKEN and CLOUDFLARE_ACCOUNT_ID in the environment (CI secrets, or your shell).
use lib.nu sh

const REQUIRED = [index.html dodai.css site.css favicon.png design-system/showcase/showcase.wasm design-system/showcase/app.js products/watchfaces/index.html privacy/index.html]

# Files of the built site that are missing under `root/public`.
export def missing_files [root: string]: nothing -> list<string> {
    $REQUIRED | where {|f| not ($root | path join public $f | path exists) }
}

# Names of the Cloudflare variables that are not set.
export def missing_env [env_vars: record]: nothing -> list<string> {
    [CLOUDFLARE_API_TOKEN CLOUDFLARE_ACCOUNT_ID] | where {|k| ($env_vars | get -o $k | default "" | is-empty) }
}

def main [--build, --dry-run] {
    let root = ($env.FILE_PWD | path dirname)
    cd $root
    if $build { sh [cargo run --release --locked] }
    let gone = (missing_files $root)
    if ($gone | is-not-empty) { error make --unspanned { msg: $"site not built, run `just build`: ($gone | str join ', ')" } }
    let unset = (missing_env $env)
    if (not $dry_run) and ($unset | is-not-empty) { error make --unspanned { msg: $"not set: ($unset | str join ', ')" } }
    sh [npx -y wrangler@latest deploy ...(if $dry_run { [--dry-run] } else { [] })]
}
