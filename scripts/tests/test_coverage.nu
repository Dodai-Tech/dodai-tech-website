use std/assert
use ../coverage.nu [command IGNORED]

assert ("src/main.rs" =~ $IGNORED)
assert ("src/shaders.rs" =~ $IGNORED)
assert not ("src/site.rs" =~ $IGNORED)
assert equal (command 95 false | where $it == "--fail-under-lines") ["--fail-under-lines"]
assert ((command 90 false) | any {|a| $a == "90" })
assert ((command 95 true) | any {|a| $a == "--html" })
assert ((command 95 false) | any {|a| $a == "--summary-only" })
