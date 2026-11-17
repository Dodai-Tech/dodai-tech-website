# Dodai Tech website

`dodaitech.dev`, hosted on Cloudflare (Workers static assets, project `dodaitech`). A Rust project:
`src/` generates the pages with `dodai-topcoat`, `dodai-icons` and `dodai-core` from crates.io, and
writes them with the hand-made files of `static/` (stylesheet, pictures, privacy page, showcase wasm)
into `public/`, which is what gets deployed. `public/` is not committed.

| Task | Command |
|---|---|
| Build into `public/` | `just build` |
| Preview at http://127.0.0.1:8765 | `just serve` |
| Pull-request gate (fmt, clippy, tests, build, dry-run deploy) | `just check` |
| Advisories, licences, bans, sources | `just audit` |
| Fuzz a chart target | `just fuzz chart_points` |
| Deploy | `just deploy` |
| Rebuild the browser showcase (needs a `dodai-ui` checkout) | `just showcase ~/Projects/dodai-ui` |
| Render the showcase shader pictures (GPU + ffmpeg) | `just shaders` |

The showcase wasm and the shader pictures come from the private `dodai-ui` workspace or need a GPU,
so they are built locally and committed under `static/design-system/showcase/`.

## Another machine

Install Rust, Node and [Nushell](https://www.nushell.sh), clone `git@github.com:Dodai-Tech/dodaitech-website.git` (Gitea mirror: `gitea@192.168.1.44:sorin/dodaitech-website.git`), `just check`. To deploy, set
`CLOUDFLARE_API_TOKEN` (permission *Workers Scripts: Edit*) and `CLOUDFLARE_ACCOUNT_ID`, then `just deploy`.

## CI (GitHub and Gitea Actions)

| Workflow | When | What |
|---|---|---|
| `ci.yml` | pull request, push to `main` | `nu scripts/ci.nu` |
| `deploy.yml` | push to `main`, by hand | build and `wrangler deploy` |
| `security.yml` | nightly, dependency changes | `cargo audit`, `cargo deny check` |
| `fuzz.yml` | nightly, source changes | 60 s per target in `fuzz/` |

Add `CLOUDFLARE_API_TOKEN` and `CLOUDFLARE_ACCOUNT_ID` as secrets on each remote that deploys. Deploying
from both GitHub and Gitea is the same site; enable the deploy workflow on one of them only.
