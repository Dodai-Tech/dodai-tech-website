use std/assert
use ../remotes.nu [REMOTES targets git_args]

assert equal ($REMOTES | columns) [origin gitea-starscream]
assert ($REMOTES.origin | str starts-with "git@github.com:")
assert equal (targets [origin other]) [origin]
assert equal (targets []) []
assert equal (git_args push origin main) [push --follow-tags origin main]
assert equal (git_args push gitea-starscream main --force) [push --follow-tags --force gitea-starscream main]
assert equal (git_args push-tags origin main) [push origin --tags]
assert equal (git_args pull origin main) [pull origin main]
assert (try { git_args nope origin main; false } catch { true })
