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
# Jobs section
################################################################################

def job_deny [] {
    log info "job_deny" "Check bans, licenses and sources"
    cargo deny --workspace check bans licenses sources
}

def job_advisories [] {
    log info "job_advisories" "Check advisories"
    cargo deny --workspace check advisories
}

def job_check [] {
    log info "job_check" "Check style"
    cargo +nightly fmt --all -- --check

    log info "job_check" "Check with `cargo check`"
    cargo check --workspace --all-features --all-targets

    log info "job_check" "Check with Clippy"
    cargo clippy --workspace --all-features --all-targets -- --deny warnings
}

def job_build [] {
    log info "job_build" "Build"
    cargo build --workspace --all-features --all-targets --verbose
}

def job_test [] {
    log info "job_test" "Run tests"
    cargo test --workspace --all-features --all-targets --verbose
}

def job_publish [tag: string] {
    log info "job_publish" "Extract tag name"
    let tag_name = $tag

    log info "job_publish" "Check if dry-run is needed"
    let run_mode = (
        if $tag_name =~ r#'^v[0-9]+\.[0-9]+\.[0-9]+-dry-run$'# {
            "--dry-run"
        } else {
            ""
        }
    )

    log info "job_publish" "Cargo package catalyser"
    cargo package -p catalyser

    log info "job_publish" "Cargo package catalyser-derive"
    cargo package -p catalyser-derive

    log info "job_publish" "Cargo publish catalyser"
    cargo publish $run_mode -p catalyser

    log info "job_publish" "Cargo publish catalyser-derive"
    cargo publish $run_mode -p catalyser-derive
}

################################################################################
# On section
################################################################################

def push_branch [branch: string] {
    if $branch == "main" {
        log info "push_branch" $"Do workflow on push branch[($branch)]..."
        job_deny
        try { job_advisories }
        job_check
        job_build
        job_test
    } else {
        print $"push_branch> Skip workflow on push branch[($branch)]..."
    }
}

def push_tag [tag: string] {
    if $tag =~ r#'^v[0-9]+\.[0-9]+\.[0-9]+(-([0-9]+|dry-run))?$'# {
        log info "push_tag" $"Do workflow on push tag[($tag)]..."
        job_deny
        try { job_advisories }
        job_check
        job_build
        job_test
        job_publish $tag
    } else {
        log info "push_tag" $"Skip workflow on push tag[($tag)]..."
    }
}

def pull_request [dest_branch: string] {
    if $dest_branch == "main" {
        (log
            info
            "pull_request"
            $"Do workflow on pull_request branch[($dest_branch)]..."
        )
        job_deny
        try { job_advisories }
        job_check
        job_build
        job_test
    } else {
        (log
            info
            "pull_request"
            $"Skip workflow on pull_request branch[($dest_branch)]..."
        )
    }
}

################################################################################
# Script
################################################################################

# On push workflow
def "main push" [--branch(-b): string, --tag(-t): string] {
    if ($branch != null and $tag != null) or ($branch == null and $tag == null) {
        error make -u {msg: "Only one of --branch or --tag should be not null"}
    } else if $branch != null {
        push_branch $branch
    } else if $tag != null {
        push_tag $tag
    }
}

# On pull_request workflow
def "main pull_request" [--branch(-b): string = "main"] {
    pull_request $branch
}

# Make script for agwaita
def main []: nothing -> nothing {
    help main
}
