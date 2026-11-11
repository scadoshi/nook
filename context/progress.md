# nook progress

Where the project actually is: phase status, decisions made, gotchas surfaced, what's next. Phases are defined in `plan.md`.

## Where I am

**Before P0, starting fresh.** The first two commits typed out the Medium guide's destination schema (Ledger, Journal, Account, Entry, Transaction, TransactionCode, CurrencyKind, Direction, a `VarChar<N>` length-checked string). It was deleted on 2026-11-11 to rebuild phase by phase, and it's still in history at `ecc42c2`. `src/lib.rs` is empty. `Cargo.toml` still lists chrono, thiserror and uuid, which P0 doesn't need.

## Decisions

Record each one with the reason, so it doesn't get re-litigated.

## Gotchas

## Status by phase

- [ ] P0 A list of amounts and a sum
- [ ] P1 Double entry
- [ ] P2 Money and accounts as real types
- [ ] P3 Reversals and idempotency
- [ ] P4 The append-only log
- [ ] P5 Stored balances, versions, reconciliation
- [ ] P6 Pending, settled, available
- [ ] P7 Journals, codes, and time
- [ ] P8 The hash chain
- [ ] P9 Concurrent writers and a network face
