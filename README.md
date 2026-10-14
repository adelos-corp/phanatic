# PHANATIC

**ADELOS Corp.** — a Mac-first Android environment for gaming, general Android use, and development.

> Status: early engineering scaffold. The repository does not yet contain an integrated Android runtime or a packaged desktop application.

## Project goals

- Provide a polished native-feeling desktop interface for managing Android virtual devices.
- Reuse established Android Emulator / QEMU technology instead of writing a CPU emulator from scratch.
- Prioritize gaming performance, compatibility, and reliable measurement.
- Keep runtime, device management, and desktop UI separated behind typed interfaces.
- Target macOS on Apple silicon first; design platform seams for a future Windows backend.

## Current state

This repository is a clean starting point. The first milestone is a reproducible runtime proof: boot a supported Android ARM64 image with an established emulator on an Apple-silicon Mac, verify graphics/audio/ADB, and then manage that runtime from PHANATIC.

No performance or broad game-compatibility claims are made until they are measured.

## Architecture (planned)

```text
apps/desktop/                  Tauri + React + TypeScript UI (planned)
crates/phanatic-core/          lifecycle and device orchestration
crates/phanatic-runtime-api/   backend-neutral runtime contract
crates/phanatic-runtime-android-emulator/
                               adapter around a verified emulator installation
crates/phanatic-android-bridge/
                               ADB-facing integration helpers
platform/macos/                macOS-specific integrations
platform/windows/              future Windows backend
docs/                          architecture and development decisions
tests/                         integration and compatibility tests
benchmarks/                    repeatable benchmark definitions and results
```

The initial code deliberately starts smaller than the eventual architecture. We will not create placeholder crates for every future capability.

## Development prerequisites

- macOS on Apple silicon for the first target
- Xcode Command Line Tools
- Git
- Rust stable toolchain (includes Cargo)
- Node.js LTS and pnpm once the desktop shell is introduced
- Android SDK Platform Tools (ADB)
- A supported Android Emulator installation and compatible ARM64 system image

Install the Rust toolchain using the official instructions: <https://rustup.rs/>.

Verify the Rust installation:

```sh
rustc --version
cargo --version
rustup show
```

The desktop shell can be developed on macOS. Building custom modern AOSP images requires a supported Linux build host; a full AOSP build is not a prerequisite for the first prototype.

## First local checks

Once Rust is installed:

```sh
cargo test --workspace
cargo run -p phanatic-core
```

The current core is only a scaffold and does not launch Android yet.

## Engineering principles

1. **Measure before optimizing.** Track cold boot, warm start, idle CPU/RAM, frame-time distributions, sustained performance, and compatibility.
2. **No fake compatibility claims.** Treat graphics, Play services, DRM, ARM translation, and anti-cheat as separate test dimensions.
3. **Use a runtime adapter.** The UI must not spawn arbitrary shell commands. Native code validates operations and supervises processes.
4. **Respect licenses.** Verify each component's and system image's redistribution terms before bundling them.
5. **Test on real hardware.** Repository CI can validate code and contracts, but cannot prove performance on an Apple GPU or guarantee a game works.
6. **Keep platform-specific code behind interfaces.** The Windows path must not require a rewrite of the UI or core orchestration.

## License

No project-wide license has been selected yet. Do not assume this repository grants permission to reuse, redistribute, or relicense third-party components. A project license decision is recorded as an early release prerequisite.
