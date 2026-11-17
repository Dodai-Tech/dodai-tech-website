# Runs every scripts/tests/test_*.nu (plain scripts: a failing `std assert` exits non-zero).
#   nu scripts/tests/run.nu

def main [] {
    mut failed = []
    for f in (glob ($env.FILE_PWD | path join "test_*.nu") | sort) {
        let r = do { ^nu $f } | complete
        if $r.exit_code == 0 { print $"✓ ($f | path basename)" } else {
            print -e $"✗ ($f | path basename)\n($r.stderr)"
            $failed = ($failed | append $f)
        }
    }
    if ($failed | is-not-empty) { exit 1 }
}
