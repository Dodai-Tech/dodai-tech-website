# Runs an external command (a list: program, then arguments) and stops the script when it fails.
export def sh [cmd: list<string>] {
    ^($cmd | first) ...($cmd | skip 1)
    if $env.LAST_EXIT_CODE != 0 {
        error make --unspanned { msg: $"($cmd | str join ' ') failed with ($env.LAST_EXIT_CODE)" }
    }
}
