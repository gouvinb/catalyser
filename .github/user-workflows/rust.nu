#!/usr/bin/env nu
#-*- nushell-ts -*-

use std/log

################################################################################
# Jobs section
################################################################################

def job_deny [] {
    print "Check bans, licenses and sources"
    cargo deny --workspace check bans licenses sources
}

def job_advisories [] {
    print "Check advisories"
    cargo deny --workspace check advisories
}

def job_check [] {
    print "Check style"
    cargo +nightly  fmt --all -- --check

    print "Check with `cargo check`"
    cargo check --workspace --all-features --all-targets

    print "Check with Clippy"
    cargo clippy --workspace --all-features --all-targets  -- --deny warnings
}

def job_build [] {
    print "Build"
    cargo build --workspace --all-features --all-targets --verbose
}

def job_test [] {
    print "Run tests"
    cargo test --workspace --all-features --all-targets --verbose
}

def job_publish [ tag: string ] {
    print "Extract tag name"
    let tag_name = $tag

    print "Check if dry-run is needed"
    let run_mode = (
        if $tag_name =~ r#'^v[0-9]+\.[0-9]+\.[0-9]+-dry-run$'# {
            "--dry-run"
        } else {
            ""
        }
    )

    print "Cargo package catalyser"
    cargo package -p catalyser

    print "Cargo package catalyser-derive"
    cargo package -p catalyser-derive

    print "Cargo publish catalyser"
    cargo publish $run_mode -p catalyser

    print "Cargo publish catalyser-derive"
    cargo publish $run_mode -p catalyser-derive
}


################################################################################
# On section
################################################################################

def push_branch [ branch: string ] {
    if $branch == "main" {
        print $"Do workflow on push branch[($branch)]..."
        job_deny
        try { job_advisories }
        job_check
        job_build
        job_test
    } else {
        print $"Skip workflow on push branch[($branch)]..."
    }
}

def push_tag [ tag: string ] {
    if $tag =~ r#'^v[0-9]+\.[0-9]+\.[0-9]+(-([0-9]+|dry-run))?$'# {
        print $"Do workflow on push tag[($tag)]..."
        job_deny
        try { job_advisories }
        job_check
        job_build
        job_test
        job_publish $tag
    } else {
        print $"Skip workflow on push tag[($tag)]..."
    }
}

def pull_request [ dest_branch: string ] {
    if $dest_branch == "main" {
        print $"Do workflow on pull_request branch[($dest_branch)]..."
        job_deny
        try { job_advisories }
        job_check
        job_build
        job_test
    } else {
        print $"Skip workflow on pull_request branch[($dest_branch)]..."
    }
}

################################################################################
# Script
################################################################################

# On push workflow
def "main push" [
    --branch(-b): string
    --tag(-t): string
] {
    if ($branch != null and $tag != null) or ($branch == null and $tag == null) {
        error make -u { msg: "Only one of --branch or --tag should be not null" }
    } else if $branch != null {
        push_branch $branch
    } else if $tag != null {
        push_tag $tag
    }
}

# On pull_request workflow
def "main pull_request" [
    --branch(-b): string = "main"
] {
    pull_request $branch
}

# Make script for agwaita
def main []: [nothing -> nothing] {
  help main
}
