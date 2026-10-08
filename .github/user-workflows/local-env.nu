#!/usr/bin/env nu
#-*- nushell-ts -*-

################################################################################
# Utils
################################################################################

def log-levels [] {
    [
        "trace"
        "debug"
        "info"
        "warning"
        "error"
        "critical"
    ]
}

def log [level: string@log-levels, tag: string, message: string] {
    let color = (
        match $level {
            "trace" => (ansi "bold")
            "debug" => (ansi "cyan_bold")
            "info" => (ansi "blue_bold")
            "warning" => (ansi "magenta_bold")
            "error" => (ansi "red_bold")
            "critical" => (ansi "red_reverse")
        }
    )
    let timestamp = $"(date now | format date "%Y-%m-%dT%H:%M:%S%.3f")"
    let fmt_message = $"(ansi reset_bold)($message)(ansi reset)"

    print $"($color)($timestamp)|($level)|($tag)> ($fmt_message)"
}

################################################################################
# Subcommand section
################################################################################

def refresh_trybuild_data [dir: path] {
    with-env { TRYBUILD: "overwrite" } {
        cd catalyser-derive

        log info "refresh_trybuild_data" $"cargo test --test ($dir)"
        cargo test --test $dir
    }
}

################################################################################
# Script
################################################################################

# Refresh all output data for trybuild tests
def "main cargo-derive trybuild" [dir: path] {
    refresh_trybuild_data $dir
}

# Developper helpers for cargo-derive crate
def "main cargo-derive" [] {
    help main cargo-derive
}

# Developper helpers
def main []: nothing -> nothing {
    help main
}
