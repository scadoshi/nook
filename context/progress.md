# nook progress

Where the project actually is: phase status, decisions made, gotchas surfaced, what's next. Phases are defined in `plan.md`.

## Where I am

**P0 working, as of 2026-11-25.** A per-account ledger (`HashMap<String, Vec<i32>>`, balance summed on every read) behind a REPL. `Command` parses with `FromStr` into typed errors (`InvalidCommand`, `InvalidPost`, `InvalidBalance`). `Repl<R: BufRead>` takes its reader, so tests can feed it input; `main` hands it stdin. Commands: post, balance, help, clear, exit. The help text is built from `Command::help_example` and `help_message`, aligned in two columns.

**Next:**
- Make `Repl` generic over the writer too (`W: Write`), so output is testable the way input already is, and `main` passes stdout. Right now reading is injected but printing goes straight to `println!`, which is asymmetric.
- Write the leftover tests, including REPL-level ones through the reader/writer harness.
- Fix the bugs below.
- Then P0's real question: notice what's wrong (money from nowhere, nothing says where it went) and move to P1.

## Bugs

Found behavior only. The fixes are his to write.

- **End of input loops forever.** Pipe a few commands in, or press Ctrl-D: once `read_line` returns 0, `get_command` returns `Disconnected`, `run` prints it and loops, and it prints `disconnected` forever. Hint: which `ReplError`s should be shown to the user, and which should end `run`? An I/O error from `read_line` takes the same path.
- **Commands are case sensitive now.** `POST bob 3` is "unrecognized command". The first REPL lowercased its input; the `FromStr` one doesn't. Decide on purpose: commands case-insensitive, but account names?
- **A balance can overflow.** `post a 2147483647` twice, then `balance a`: `sum()` over `i32` panics in a debug build and wraps in release. That's the P0 lesson pointing at P2 (checked math, an `Amount` type).
- **Clippy:** `Command::account` and `Command::amount` are only used in tests, so they warn as dead code outside them.

## Decisions

Record each one with the reason, so it doesn't get re-litigated.

## Gotchas

## Status by phase

- [ ] P0 A list of amounts and a sum (working REPL; tests, writer injection and bugs left)
- [ ] P1 Double entry
- [ ] P2 Money and accounts as real types
- [ ] P3 Reversals and idempotency
- [ ] P4 The append-only log
- [ ] P5 Stored balances, versions, reconciliation
- [ ] P6 Pending, settled, available
- [ ] P7 Journals, codes, and time
- [ ] P8 The hash chain
- [ ] P9 Concurrent writers and a network face
