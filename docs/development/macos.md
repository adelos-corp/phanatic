# macOS development setup

## 1. Apple tools

Install Xcode Command Line Tools:

```sh
xcode-select --install
xcodebuild -version
```

Install Homebrew using its official instructions at https://brew.sh/ if it is not already installed.

## 2. Rust and Cargo

Install the official Rust toolchain through https://rustup.rs/. Cargo is bundled with the Rust toolchain.

Then open a new terminal and run:

```sh
rustup default stable
rustup target add aarch64-apple-darwin
rustc --version
cargo --version
cargo test --workspace
cargo run -p phanatic-core
```

If `rustup` is installed but the commands are not found, ensure `$HOME/.cargo/bin` is on PATH and restart the terminal.

## 3. Android tools

Install Android Studio or the Android SDK command-line tools. Install Android SDK Platform Tools (ADB), Android Emulator, and a compatible ARM64 image through SDK Manager.

Before integrating the runtime, record:

- macOS version and Mac chip.
- Emulator version and exact system image.
- Whether hardware acceleration and GPU rendering are enabled.
- ADB connectivity.
- Test APK install/launch result.
- Audio behavior.
- Startup time and idle CPU/memory.

## 4. Desktop frontend (introduced in a later step)

Install Node.js LTS and pnpm. Tauri's official prerequisites and installation instructions should be checked when the desktop project is generated: https://v2.tauri.app/start/prerequisites/.

Do not install unverified third-party scripts or paste credentials into shell commands.
