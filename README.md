# canbus-binding-rs-starter

This project is meant as a minimal example of how to:

- generate Rust code from a DBC file using `dbcparser-cli`,
- plug that generated API into the `canbus-core-binding-rs` CAN stack,
- expose the result as an AFB v4 binding you can run with `afb-binder`,
- replay a CAN trace to exercise the API.

In a real project you will typically use the `DbcParser` API (see `canbus-core-binding-rs` project) directly in your own binding or service. This repository focuses on the “starter” example to get you running quickly.

## repository layout

- `Cargo.toml`  
  Rust crate definition for the `can-starter` binding (`afb_starter` cdylib).

- `src/starter.rs`  
  Main binding implementation. It will call into the generated DBC API.

- `src/__starter.rs` (generated)  
  Generated Rust module created from `starter.dbc` by `dbcparser-cli`.  
  This file is **not** shipped in the repository; you generate it locally.

- `binding-config/binding-config.json`  
  AFB binder configuration loading:
  - `libafb_sockcan.so` (core CAN/BCM binding from `canbus-core-binding-rs`),
  - `libafb_starter.so` (this starter binding).

## prerequisites

You will need:

- A recent stable **Rust** toolchain and **Cargo**.
- The **AFB v4** binder (`afb-binder`) available in your `$PATH`.
- A **Linux** system with **SocketCAN** support.
- The **can-utils** package (`canplayer`, etc.).
- The `canbus-core-binding-rs` and related crates available at the paths expected in `Cargo.toml`, or the full `canforge-rs` workspace checked out (recommended).

## quick start using `dbcparser-cli`

For a quick start you can use `dbcparser-cli` from the command line to generate the Rust API from a DBC file. In production, you typically integrate directly with the `DbcParser` API from `canbus-core-binding-rs`.

### 1. clone the workspace

Clone the workspace that contains both the core CAN bindings and this starter example:

```bash
git clone https://github.com/redpesk-common/canforge-rs.git
cd canforge-rs
```

### 2. install `dbcparser-cli`

From the workspace root (where `dbcparser-cli` is available as a path dependency):

```bash
cargo install --path dbcparser-cli
```

If this is your first `cargo install`, Cargo will typically install binaries into `$HOME/.cargo/bin`.

Make sure this directory is in your `PATH`:

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> "$HOME/.bashrc"
source "$HOME/.bashrc"
```

Now you should be able to run:

```bash
dbcparser-cli --help
```

### 3. generate the DBC-based Rust API

Go to the `canbus-binding-rs-starter` directory (or wherever your starter binding lives). Then run:

```bash
dbcparser-cli --uid Starter --in ./starter.dbc --out ./src/__starter.rs --whitelist "280,599,614"
```

Where:

- `--uid Starter` is the logical name for this DBC API,
- `--in ./starter.dbc` points to your DBC file,
- `--out ./src/__starter.rs` is the Rust file that will be generated and used by `src/starter.rs`.

After this step, `src/__starter.rs` should exist and be included by the binding code.

### 4. build the bindings

It is convenient to keep all build artefacts under the current workspace:

```bash
export CARGO_TARGET_DIR="$(pwd)/target"
```

Then build the bindings (from the workspace root or any compatible Cargo workspace setup):

```bash
cargo build
```

This should produce:

- `libafb_sockcan.so` (from `canbus-core-binding-rs`),
- `libafb_starter.so` (from this starter binding),

under `${CARGO_TARGET_DIR}/debug` (or a similar path depending on your configuration).

### 5. run the AFB binder

From the `canbus-binding-rs-starter` directory (or any directory where the config path is valid), run:

export your CARGO_TARGET_DIR value e.g.:

```bash
export CARGO_TARGET_DIR=$(pwd)/target

afb-binder --traceevt=all \
           --tracereq=all \
           --traceapi=all \
           --config=./binding-config/binding-config.json \
           -vvv
```

This:

- starts the AFB binder on port `1234`,
- loads the `sockbmc` CAN core binding,
- loads the `bms` (starter) binding that exposes your DBC signals.

You can now connect AFB devtools (or any AFB client) to this binder and inspect the APIs and events.

### 6. replay the CAN trace

Assuming you have a CAN log file (for example `starter.log`) and a configured CAN interface (physical or `vcan`), you can replay the traffic with:

```bash
canplayer -v -l i -g 10 -I ./starter.log
```

Typical setup for a virtual CAN interface might look like:

```bash
sudo modprobe vcan
sudo ip link add dev vcan0 type vcan
sudo ip link set up vcan0
```

Then adjust the `canplayer` command line to match your CAN interface names if needed.

As frames are replayed, the `sockbmc` binding will receive them via SocketCAN, and the starter binding will decode them using the generated DBC API and expose them through AFB.

---

## using `DbcParser` directly

The `dbcparser-cli` approach is convenient for development and experimentation. In production projects you will usually:

- depend directly on the `dbcparser` library (from the `canforge-rs` workspace),
- call `DbcParser` from your `build.rs` or a dedicated code generation step,
- generate your DBC-based Rust API as part of the normal Cargo build.

This starter example is intentionally simple and CLI-driven so you can focus on:

- understanding the wiring between AFB v4 and SocketCAN,
- seeing how DBC-generated Rust APIs are used in a real binding,
- having a minimal but concrete project to copy and adapt.

---

## license

The `canbus-binding-rs-starter` crate is distributed under the terms of the **GNU General Public License v3.0 only** (GPL-3.0-only).  
See the `LICENSE` terms in the parent project or the source headers for details.
