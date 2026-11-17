#!/usr/bin/env nu
# Deploys the committed site to Cloudflare (Workers static assets, project `dodaitech`).
#   nu scripts/deploy.nu [--dry-run]
# Needs Node and, unless --dry-run, CLOUDFLARE_API_TOKEN and CLOUDFLARE_ACCOUNT_ID in the
# environment (CI secrets, or your shell on another machine). The site is generated and
# committed; this script only uploads it.

const REQUIRED = [index.html dodai.css site.css favicon.png wrangler.jsonc design-system/showcase/showcase.wasm design-system/showcase/app.js products/watchfaces/index.html privacy/index.html]

# Files of the site that are missing under `root`.
export def missing_files [root: string]: nothing -> list<string> {
    $REQUIRED | where {|f| not ($root | path join $f | path exists) }
}

# Names of the Cloudflare variables that are not set.
export def missing_env [env_vars: record]: nothing -> list<string> {
    [CLOUDFLARE_API_TOKEN CLOUDFLARE_ACCOUNT_ID] | where {|k| ($env_vars | get -o $k | default "" | is-empty) }
}

def main [--dry-run] {
    let root = ($env.FILE_PWD | path dirname)
    let gone = (missing_files $root)
    if ($gone | is-not-empty) { error make --unspanned { msg: $"site incomplete, run the generator first: ($gone | str join ', ')" } }
    let unset = (missing_env $env)
    if (not $dry_run) and ($unset | is-not-empty) { error make --unspanned { msg: $"not set: ($unset | str join ', ')" } }
    cd $root
    let args = if $dry_run { [deploy --dry-run] } else { [deploy] }
    ^npx -y wrangler@latest ...$args
}
