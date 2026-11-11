pub mod code;
pub mod kind;

use chrono::{DateTime, Utc};

use uuid::Uuid;

use crate::{
    journal::JournalId,
    ledger::id::LedgerId,
    transaction::{code::TransactionCodeId, kind::TransactionKind},
};

#[derive(Debug, Clone, Copy)]
pub struct TransactionId(Uuid);

#[derive(Debug, Clone)]
pub struct Transaction {
    id: Uuid,
    description: String,
    transaction_date: DateTime<Utc>,
    journal_id: JournalId,
    ledger_id: LedgerId,
    kind: TransactionKind,
    version: i32,
    transaction_code_id: TransactionCodeId,
    layer_id: i32,
    posted_at: DateTime<Utc>,
    effective_at: DateTime<Utc>,
    reversed_by_transaction_id: TransactionId,
    reverses_transaction_id: TransactionId,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
