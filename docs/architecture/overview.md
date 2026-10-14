# Architecture overview

## Design boundary

The desktop UI must never launch arbitrary shell commands. It calls a narrow, typed native API. The core validates requests, selects a runtime adapter, supervises runtime processes, and returns sanitized status and errors.

## Layers

1. **Desktop shell (planned):** Tauri + React + TypeScript.
2. **Core:** device registry, lifecycle orchestration, configuration validation, events.
3. **Runtime API:** backend-neutral device/runtime contracts.
4. **Runtime adapters:** established Android Emulator implementation first; future alternatives behind the same API.
5. **Host integration:** macOS-specific services first, Windows implementations later.
6. **Android guest:** independently managed system image and persistent user data.

## MVP integration path

1. Detect a supported local emulator installation.
2. Record its version and capabilities.
3. Enumerate known virtual-device profiles.
4. Start/stop a selected virtual device through a controlled adapter.
5. Connect through ADB for APK installation, shell, and logs.
6. Measure boot time and resource use.
7. Add custom UI after the runtime workflow has been proven.

## Security boundaries

- Never accept an arbitrary executable path from an untrusted UI request.
- Use argument arrays, not shell-interpolated command strings.
- Apply timeouts and output-size limits to subprocesses.
- Avoid logging APK contents, personal data, environment variables, or credentials.
- Keep Android system images and application data separate from executable binaries.
- Surface that root, snapshots, or other privileged capabilities are unavailable when the selected backend cannot safely support them.
