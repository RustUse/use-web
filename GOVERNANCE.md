# Governance

RustUse/use-web follows the broader RustUse governance model while keeping repository-level decisions close to the maintainers responsible for this workspace.

## Scope

This repository exists to host small, focused Rust crates for web, HTTP, browser-adjacent, and API primitives.

Governance decisions should preserve that scope:

- lightweight utilities over framework behavior
- composable string and value helpers over large abstractions
- stable, predictable APIs over speculative surface growth

## Maintainer decisions

Maintainers make day-to-day decisions about API direction, release readiness, dependency policy, and contributor workflow.

Changes that widen the project scope significantly should be discussed in the open before they are merged.

## Release authority

Release authority belongs to the canonical RustUse GitHub repository for `use-web`.

crates.io publishing, release notes, changelog management, and release automation are coordinated from that canonical repository.
