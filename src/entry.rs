pub mod kind;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    account::AccountId,
    entry::kind::EntryKind,
    journal::JournalId,
    ledger::id::LedgerId,
    transaction::TransactionId,
    types::{currency::CurrencyKind, direction::Direction},
};

#[derive(Debug, Clone)]
pub struct Entry {
    id: Uuid,
    transaction_id: TransactionId,
    account_id: AccountId,
    journal_id: JournalId,
    ledger_id: LedgerId,
    currency: CurrencyKind,
    amount: i64,
    direction: Direction,
    version: i32,
    account_version: i32,
    layer_id: i32,
    sequence: i32,
    kind: EntryKind,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
