list:
    just --list

# ================================================================
#   Example `.vscode/settings.json` for `rust-analyzer`:
# ================================================================

# {
#     "rust-analyzer.check.overrideCommand": [
#         "just",
#         "on-save",
#     ],
#     "rust-analyzer.checkOnSave": true,
# }

# ================================================================
#   Smaller scripts
# ================================================================

# Run ripgrep, but don't return an error if nothing matched.
[group("ripgrep")]
rg-maybe-no-match *args:
    @rg {{ args }} || [ $? -eq 1 ]

# Find lines not ending in a comma, where the next line starts with `]`, `)`, or `>`.
[group("ripgrep")]
find-possible-missing-commas: \
    (rg-maybe-no-match ''' -U '[^,]\n[ ]*\]' ''') \
    (rg-maybe-no-match ''' -U '[^,]\n[ ]*\)' ''') \
    (rg-maybe-no-match ''' -U '[^,]\n[ ]*>' ''')

# Find any `#[allow(...)]` attribute, or to be precise, find `[allow(`.
[group("ripgrep")]
find-allow-attributes: (rg-maybe-no-match '"\[allow\("')

# Find any possible sites of unsafe code.
[group("ripgrep")]
find-unsafe-code: (rg-maybe-no-match '"unsafe_code|unsafe"')

# ================================================================
#   Check and Clippy
# ================================================================

check *args:
    cargo +stable hack clippy --rust-version --feature-powerset {{args}}

clippy *args:
    cargo +stable hack clippy --rust-version --feature-powerset {{args}}

test *args:
    cargo +stable hack test --rust-version --feature-powerset {{args}}

show-yoke-ub *args:
    cargo +nightly-2026-01-18 miri run --package yoke-problem {{args}}

[group("on-save")]
on-save: (clippy "--message-format=json")
