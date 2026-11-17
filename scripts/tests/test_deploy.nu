use std/assert
use ../deploy.nu [missing_files missing_env]

let root = mktemp -d
assert ((missing_files $root | length) > 0)
let site = ($root | path join public)
mkdir ($site | path join design-system showcase) ($site | path join products watchfaces) ($site | path join privacy)
for f in [index.html dodai.css site.css favicon.png design-system/showcase/showcase.wasm design-system/showcase/app.js products/watchfaces/index.html privacy/index.html] {
    "" | save ($site | path join $f)
}
assert equal (missing_files $root) []

assert equal (missing_env {}) [CLOUDFLARE_API_TOKEN CLOUDFLARE_ACCOUNT_ID]
assert equal (missing_env {CLOUDFLARE_API_TOKEN: "t" CLOUDFLARE_ACCOUNT_ID: ""}) [CLOUDFLARE_ACCOUNT_ID]
assert equal (missing_env {CLOUDFLARE_API_TOKEN: "t" CLOUDFLARE_ACCOUNT_ID: "a"}) []

use ../ci.nu gate
assert equal (gate | first | first 2) [cargo fmt]
assert ((gate | each {|c| $c.0 } | uniq) == [cargo])
