# nook progress

Where the project actually is: phase status, decisions made, gotchas surfaced, what's next. Phases are defined in `plan.md`.

## Where I am

**P0 working, as of 2026-11-25.** A per-account ledger (`HashMap<String, Vec<i32>>`, balance summed on every read) behind a REPL. Accounts must be created before use: `create` adds one (64-char name limit, no duplicates), and `post` and `balance` return `AccountNotFound` on a missing one (`CreateError`, `PostError`, `BalanceError` in `ledger.rs`). `post` refuses an amount that would overflow the balance. `Command` parses with `FromStr`, case-insensitive, into typed errors (`InvalidCommand`, `InvalidPost`, `InvalidBalance`). `Repl<R: BufRead>` takes its reader; end of input means exit. Commands: help, create, post, balance, exit (clear was removed). Help is aligned in two columns from `help_example` and `help_message`.

As of 2026-11-25 late: 6 commits pushed (disconnect handling through the bug-board fixes); create and the not-found errors are uncommitted on the laptop. 15 of 16 tests pass, clippy is clean.

**Next:**
- Make `Repl` generic over the writer too (`W: Write`), so output is testable the way input already is, and `main` passes stdout. Right now reading is injected but printing goes straight to `println!`, which is asymmetric.
- Write the leftover tests, including REPL-level ones through the reader/writer harness.
- Fix the bugs below.
- Then P0's real question: notice what's wrong (money from nowhere, nothing says where it went) and move to P1.

## Bugs

Found behavior only. The fixes are his to write.

- [x] **End of input loops forever.** Pipe a few commands in, or press Ctrl-D: once `read_line` returns 0, `get_command` returns `Disconnected`, `run` prints it and loops, and it prints `disconnected` forever. Hint: which `ReplError`s should be shown to the user, and which should end `run`? An I/O error from `read_line` takes the same path.
    - Removed disconnected as an error since it isn't. Just made read output of Ok(0) to return Command::Exit as well
- [x] **Commands are case sensitive now.** `POST bob 3` is "unrecognized command". The first REPL lowercased its input; the `FromStr` one doesn't. Decide on purpose: commands case-insensitive, but account names?
    - Made to be case insensitive
- [x] **A balance can overflow.** `post a 2147483647` twice, then `balance a`: `sum()` over `i32` panics in a debug build and wraps in release. That's the P0 lesson pointing at P2 (checked math, an `Amount` type).
    - Build a post to decline when the addition would overflow
- [x] **Clippy:** `Command::account` and `Command::amount` are only used in tests, so they warn as dead code outside them.
    - Ignoring them with a config tag for; now we'll use later
- [x] **Nit, naming:** `InvalidPost::MissingAccount` and `InvalidBalance::Account` are the same failure under two names. Pick one and use it in both.
    - Consolidated the errors into missing args

Open, found 2026-11-25 after create and not-found landed:

- [ ] **`create_trims_account_names` fails.** `create("  foo  ")` then `balance("foo")` is `AccountNotFound`. `create` trims only for the length check; what gets stored, and what `balance` and `post` look up, is the untrimmed string. Hint: where should a name become its canonical form, once at the edge or in every method? Note the REPL can't hit this (`split_whitespace` already strips), so it's a `Ledger` API question.
- [ ] **Blank and whitespace-only names.** `Ledger::create("")` and `create("   ")` succeed through the API. Same "where does validation live" question as above.
- [ ] **Extra arguments are silently dropped.** `create foo 1`, `post a 1 2`, `balance a b` all parse fine, and `parse_command_ok_create` asserts on `create foo 1`. Decide on purpose: ignore, or reject with a "too many arguments" error?
- [ ] **A missing `create` argument says "invalid post".** `create` with no name returns `InvalidPost::MissingArg`, so it prints `invalid post: missing argument`. And `balance` still has its own `InvalidBalance::Account` ("missing account"), so the naming nit is only half consolidated. This is the shared-argument-errors question from before.
- [ ] **Output goes three different ways.** Create and post errors: `eprintln!` with `error: `. Balance errors: `eprintln!` without the prefix. Parse errors: `println!` to stdout. Settle this before the writer injection, since that's where it'll show.
- [ ] **Account names are case sensitive, commands aren't.** `create Bob` then `balance bob` is not found. Could be right; worth writing down as a decision either way.
- [ ] **A reader that keeps erroring loops forever.** An I/O error from `read_line` is printed and the loop reads again. Same shape as the old disconnect bug. Rare from stdin, easy to hit from a test reader.
- [ ] **Nit:** `post` looks the account up twice (`balance`, then `get_mut`). Fine at P0. Can it be one lookup?
- [ ] **Untested:** nothing exercises `Repl::run` yet (waits on the writer), and no test covers posting a negative amount past `i32::MIN`.


## Decisions

Record each one with the reason, so it doesn't get re-litigated.

## Gotchas

## Status by phase

- [ ] P0 A list of amounts and a sum (working REPL with create; tests, writer injection and the open bugs left)
- [ ] P1 Double entry
- [ ] P2 Money and accounts as real types
- [ ] P3 Reversals and idempotency
- [ ] P4 The append-only log
- [ ] P5 Stored balances, versions, reconciliation
- [ ] P6 Pending, settled, available
- [ ] P7 Journals, codes, and time
- [ ] P8 The hash chain
- [ ] P9 Concurrent writers and a network face
