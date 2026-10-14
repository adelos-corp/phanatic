# ADR-001: Desktop application stack

- Status: Accepted for prototype
- Date: 2026-10-14

## Decision

Use Tauri 2, React, and TypeScript for the desktop shell, with Rust for native orchestration.

## Rationale

- A web-based UI enables fast visual iteration.
- Rust can manage processes and expose a typed application API.
- Keeping the UI separate from runtime control supports a future Windows port.
- The emulator runtime remains an independent component.

## Consequences

- Requires Node.js/pnpm and the Rust toolchain.
- The frontend cannot be treated as a security boundary.
- Runtime commands must be validated and executed by native code.
- This decision does not imply that Tauri or WebView can provide GPU virtualization.
