#!/usr/bin/env bash
# Usage: bash scripts/smoke_test_local.sh
#
# Smoke-tests the SAFE-HAVEN contract against a local Soroban
# standalone node (stellar network start local).
#
# Prerequisites:
#   - stellar CLI installed (https://developers.stellar.org/docs/tools/developer-tools/cli/install-cli)
#   - Contract WASM built: make build
#   - jq installed (apt-get install jq / brew install jq)
#
# The script:
#   1. Starts a local Soroban node
#   2. Generates a funded test identity
#   3. Deploys the contract
#   4. Calls initialize, deposit, get_vault, time_remaining, withdraw, depositor count
#   5. Asserts expected outputs using jq and string comparison
#   6. Stops the local node

set -euo pipefail

WASM="target/wasm32-unknown-unknown/release/safe_haven.wasm"
NETWORK="local"
IDENTITY="smoke-test-user"

# ── helpers ──────────────────────────────────────────────────────────────────

pass() { echo "  ✓ $*"; }
fail() { echo "  ✗ $*" >&2; exit 1; }

assert_contains() {
    local label="$1" expected="$2" actual="$3"
    if echo "$actual" | grep -qF "$expected"; then
        pass "$label"
    else
        fail "$label — expected to contain '$expected', got: $actual"
    fi
}

# Assert that a numeric value matches expected
assert_eq() {
    local label="$1" expected="$2" actual="$3"
    if [ "$actual" = "$expected" ]; then
        pass "$label ($expected)"
    else
        fail "$label — expected '$expected', got '$actual'"
    fi
}

# Assert that a numeric value is greater than a threshold
assert_gt() {
    local label="$1" threshold="$2" actual="$3"
    if [ "$actual" -gt "$threshold" ] 2>/dev/null; then
        pass "$label ($actual)"
    else
        fail "$label — expected > $threshold, got: $actual"
    fi
}

# ── 1. Build check ────────────────────────────────────────────────────────────

echo "==> Checking WASM..."
[ -f "$WASM" ] || { echo "WASM not found. Run 'make build' first."; exit 1; }
pass "WASM found: $WASM"

# ── 2. Start local node ───────────────────────────────────────────────────────

echo "==> Starting local Soroban node..."
stellar network start "$NETWORK" --background 2>/dev/null || true
sleep 2
pass "Local node started"

cleanup() {
    echo "==> Stopping local node..."
    stellar network stop "$NETWORK" 2>/dev/null || true
}
trap cleanup EXIT

# ── 3. Identity & funding ─────────────────────────────────────────────────────

echo "==> Setting up identity..."
stellar keys generate "$IDENTITY" --network "$NETWORK" --fund 2>/dev/null || true
ADMIN_ADDR=$(stellar keys address "$IDENTITY")
pass "Identity: $ADMIN_ADDR"

# ── 4. Deploy ─────────────────────────────────────────────────────────────────

echo "==> Deploying contract..."
CONTRACT_ID=$(stellar contract deploy \
    --wasm "$WASM" \
    --source "$IDENTITY" \
    --network "$NETWORK")
pass "Contract deployed: $CONTRACT_ID"

# ── 5. Initialize ─────────────────────────────────────────────────────────────

echo "==> Calling initialize..."
stellar contract invoke \
    --id "$CONTRACT_ID" \
    --source "$IDENTITY" \
    --network "$NETWORK" \
    -- initialize \
    --admin "$ADMIN_ADDR" > /dev/null
pass "initialize OK"

# ── 5b. Verify depositor count starts at 0 ────────────────────────────────────

echo "==> Verifying initial depositor count..."
DEPOSITOR_COUNT=$(stellar contract invoke \
    --id "$CONTRACT_ID" \
    --source "$IDENTITY" \
    --network "$NETWORK" \
    -- get_depositor_count)
assert_eq "depositor_count == 0" "0" "$DEPOSITOR_COUNT"

# ── 6. Wrap native XLM as a token ────────────────────────────────────────────

echo "==> Wrapping native XLM..."
TOKEN_ID=$(stellar contract asset deploy \
    --asset native \
    --source "$IDENTITY" \
    --network "$NETWORK")
pass "Token: $TOKEN_ID"

# ── 7. Deposit ────────────────────────────────────────────────────────────────

