pub mod kind;
pub mod status;

use chrono::{DateTime, Utc};

use uuid::Uuid;

use crate::{
    journal::{kind::JournalKind, status::JournalStatus},
    ledger::id::LedgerId,
    types::VarChar,
};

#[derive(Debug, Clone, Copy)]
pub struct JournalId(Uuid);

#[derive(Debug, Clone)]
pub struct Journal {
    id: Uuid,
    name: VarChar<100>,
    description: String,
    status: JournalStatus,
    kind: JournalKind,
    ledger_id: LedgerId,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
