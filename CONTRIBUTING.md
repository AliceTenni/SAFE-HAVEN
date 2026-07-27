# Contributing

Thank you for your interest in contributing to SAFE-HAVEN!

## Branch Naming

| Type | Pattern | Example |
|---|---|---|
| New feature | `feat/<short-description>` | `feat/multi-token-support` |
| Bug fix | `fix/<short-description>` | `fix/unlock-time-overflow` |
| Chore / tooling | `chore/<short-description>` | `chore/update-dependencies` |
| Docs | `docs/<short-description>` | `docs/contributing-guide` |

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <short summary>

[optional body]
```

Common types: `feat`, `fix`, `docs`, `chore`, `test`, `refactor`, `ci`.

Examples:
```
feat(contract): add multi-depositor support
fix(storage): correct TTL bump on emergency withdraw
docs: add CONTRIBUTING.md
```

## Local Development

```bash
# Full check: fmt + clippy + tests
make check

# Build optimized WASM and report size
make build && make optimize
```

## Performance Conventions

### Cache `env.ledger().timestamp()` in a local variable

Every call to `env.ledger().timestamp()` is a host-function invocation with a non-trivial cost in the Soroban execution environment. Always cache the result in a `let now` binding at the top of any function that reads the ledger timestamp more than once:

```rust
// good
pub fn some_fn(env: Env, depositor: Address) -> Result<(), VaultError> {
    let now = env.ledger().timestamp();
    if now < entry.unlock_time {
        return Err(VaultError::FundsStillLocked);
    }
    // ... use `now` again later without re-invoking the host
}

// bad â€” calls the host twice for the same value
pub fn some_fn(env: Env, depositor: Address) -> Result<(), VaultError> {
    if env.ledger().timestamp() < entry.unlock_time {
        return Err(VaultError::FundsStillLocked);
    }
    let elapsed = env.ledger().timestamp() - start;
}
```

This convention applies to any repeated host accessor (`env.ledger().sequence()`, `env.current_contract_address()`, etc.) â€” read once, store locally, reuse the binding.

## Before Opening a PR

- [ ] `make check` passes locally
- [ ] New tests added for any new behaviour
- [ ] README updated if the public API changed

- [ ] CHANGELOG.md updated under [Unreleased] with a summary of the change

## Required Status Checks

All PRs must pass the following CI jobs before they can be merged:

| Job | Description |
|---|---|
| **security-audit** | Scans dependencies for known vulnerabilities (`cargo audit`) |
| **lint** | Ensures code formatting (`cargo fmt`) and passes Clippy lints |
| **test** | Runs all unit tests and doc tests |
| **deny** | Verifies license compliance and ban policy (`cargo deny`) |
| **build** | Compiles and optimizes the WASM binary for both stable and MSRV toolchains |
| **geiger** | Scans for unsafe Rust code |

> **Note:** These checks are enforced via branch protection rules in the GitHub repository settings.
> Make sure `main` and `develop` branches have these status checks configured as required.
> A PR that fails any of these jobs should not be merged until the failures are resolved.

## Submitting a PR

1. Push your branch and open a PR against `main`.
2. Fill in the PR description with a summary of changes and what was tested.
3. Link any related issue with `Closes #<issue-number>`.

## Test Snapshots

Running `cargo test` may generate a `contracts/safe-haven/test_snapshots/` directory containing XDR snapshots of contract state produced by the Soroban test environment. These are transient build artefacts, not committed regression fixtures, and are listed in `.gitignore`. Do not commit them.