echo "==> Calling deposit..."
# unlock_time = now + 120 seconds
UNLOCK_TIME=$(( $(date +%s) + 120 ))
stellar contract invoke \
    --id "$CONTRACT_ID" \
    --source "$IDENTITY" \
    --network "$NETWORK" \
    -- deposit \
    --depositor "$ADMIN_ADDR" \
    --token "$TOKEN_ID" \
    --amount 1000 \
    --unlock_time "$UNLOCK_TIME" > /dev/null
pass "deposit OK"

# ── 7b. Verify depositor count incremented ────────────────────────────────────

echo "==> Verifying depositor count after deposit..."
DEPOSITOR_COUNT=$(stellar contract invoke \
    --id "$CONTRACT_ID" \
    --source "$IDENTITY" \
    --network "$NETWORK" \
    -- get_depositor_count)
assert_eq "depositor_count == 1" "1" "$DEPOSITOR_COUNT"

# ── 8. get_vault ──────────────────────────────────────────────────────────────

echo "==> Calling get_vault..."
VAULT_OUT=$(stellar contract invoke \
    --id "$CONTRACT_ID" \
    --source "$IDENTITY" \
    --network "$NETWORK" \
    -- get_vault \
    --depositor "$ADMIN_ADDR")

# Parse the JSON output to assert individual fields
VAULT_AMOUNT=$(echo "$VAULT_OUT" | jq -r '.amount // empty')
VAULT_UNLOCK=$(echo "$VAULT_OUT" | jq -r '.unlock_time // empty')
VAULT_PENALTY=$(echo "$VAULT_OUT" | jq -r '.penalty_bps // empty')

assert_eq "vault.amount == 1000" "1000" "$VAULT_AMOUNT"
assert_eq "vault.unlock_time == $UNLOCK_TIME" "$UNLOCK_TIME" "$VAULT_UNLOCK"
assert_eq "vault.penalty_bps == 0" "0" "$VAULT_PENALTY"
pass "get_vault returns expected values"

# ── 9. time_remaining ────────────────────────────────────────────────────────

echo "==> Calling time_remaining..."
TIME_OUT=$(stellar contract invoke \
    --id "$CONTRACT_ID" \
    --source "$IDENTITY" \
    --network "$NETWORK" \
    -- time_remaining \
    --depositor "$ADMIN_ADDR")
# Should be > 0 since we just deposited with a 120s lock
assert_gt "time_remaining > 0" "0" "$TIME_OUT"

# ── 9b. Verify time_remaining is approximately <= 120 ─────────────────────────

echo "==> Verifying time_remaining ≤ 120..."
if [ "$TIME_OUT" -le 120 ] 2>/dev/null; then
    pass "time_remaining <= 120 ($TIME_OUT)"
else
    fail "time_remaining should be <= 120, got: $TIME_OUT"
fi

# ── 10. withdraw (should fail — still locked) ─────────────────────────────────

echo "==> Calling withdraw (expect FundsStillLocked)..."
WITHDRAW_ERR=$(stellar contract invoke \
    --id "$CONTRACT_ID" \
    --source "$IDENTITY" \
    --network "$NETWORK" \
    -- withdraw \
    --depositor "$ADMIN_ADDR" 2>&1 || true)
assert_contains "withdraw fails while locked" "FundsStillLocked" "$WITHDRAW_ERR"

# ── 10b. Verify vault still exists (was NOT removed by failed withdraw) ───────

echo "==> Verifying vault still exists after failed withdraw..."
VAULT_CHECK=$(stellar contract invoke \
    --id "$CONTRACT_ID" \
    --source "$IDENTITY" \
    --network "$NETWORK" \
    -- get_vault \
    --depositor "$ADMIN_ADDR")
VAULT_CHECK_AMOUNT=$(echo "$VAULT_CHECK" | jq -r '.amount // empty')
assert_eq "vault still has amount 1000" "1000" "$VAULT_CHECK_AMOUNT"

# ── 10c. Verify depositor count unchanged after failed withdraw ───────────────

echo "==> Verifying depositor count unchanged after failed withdraw..."
DEPOSITOR_COUNT=$(stellar contract invoke \
    --id "$CONTRACT_ID" \
    --source "$IDENTITY" \
    --network "$NETWORK" \
    -- get_depositor_count)
assert_eq "depositor_count still == 1" "1" "$DEPOSITOR_COUNT"

# ── Done ──────────────────────────────────────────────────────────────────────

echo ""
echo "All smoke tests passed."
