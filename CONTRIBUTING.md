# Contributing

RustUse/use-web is intentionally small and composable. Favor correctness, clear naming, and narrow APIs over broad feature count.

For routing and organization-wide policy, use the RustUse defaults for
[support](https://github.com/RustUse/.github/blob/main/SUPPORT.md),
[security](https://github.com/RustUse/.github/blob/main/SECURITY.md), and the
[code of conduct](https://github.com/RustUse/.github/blob/main/CODE_OF_CONDUCT.md),
alongside `GOVERNANCE.md` and `MAINTAINERS.md` in this repository.

## Development Flow

1. Make the smallest useful change that improves one crate or workflow.
2. Add or update tests for every public function or type you change.
3. Keep dependencies lightweight unless the workspace would clearly be worse without them.
4. Preserve the explicit, lightweight API direction for web, HTTP, and markup primitives.

## Local Validation

```sh
cargo fmt --all -- --check
cargo check --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo deny check
cargo audit
cargo test --workspace --all-features
cargo test --workspace --no-default-features
cargo doc --workspace --all-features --no-deps
```

## Documentation

- Update the root README when the crate list or facade story changes.
- Keep crate README examples small and runnable.
- Keep docs aligned with the current workspace surface.
- Keep the `use-web` and `rustuse` facades aligned when the exported surface changes.

## Release Policy

- The workspace-level default keeps `publish = false`, while the current first-wave crate manifests opt in with `publish = true`.
- The current first-wave publish surface includes every focused crate under `crates/` plus `use-web`.
- Versions move in lockstep at `0.x.y` for now.
- Until `1.0`, breaking API changes should bump the minor version and compatible additive changes should bump the patch version.
- `Cargo.lock` is committed intentionally for reproducible CI, security checks, and release dry runs in this library workspace.
