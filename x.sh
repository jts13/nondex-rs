#!/usr/bin/env bash

set -euo pipefail

# TODO(toms): add 'usage' `--help` option with explanation of options
# TODO(toms): use `getopts` for more robustness?
# TODO(toms): eventually move this functionality into the `nondex` crate

# TODO(toms): temporary - list of popular crates (pulled from initial Python script)
#crates = [
#    # Most stars on GitHub
#    ("bat", "https://github.com/sharkdp/bat"),
#    ("fd", "https://github.com/sharkdp/fd"),
#    ("ripgrep", "https://github.com/BurntSushi/ripgrep"),
#    ("ruff", "https://github.com/astral-sh/ruff"),
#
#    # Most downloaded on crates.io
#    # ("syn", "https://github.com/dtolnay/syn"),
#    # ("proc-macro2", "https://github.com/dtolnay/proc-macro2"),
#    # ("quote", "https://github.com/dtolnay/quote"),
#    ("bitflags", "https://github.com/bitflags/bitflags"),
#    ("libc", "https://github.com/rust-lang/libc"),
#    ("hashbrown", "https://github.com/rust-lang/hashbrown"),
#    ("rust-base64", "https://github.com/marshallpierce/rust-base64"),
#    ("cfg-if", "https://github.com/alexcrichton/cfg-if"),
#    ("rand-core", "https://github.com/rust-random/rand-core"),
#    ("rand", "https://github.com/rust-random/rand"),
#    ("serde", "https://github.com/serde-rs/serde"),
#    ("regex-syntax", "https://github.com/rust-lang/regex"),
#    ("itoa", "https://github.com/dtolnay/itoa"),
#]

function x-test() {
  local REPO
  local CRATE

  while [[ $# -gt 0 ]]; do
    case $1 in
      --repo)
        REPO="$2"
        shift
        shift
        ;;
      --crate)
        CRATE="$2"
        shift
        shift
        ;;
      *)
        echo "Unknown option: ${1}"
        exit 1
        ;;
    esac
  done

  export RUSTFLAGS=""
  export __CARGO_TESTS_ONLY_SRC_ROOT="$PWD/vendor/rust-src-nightly/rust-src/lib/rustlib/src/rust/library"

  if [[ -n "${REPO}" && -n "${CRATE}" ]]; then
    local REPO_NAME="${CRATE}"

    # TODO(toms): detect --target based on host platform (currently hard-coded to macOS)
    # TODO(toms): avoid `|| true` (and hopefully also `rm -rf`)

    (cd .nondex \
      && git clone "${REPO}" "${REPO_NAME}" || true \
      && cd "${REPO_NAME}" \
      && cargo clean \
      && cargo test --all-features --release --target=aarch64-apple-darwin -Zbuild-std -- -Zunstable-options --format=json
    )
  elif [[ -n "${CRATE}" ]]; then
    cargo clean # TODO(toms): avoid doing this every time?
    cargo test --all-features --release -p "${CRATE}" --target=aarch64-apple-darwin -Zbuild-std -- -Zunstable-options --format=json
  else
    echo "Must specify either --repo or --crate"
    exit 1
  fi
}

mkdir -p ".nondex"

if [[ $# -lt 1 ]]; then
  echo "Missing command!"
  exit 1
fi

COMMAND=$1
shift

case $COMMAND in
  test)
    x-test "$@"
    ;;
  *)
    echo "Invalid command: ${COMMAND}"
    exit
    ;;
esac
