# developer guide: local overrides for git dependencies (canforge-rs and afb-librust)

Repository: `git@git.ovh.iot:redpesk/redpesk-common/canbus-binding-rs-starter.git`

This project consumes Rust crates hosted in external git repositories. For day-to-day development, contributors should be able to work with **local checkouts** of those repositories **without modifying this project’s sources**.

The recommended approach is a **per-developer Cargo patch** in `~/.cargo/config.toml`.

This document covers two upstream repositories used by this project:

- `https://github.com/redpesk-common/canforge-rs.git` (e.g., `dbcparser`)
- `https://github.com/redpesk-common/afb-librust` (crate `afbv4`)
- `https://github.com/redpesk-common/canbus-core-binding-rs.git`

## goals and constraints

- Do not commit local `path = ...` dependencies into this repository.
- Developers can switch between upstream git sources and local checkouts by editing only:
  - `~/.cargo/config.toml`
- Keep the consumer repository buildable for CI using upstream git dependencies.

## prerequisites

- Rust toolchain installed (stable recommended)
- `git`

Optional:

- `cargo-tree` comes with Cargo (`cargo tree` is available on recent toolchains)
- `cargo-edit`

## how Cargo patching works

Cargo supports overriding dependency sources via `[patch]` tables.

When a crate is pulled from a specific git URL, you can replace that crate with a local checkout using:

- a patch key matching the git URL exactly, and
- a `path` entry pointing to the crate directory containing the crate’s `Cargo.toml`.

This happens **locally** and does not require changing any project manifests.

## 1) clone the upstream repositories locally

Pick a place for local development checkouts:

```bash
mkdir -p ~/dev/redpesk-common
cd ~/dev/redpesk-common
```

Clone `canforge-rs`:

```bash
git clone https://github.com/redpesk-common/canforge-rs.git
```

Clone `afb-librust`:
