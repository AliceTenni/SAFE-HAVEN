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

pub fn token_proposed(env: &Env, token: &Address, proposer: &Address) {
    let topics = (Symbol::new(env, "token_proposed"), token.clone());
    env.events().publish(topics, proposer.clone());
}

pub fn token_reviewed(env: &Env, token: &Address, reviewer: &Address, passed: bool) {
    let topics = (Symbol::new(env, "token_reviewed"), token.clone());
    env.events().publish(topics, (reviewer.clone(), passed));
}

pub fn token_approved(env: &Env, token: &Address, approver: &Address) {
    let topics = (Symbol::new(env, "token_approved"), token.clone());
    env.events().publish(topics, approver.clone());
}

pub fn governance_proposed(env: &Env, proposal_id: u32, proposer: &Address) {
    let topics = (Symbol::new(env, "gov_proposed"), proposal_id);
    env.events().publish(topics, proposer.clone());
}

pub fn governance_voted(env: &Env, proposal_id: u32, voter: &Address, support: bool, weight: i128) {
    let topics = (Symbol::new(env, "gov_voted"), proposal_id, voter.clone());
    env.events().publish(topics, (support, weight));
}

pub fn governance_executed(env: &Env, proposal_id: u32) {
    let topics = (Symbol::new(env, "gov_executed"), proposal_id);
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
