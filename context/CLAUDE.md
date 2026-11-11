# nook project context

## What this is

A double-entry ledger in Rust, built by hand from the dumbest version that could work up to something shaped like the Medium guide's schema: ledgers, journals, accounts, transactions, entries, transaction codes, pending and settled balances.

The point is the muscle: modeling money correctly, keeping invariants that can't be broken, append-only history, and the concurrency a real ledger has to survive. Shipping a product isn't the point, so shortcuts that skip the muscle defeat the project.

## Sources

- Oskar Dudycz, [Building your own Ledger Database](https://www.architecture-weekly.com/p/building-your-own-ledger-database) (Architecture Weekly). The why: append-only, immutable, verifiable, corrections as new records, hash-linked entries for tamper evidence. Also why a ledger isn't an event store: it records outcomes, it doesn't drive decisions.
- Mahammad Khalilov, [How to build a custom ledger system](https://medium.com/@mahammadkhalilov/how-to-build-a-custom-ledger-system-a-comprehensive-guide-838c73ca512d) (Medium). The destination schema. The first two commits typed it out (Ledger, Journal, Account, Entry, Transaction, TransactionCode). It was deleted to start fresh and is still in history at `ecc42c2`.

## Trajectory

Phased, dumbest first (full detail in `context/plan.md`):

- **P0** A list of amounts and a sum
- **P1** Double entry: a transaction balances or doesn't happen
- **P2** Money and accounts as real types
- **P3** Nothing is ever edited: reversals and idempotency
- **P4** On disk: the append-only log
- **P5** Stored balances, versions, and reconciliation
- **P6** Pending, settled, available
- **P7** Journals, codes, and time
- **P8** Tamper evidence: the hash chain
- **P9 (stretch)** Concurrent writers and a network face

Where it actually stands: `context/progress.md`.

## Sibling repos

`steller` (Redis-compatible server) and `chickadee` (LSM storage engine) are the other hand-written systems projects, and their discipline rules carry over. chickadee already did length-prefixed, checksummed log framing and crash recovery. So P4 is rehearsed muscle, and the ledger rules on top of it are what's new.

## Discipline rules

See `context/rules.md`. The **AI collaboration mode** section at the top is mandatory reading for any AI assistant.

The short version:

- Write by hand: domain types, the posting path, every invariant check, balance math, the log, the first version of every test.
- AI lane: boilerplate (Cargo.toml deps, lint config, test scaffolding after the first test), explanations, debugging help *after* you've read the compiler error yourself.
- AI mode: guide educationally. No straight answers, no code in `.rs` files unless the user explicitly asks. Lead with questions. Affirm correctness when it's right rather than pushing toward churn.
- Read compiler errors first. Always.
- Commit small. One feature, one commit.
