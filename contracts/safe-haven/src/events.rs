use soroban_sdk::{symbol_short, Address, Env, Symbol};

pub fn contract_initialized(
    env: &Env,
    admin: &Address,
    fee_recipient: &Address,
    max_deposit: i128,
    max_lock_secs: u64,
) {
    let topics = (Symbol::new(env, "initialized"),);
    env.events()
        .publish(topics, (admin.clone(), fee_recipient.clone(), max_deposit, max_lock_secs));
}

pub fn deposit(env: &Env, depositor: &Address, token: &Address, amount: i128, unlock_time: u64, deposit_id: u32) {
    let topics = (symbol_short!("deposit"), depositor.clone(), token.clone());
    env.events().publish(topics, (amount, unlock_time, deposit_id));
}

pub fn deposit_by_ledger(env: &Env, depositor: &Address, token: &Address, amount: i128, unlock_ledger: u32, deposit_id: u32) {
    let topics = (Symbol::new(env, "dep_by_ledger"), depositor.clone(), token.clone());
    env.events().publish(topics, (amount, unlock_ledger, deposit_id));
}

pub fn withdraw(env: &Env, depositor: &Address, token: &Address, amount: i128, deposit_id: u32) {
    let topics = (symbol_short!("withdraw"), depositor.clone(), token.clone());
    env.events().publish(topics, (amount, deposit_id));
}

pub fn emergency_withdraw(
    env: &Env,
    admin: &Address,
    depositor: &Address,
    token: &Address,
    amount: i128,
    deposit_id: u32,
) {
    // admin is placed in the data payload rather than topics to avoid
    // leaking the admin address in the publicly-indexed event topic stream.
    let topics = (Symbol::new(env, "emrg_wdraw"), depositor.clone());
    env.events()
        .publish(topics, (admin.clone(), token.clone(), amount, deposit_id));
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
    deposit_id: u32,
) {
    let topics = (
        Symbol::new(env, "dep_cancel"),
        depositor.clone(),
        token.clone(),
    );
    env.events().publish(topics, (amount, penalty, deposit_id));
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

pub fn upgrade_proposed(
    env: &Env,
    admin: &Address,
    new_contract_id: &Address,
    execute_after: u64,
) {
    let topics = (Symbol::new(env, "upgrade_prop"), admin.clone());
    env.events()
        .publish(topics, (new_contract_id.clone(), execute_after));
}

pub fn upgrade_executed(
    env: &Env,
    admin: &Address,
    new_contract_id: &Address,
    old_version: &soroban_sdk::String,
    new_version: &soroban_sdk::String,
) {
    let topics = (Symbol::new(env, "upgrade_exec"), admin.clone());
    env.events().publish(
        topics,
        (new_contract_id.clone(), old_version.clone(), new_version.clone()),
    );
}

pub fn upgrade_rolled_back(
    env: &Env,
    admin: &Address,
    rolled_back_version: &soroban_sdk::String,
    restored_version: &soroban_sdk::String,
) {
    let topics = (Symbol::new(env, "upgrade_rollback"), admin.clone());
    env.events().publish(
        topics,
        (rolled_back_version.clone(), restored_version.clone()),
    );
}
