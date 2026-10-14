# ADR-002: Reuse an established Android runtime

- Status: Proposed, pending Apple-silicon proof
- Date: 2026-10-14

## Decision

Evaluate the Android Emulator and its supported macOS ARM64 accelerated configuration as the first runtime backend. Do not implement a custom hypervisor or CPU emulator in the MVP.

## Acceptance gate

Before locking this decision, demonstrate on supported Apple-silicon hardware:

- A compatible ARM64 system image boots reproducibly.
- Graphics acceleration works.
- Audio and ADB work.
- APK install and launch work.
- Start/stop behavior and logs can be automated.
- License and redistribution obligations are documented.

## Consequence

The runtime adapter must report capabilities instead of assuming every backend supports snapshots, metrics, graphics options, or device control.
