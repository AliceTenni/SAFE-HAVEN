use soroban_sdk::{symbol_short, Address, Env, Symbol};

// Compile-time assertions to ensure event symbol literals are within Soroban's
// symbol size limit (32 bytes). Soroban SDK v22 uses 32-byte symbols; these
// checks will cause a compile failure if any literal exceeds that.
const _: [(); 32 - "deposit_by_ledger".len()] = [(); 32 - "deposit_by_ledger".len()];
const _: [(); 32 - "emrg_wdraw".len()] = [(); 32 - "emrg_wdraw".len()];
const _: [(); 32 - "adm_xfr_init".len()] = [(); 32 - "adm_xfr_init".len()];
const _: [(); 32 - "adm_xfr_cancel".len()] = [(); 32 - "adm_xfr_cancel".len()];
const _: [(); 32 - "adm_xfr_done".len()] = [(); 32 - "adm_xfr_done".len()];
const _: [(); 32 - "adm_renounce".len()] = [(); 32 - "adm_renounce".len()];
const _: [(); 32 - "lock_extended".len()] = [(); 32 - "lock_extended".len()];
const _: [(); 32 - "dep_cancel".len()] = [(); 32 - "dep_cancel".len()];
const _: [(); 32 - "paused".len()] = [(); 32 - "paused".len()];
const _: [(); 32 - "unpaused".len()] = [(); 32 - "unpaused".len()];
const _: [(); 32 - "withdraw_to".len()] = [(); 32 - "withdraw_to".len()];

pub fn deposit(env: &Env, depositor: &Address, token: &Address, amount: i128, unlock_time: u64) {
    let topics = (symbol_short!("deposit"), depositor.clone(), token.clone());
    env.events().publish(topics, (amount, unlock_time));
}

pub fn deposit_by_ledger(env: &Env, depositor: &Address, token: &Address, amount: i128, unlock_ledger: u32) {
    // Emit a distinct event for ledger-based deposits so indexers and UIs
    // can treat `unlock_ledger` as a u32 ledger sequence rather than a
    // timestamp. Topic: `deposit_by_ledger(depositor, token)` Payload: `(amount, unlock_ledger:u32)`.
    let topics = (Symbol::new(env, "deposit_by_ledger"), depositor.clone(), token.clone());
    env.events().publish(topics, (amount, unlock_ledger));
}

pub fn withdraw(env: &Env, depositor: &Address, token: &Address, amount: i128) {
    let topics = (symbol_short!("withdraw"), depositor.clone(), token.clone());
    env.events().publish(topics, amount);
}

pub fn emergency_withdraw(
    env: &Env,
    admin: &Address,
    depositor: &Address,
    token: &Address,
    amount: i128,
) {
    // admin is placed in the data payload rather than topics to avoid
    // leaking the admin address in the publicly-indexed event topic stream.
    let topics = (Symbol::new(env, "emrg_wdraw"), depositor.clone());
    env.events()
        .publish(topics, (admin.clone(), token.clone(), amount));
}

pub fn admin_transfer_initiated(env: &Env, current_admin: &Address, pending_admin: &Address) {
    let topics = (Symbol::new(env, "adm_xfr_init"), current_admin.clone());
    env.events().publish(topics, pending_admin.clone());
}

pub fn admin_transfer_cancelled(env: &Env, current_admin: &Address, pending_admin: &Address) {
    let topics = (Symbol::new(env, "adm_xfr_cancel"), current_admin.clone());
    env.events().publish(topics, pending_admin.clone());
}

pub fn admin_transfer_accepted(env: &Env, new_admin: &Address) {
    let topics = (Symbol::new(env, "adm_xfr_done"), new_admin.clone());
    env.events().publish(topics, ());
}

pub fn admin_renounced(env: &Env, former_admin: &Address) {
    let topics = (Symbol::new(env, "adm_renounce"), former_admin.clone());
    env.events().publish(topics, ());
}

pub fn lock_extended(
    env: &Env,
    depositor: &Address,
    old_unlock_time: u64,
    new_unlock_time: u64,
) {
    let topics = (Symbol::new(env, "lock_extended"), depositor.clone());
    env.events().publish(topics, (old_unlock_time, new_unlock_time));
}

pub fn deposit_cancelled(
    env: &Env,
    depositor: &Address,
    token: &Address,
    amount: i128,
    penalty: i128,
) {
    let topics = (
        Symbol::new(env, "dep_cancel"),
        depositor.clone(),
        token.clone(),
    );
    env.events().publish(topics, (amount, penalty));
}

pub fn paused(env: &Env, admin: &Address) {
    let topics = (Symbol::new(env, "paused"), admin.clone());
    env.events().publish(topics, ());
}

pub fn unpaused(env: &Env, admin: &Address) {
    let topics = (Symbol::new(env, "unpaused"), admin.clone());
    env.events().publish(topics, ());
}

pub fn withdraw_to(
    env: &Env,
    depositor: &Address,
    recipient: &Address,
    token: &Address,
    amount: i128,
) {
    let topics = (Symbol::new(env, "withdraw_to"), depositor.clone(), token.clone());
    env.events().publish(topics, (recipient.clone(), amount));
}
