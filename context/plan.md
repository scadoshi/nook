# nook plan

Start with the dumbest ledger that could possibly work, then add one structure at a time. Each phase only adds what the phase before it couldn't handle. The full schema from the Medium guide is the destination, not the starting point.

Every phase ends the same way: it works by hand from a REPL, it has tests that try to break it, and it's committed.

## Starting fresh

The first two commits typed out the whole destination schema (Ledger, Journal, Account with six balance fields, Entry with `layer_id` and `sequence`, Transaction with reversal links). It was deleted on 2026-11-11 to start from P0, and it's still in git history (`ecc42c2`) as a reference. Every type and field comes back only when a phase uses it.

Question for yourself, at every phase: could you explain each field to an interviewer, or is it only there because the article had it?

---

## P0: A list of amounts and a sum

The dumbest ledger. A `Vec` of `(account, amount)` pairs. `post <account> <amount>` appends one. `balance <account>` sums them. Nothing is ever removed.

- REPL on stdin, the same shape as chickadee's first phase
- Amounts are signed integers
- Balance is computed every time, never stored

Done when: you can post, read a balance, and you've noticed what's wrong with it. Money can come from nowhere, and nothing says where it went.

Questions: Why is "balance is derived, not stored" already the right call, even here? What would you lose by storing a running total instead?

## P1: Double entry: a transaction balances or doesn't happen

A transaction is a group of entries posted together, and its debits must equal its credits. `transfer alice bob 100` writes two entries: a debit on one account, a credit on the other.

- `Direction { Debit, Credit }` and an unsigned amount, instead of a signed number
- A `Transaction` holds two or more entries and gets validated before anything is written
- All-or-nothing: an unbalanced transaction writes zero entries
- A transfer to the same account is a decision to make on purpose

Done when: no command can create or destroy money, and a test proves an unbalanced transaction leaves the ledger untouched.

Questions: Signed amounts or direction plus unsigned amount? What does each make impossible? Where does the balanced check live so that nothing can skip it?

## P2: Money and accounts as real types

Strings and bare integers turn into types that refuse bad states.

- An `Amount` newtype over minor units (cents), with checked add and subtract. Overflow is an error.
- Currency on every account. An entry's currency must match its account's currency. A transaction balances per currency.
- Accounts get an id (uuid v7) and a kind.
- Normal balance: asset and expense accounts grow with debits, liability, equity and revenue accounts grow with credits. This is what `BalanceKind` really is, and the A/B/C placeholders get their real names here.
- Accounts must exist before anything is posted to them

Done when: an amount can't be negative where it shouldn't be, currencies can't mix, and a balance reads with the right sign for its account's kind.

Questions: Why integers and never floats? (Try `0.1 + 0.2` once.) What's the smallest unit for JPY versus USD, and where does that knowledge live? Should a `CurrencyKind` know its own decimal places?

## P3: Nothing is ever edited: reversals and idempotency

A ledger's history is the product. Mistakes get fixed by new transactions, never by changing old ones.

- No update or delete anywhere in the API. Make it impossible, not just discouraged.
- `reverse <txn_id>` posts a mirror transaction and links the two (`reverses_transaction_id`, `reversed_by_transaction_id`)
- A transaction can be reversed once
- Idempotency key on every post: the same key posted twice returns the first result and writes nothing new

Done when: a reversal brings both balances back, a double reversal is rejected, and a retried post with the same key doesn't double-charge.

Questions: Why is reversal better than delete for an auditor? Where does the idempotency key get checked, before or after validation? What should a retry with the same key but a different amount do?

## P4: On disk: the append-only log

The ledger survives a restart. This is chickadee muscle, so lean on it.

- Every committed transaction is appended to a log file as one record, with all its entries
- Framing: length prefix plus checksum, like chickadee's header
- `sync_all()` before a post reports success
- Startup replays the log to rebuild everything. Balances are still derived.
- A torn final record (a crash mid-write) is dropped. Garbage in the middle is fatal.

Done when: kill the process mid-run, restart, and every acknowledged transaction is there and nothing half-written is.

Questions: Why does a whole transaction have to be one record, not one record per entry? What would a torn transaction look like if it weren't? What's different here from chickadee's log, where an overwrite is normal?

## P5: Stored balances, versions, and reconciliation

Summing every entry for every read gets slow. Store the balance, but keep proving it's right.

- Accounts carry `settled_debit`, `settled_credit` and a `version` that increments on every post
- Entries record the `account_version` they produced, so the history shows the order things happened in
- Optimistic concurrency: a post can say "only if the account is still at version N" and fails if it isn't
- `reconcile` recomputes every balance from the entries and compares it with the stored one. Any mismatch is a bug.

Done when: stored and derived balances agree after every test, and a stale-version post is rejected.

Questions: Why keep debit and credit totals separately instead of one net number? What does the version protect against that a lock wouldn't? When would reconcile ever disagree?

## P6: Pending, settled, available

Real money moves in steps. A card authorization holds funds before it settles. This is where `pending_*` and `available_*` come from.

- Layers: an entry is pending or settled (`layer_id` in the schema)
- `hold`: a pending debit that lowers available but not settled
- `settle`: turns a hold into a settled entry
- `void`: releases a hold with no settlement
- Available balance = settled minus pending debits (decide whether pending credits count)
- Optional: reject a hold that would take available below zero, per account kind

Done when: a hold then settle, and a hold then void, both leave every balance right, and reconcile still agrees.

Questions: Should pending credits raise the available balance? (Banks say no. Why?) Is a settle a new transaction or a change to the hold? Which answer keeps P3's no-edits rule?

## P7: Journals, codes, and time

The organizing structures from the schema, now that there's something to organize.

- Ledger as the top container, journals inside it (open or closed). A closed journal takes no posts.
- Transaction codes: named, reusable kinds of transaction (`DEPOSIT`, `FEE`) instead of free text
- `posted_at` (when it was written) vs `effective_at` (when it counts). A backdated transaction is legal and visible.
- `balance <account> --as-of <time>`: a point-in-time balance from the entries

Done when: an as-of balance before a backdated transaction excludes it, and posting to a closed journal fails.

Questions: Which time does an as-of query use, posted or effective? What does an auditor need to see when they differ?

## P8: Tamper evidence: the hash chain

From the Architecture Weekly article: "each transaction linked to the previous one", so a changed record is detectable.

- Each log record stores the hash of the previous record plus its own content
- `verify` walks the chain and reports the first record that doesn't match
- A test edits a byte in the middle of the log and `verify` catches it

Done when: any single changed byte anywhere in history is caught.

Questions: What's the difference between this and chickadee's per-record CRC? What does a hash chain prove, and what can't it stop?

## P9 (stretch): Concurrent writers and a network face

- A TCP server with a simple line protocol, like chickadee's phase 5
- Many writers: per-account locks, and a fixed lock order so two transfers in opposite directions can't deadlock
- A benchmark of transfers per second, with the honest write-up of where it loses (steller's `BENCHMARKS.md` is the model)
- Maybe a Postgres adapter behind a port, to compare against the hand-rolled log

Questions: alice→bob and bob→alice at the same moment, both holding one lock and waiting on the other. What breaks the cycle? Is one global lock actually that bad at this scale? Measure before deciding.
