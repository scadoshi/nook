pub mod id;
pub mod kind;

use chrono::{DateTime, Utc};

use uuid::Uuid;

use crate::{ledger::kind::LedgerKind, types::variable_character::VarChar};

#[derive(Debug, Clone)]
pub struct Ledger {
    id: Uuid,
    name: VarChar<100>,
    description: String,
    kind: LedgerKind,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
