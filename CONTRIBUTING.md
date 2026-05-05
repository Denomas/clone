# Contributing to Clone

Thank you for your interest in contributing to Clone.

## Getting Started

### Prerequisites

- Linux host with KVM (`/dev/kvm`)
- Rust 1.87+ (install via [rustup](https://rustup.rs) or [mise](https://mise.jdx.dev))
- For e2e tests: root access, kernel 6.5+, busybox-static

### Setup

```bash
# Install toolchain and dev tools (if using mise)
mise install
mise run setup    # installs cargo-audit, cargo-deny, pre-commit hooks

# Or manually:
cargo install cargo-audit cargo-deny
pip install pre-commit && pre-commit install && pre-commit install --hook-type pre-push
```

### Local CI (mandatory before every push)

**All local checks MUST pass before pushing.** Do not rely on remote CI
to catch issues -- run them locally first. This is enforced by pre-commit
hooks if you ran `mise run setup`.

```bash
# REQUIRED before every push:
make ci           # fmt + clippy + test + deny + audit

# Individual checks (for faster iteration during development):
make lint         # fmt + clippy only (~5s)
make test         # unit tests (~1s)
make deny         # license + advisory + dependency checks
make audit        # known vulnerability check
```

| Check | When | What it catches |
|-------|------|-----------------|
| `cargo fmt --check` | Every commit (pre-commit hook) | Formatting issues |
| `cargo clippy -D warnings` | Every commit (pre-commit hook) | Lint warnings, code smells |
| `cargo test` | Every push (pre-push hook) | Regressions, broken logic |
| `cargo deny check` | Every push (pre-push hook) | License violations, vulnerable deps |
| `cargo audit` | Before PR | Known security advisories |

If a pre-commit hook fails, the commit/push is rejected. Fix the issue
and retry. Do not bypass hooks with `--no-verify`.

## Development Workflow

1. Fork the repository
2. Create a feature branch from `master`
3. Make your changes
4. Run `make ci` to verify all checks pass locally
5. Commit with [Conventional Commits](https://www.conventionalcommits.org/) format
6. Open a Pull Request against `master`

### Commit Message Format

```
type(scope): description

feat:     new feature
fix:      bug fix
perf:     performance improvement
refactor: code change that neither fixes a bug nor adds a feature
docs:     documentation only
test:     adding or correcting tests
ci:       CI/CD changes
chore:    maintenance
```

### Code Style

- `rustfmt.toml` enforces `max_width = 120`
- All public APIs should have doc comments
- Prefer safe Rust; `unsafe` blocks require a `// SAFETY:` comment

## Architecture

See [README.md](README.md) for the module layout. Key directories:

- `src/vmm/` -- VM lifecycle, vCPU threads
- `src/virtio/` -- Virtio device implementations
- `src/boot/` -- Kernel loading, ACPI tables
- `src/memory/` -- Guest memory management
- `crates/guest-agent/` -- In-guest vsock agent
- `crates/clone-init/` -- Minimal init for generated initrd

## Testing

- **Unit tests**: `cargo test` (runs on any platform)
- **E2E tests**: `sudo make e2e` (requires Linux + KVM + root)
- See `tests/e2e/run_all.sh` for the full e2e suite
