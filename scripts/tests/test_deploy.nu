use std/assert
use ../deploy.nu [missing_files missing_env]

let root = mktemp -d
assert ((missing_files $root | length) > 0)
mkdir ($root | path join design-system showcase) ($root | path join products watchfaces) ($root | path join privacy)
for f in [index.html dodai.css site.css favicon.png wrangler.jsonc design-system/showcase/showcase.wasm design-system/showcase/app.js products/watchfaces/index.html privacy/index.html] {
    "" | save ($root | path join $f)
}
assert equal (missing_files $root) []

assert equal (missing_env {}) [CLOUDFLARE_API_TOKEN CLOUDFLARE_ACCOUNT_ID]
assert equal (missing_env {CLOUDFLARE_API_TOKEN: "t" CLOUDFLARE_ACCOUNT_ID: ""}) [CLOUDFLARE_ACCOUNT_ID]
assert equal (missing_env {CLOUDFLARE_API_TOKEN: "t" CLOUDFLARE_ACCOUNT_ID: "a"}) []
