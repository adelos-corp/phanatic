# ADR-003: Build custom AOSP images on Linux

- Status: Accepted
- Date: 2026-10-14

## Decision

Desktop development occurs on macOS, while custom modern AOSP platform images are built on a supported Linux host.

## Rationale

Modern AOSP builds require a supported Linux build environment; a Mac desktop development setup is not treated as a substitute for that platform build host.

## Consequences

- A custom AOSP build is not required for the first runtime proof.
- Image build instructions and pinned manifests will be documented separately.
- Do not check large platform images or proprietary packages into this repository.
