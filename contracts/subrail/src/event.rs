//! Typed contract events for every observable state change.
//!
//! Every subscription lifecycle event carries `prev_status`/`new_status`
//! so off-chain indexers can rebuild the exact state machine without
//! re-reading contract storage.

use soroban_sdk::{contractevent, Address, Env};

use crate::types::SubscriptionStatus;

#[contractevent]
#[derive(Clone, Debug)]
pub struct PlanCreated {
    #[topic]
    pub plan_id: u64,
    #[topic]
    pub merchant: Address,
    pub amount: i128,
    pub interval: u64,
}

#[contractevent]
#[derive(Clone, Debug)]
pub struct PlanStatusChanged {
    #[topic]
    pub plan_id: u64,
    pub active: bool,
}

#[contractevent]
#[derive(Clone, Debug)]
pub struct Subscribed {
    #[topic]
    pub sub_id: u64,
    #[topic]
    pub plan_id: u64,
    pub subscriber: Address,
    pub max_amount: i128,
}

#[contractevent]
#[derive(Clone, Debug)]
pub struct Charged {
    #[topic]
    pub sub_id: u64,
    pub amount: i128,
    pub periods_paid: u32,
}

#[contractevent]
#[derive(Clone, Debug)]
pub struct ChargeFailed {
    #[topic]
    pub sub_id: u64,
    pub prev_status: SubscriptionStatus,
    pub new_status: SubscriptionStatus,
    pub failed_attempts: u32,
}

#[contractevent]
#[derive(Clone, Debug)]
pub struct StatusChanged {
    #[topic]
    pub sub_id: u64,
    pub prev_status: SubscriptionStatus,
    pub new_status: SubscriptionStatus,
}

#[contractevent]
#[derive(Clone, Debug)]
pub struct Deposited {
    #[topic]
    pub sub_id: u64,
    pub amount: i128,
    pub balance: i128,
}

#[contractevent]
#[derive(Clone, Debug)]
pub struct Withdrawn {
    #[topic]
    pub sub_id: u64,
    pub amount: i128,
    pub balance: i128,
}

// ── Emission helpers (keep call sites in lib.rs one-liners) ─────────────

pub fn plan_created(env: &Env, plan_id: u64, merchant: &Address, amount: i128, interval: u64) {
    PlanCreated {
        plan_id,
        merchant: merchant.clone(),
        amount,
        interval,
    }
    .publish(env);
}

pub fn plan_status_changed(env: &Env, plan_id: u64, active: bool) {
    PlanStatusChanged { plan_id, active }.publish(env);
}

pub fn subscribed(env: &Env, sub_id: u64, plan_id: u64, subscriber: &Address, max_amount: i128) {
    Subscribed {
        sub_id,
        plan_id,
        subscriber: subscriber.clone(),
        max_amount,
    }
    .publish(env);
}

pub fn charged(env: &Env, sub_id: u64, amount: i128, periods_paid: u32) {
    Charged {
        sub_id,
        amount,
        periods_paid,
    }
    .publish(env);
}

pub fn charge_failed(
    env: &Env,
    sub_id: u64,
    prev_status: SubscriptionStatus,
    new_status: SubscriptionStatus,
    failed_attempts: u32,
) {
    ChargeFailed {
        sub_id,
        prev_status,
        new_status,
        failed_attempts,
    }
    .publish(env);
}

pub fn status_changed(
    env: &Env,
    sub_id: u64,
    prev_status: SubscriptionStatus,
    new_status: SubscriptionStatus,
) {
    StatusChanged {
        sub_id,
        prev_status,
        new_status,
    }
    .publish(env);
}

pub fn deposited(env: &Env, sub_id: u64, amount: i128, balance: i128) {
    Deposited {
        sub_id,
        amount,
        balance,
    }
    .publish(env);
}

pub fn withdrawn(env: &Env, sub_id: u64, amount: i128, balance: i128) {
    Withdrawn {
        sub_id,
        amount,
        balance,
    }
    .publish(env);
}
