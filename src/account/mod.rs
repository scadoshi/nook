pub mod balance;
pub mod kind;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    account::{balance::kind::BalanceKind, kind::AccountKind},
    ledger::id::LedgerId,
    types::{VarChar, currency::CurrencyKind},
};

#[derive(Debug, Clone, Copy)]
pub struct AccountId(Uuid);

pub struct Account {
    id: Uuid,
    name: VarChar<50>,
    description: String,
    kind: AccountKind,
    balance_kind: BalanceKind,
    settled_debit: i64,
    settled_credit: i64,
    pending_debit: i64,
    pending_credit: i64,
    pending_balance: i64,
    available_debit: i64,
    available_credit: i64,
    available_balance: i64,
    version: i32,
    ledger_id: LedgerId,
    currency: CurrencyKind,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
