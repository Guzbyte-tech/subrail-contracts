use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// Contract has not been initialized yet.
    NotInitialized = 1,
    /// Contract has already been initialized.
    AlreadyInitialized = 2,
    /// The referenced plan does not exist.
    PlanNotFound = 3,
    /// The referenced subscription does not exist.
    SubscriptionNotFound = 4,
    /// The plan is not accepting new activity.
    PlanInactive = 5,
    /// Caller is not authorized for this action.
    Unauthorized = 6,
    /// Amounts must be strictly positive.
    InvalidAmount = 7,
    /// Billing interval must be strictly positive.
    InvalidInterval = 8,
    /// The per-period cap is below the plan price.
    CapBelowPlanAmount = 9,
    /// The subscription is not in a state that permits this action.
    InvalidStatus = 10,
    /// The current billing period has not elapsed yet.
    ChargeNotDue = 11,
    /// Withdrawal amount exceeds the subscription's deposited balance.
    InsufficientBalance = 12,
    /// Initial deposit must cover at least the first period.
    DepositBelowFirstCharge = 13,
    /// The plan price exceeds the subscriber's per-period cap.
    CapExceeded = 14,
}
