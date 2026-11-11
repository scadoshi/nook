use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{ledger::id::LedgerId, types::VarChar};

#[derive(Debug, Clone, Copy)]
pub struct TransactionCodeId(Uuid);

#[derive(Debug, Clone)]
pub struct TransactionCode {
    id: Uuid,
    code: VarChar<50>,
    description: String,
    ledger_id: LedgerId,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
