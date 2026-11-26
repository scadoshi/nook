# nook progress

Where the project actually is: phase status, decisions made, gotchas surfaced, what's next. Phases are defined in `plan.md`.

## Where I am

**P0 working, as of 2026-11-25 (late).** A per-account ledger (`HashMap<AccountName, Vec<i32>>`, balance summed on every read) behind a REPL. Files: `ledger/mod.rs` (`Ledger`, `CreateError`, `PostError`, `BalanceError`) and `ledger/account_name.rs`. `AccountName` is a newtype built only through `FromStr`: it trims, lowercases, and rejects empty names, names over 64 chars, and names with inner whitespace. `Command` parses case-insensitively into one `ParseCommandError` (empty, unrecognized, missing or extra arguments, a bad int, a bad account name). `Repl<R: BufRead, W: Write>` reads and writes through injected handles; `main` passes stdin and stdout. Help comes from a `const HELP` table of `HelpEntry { example, message }`. The scadoshi-rust lints, `clippy.toml` and `rustfmt.toml` are in place.

22 tests pass, 18 are ignored stubs (the two label stubs can be filled in now), fmt is clean. Clippy: no errors; the warnings are docs (deferred) plus the few bugs below. 

**Next:**
- Chip away at the open bugs below.
- Fill in the ignored stubs (`#[ignore = "stub, not written yet"]`), starting with the `Repl::run` ones now that the writer is injected.
- Docs, once the shape settles.
- Then P0's real question: notice what's wrong (money from nowhere, nothing says where it went) and move to P1.

## Bugs

Found behavior only. The fixes are his to write.

Fixed so far: the end-of-input loop, case-sensitive commands, balance overflow, dead accessor methods, duplicate missing-account errors, the trim bug, blank names, extra arguments (now rejected), the wrong "invalid post" error on `create`, output going three ways (all through the writer now), account-name case (lowercased by `AccountName`), dollar labels on post and balance, the dead `ACCOUNT_NAME_MAX_LEN` copy, `cannot_exceed_max` (now 63, 64 and 65), and the missing-argument name (now `account name`, matching help).

Open, found 2026-11-25 after `AccountName` and the help table:

- [ ] **A reader that keeps erroring loops forever.** Fix is in (uncommitted): `run` matches `Err(e @ ReplError::Io(..))` and returns it, and shows and continues on `Err(e @ ReplError::Command(..))`. EOF was never part of this; `get_command` maps `Ok(0)` to `Exit`. Left: fill in `run_returns_reader_io_error` and `run_returns_writer_io_error`. Each needs a small test-only type, one implementing `BufRead` whose read always errors, one implementing `Write` whose write always errors. Assert `run` returns `Err(ReplError::Io(..))`. Check it off once the reader test passes.
- [ ] **Error messages read unevenly.** `Empty` says "account name cannot be empty", but `ExceedsMaxLength` and `IncludesWhitespace` drop the subject ("cannot exceed 64 chars"). Parse errors print bare while ledger errors get a `create error:` style prefix. A blank line prints "input was empty"; most REPLs just show the prompt again. Negative amounts read "posted -1 dollars"; fine, but decide on purpose.
- [ ] **Clippy, small:** `get_mut(&account_name)` in `post` borrows something that's already a reference; `Command` is `pub` but never leaves the crate (`unreachable_pub` wants `pub(super)`); `main`'s `eprintln!` wants an `#[expect]` with a reason, since it's the CLI's own output.
- [ ] **`Deref` on `AccountName`.** The guide keeps `Deref` for smart pointers. The `&*account_name` in every `writeln!` is the symptom. What would let you write `{account_name}` directly?
- [ ] **Stale ledger stubs.** `create_allows_name_at_max_len`, `create_rejects_blank_name` and `account_names_are_case_sensitive` were written for the old `String` API. Those rules live in `AccountName` now, and its tests (mostly) cover them. One stub's doc still names `ACCOUNT_NAME_MAX_LEN`. Delete them, or rewrite them as tests of what `Ledger` itself still promises.
- [ ] **Nits:** `post` looks the account up twice (`balance`, then `get_mut`). `rejects_whitespace` actually tests `Empty`, so the name is off. `crate::tests` (the helper module) shares a name with every `mod tests`. Lowercasing before the length check can change the char count for some non-ASCII letters (Turkish `İ` becomes two chars), which is an edge case worth knowing exists.

## Decisions

Record each one with the reason, so it doesn't get re-litigated.

- Account names are case-insensitive: `AccountName` trims and lowercases once, at parse, so every method sees one canonical form.
- Commands reject extra arguments (`TooManyArguments`) rather than ignoring them.
- `Ledger` takes plain types: `AccountName` by value where it stores, `&AccountName` where it only looks.
- Help is a `const` table, not methods on `Command`. Trade-off accepted: nothing forces a new command to get a help row.
- Test style: tables with `"input = {str:?}"` in the message, actual value first in `assert_eq!`, `.parse()` with a typed `let` and the turbofish only inside asserts.

## Gotchas

## Status by phase

- [ ] P0 A list of amounts and a sum (working REPL; open bugs, ignored stubs and docs left)
- [ ] P1 Double entry
- [ ] P2 Money and accounts as real types
- [ ] P3 Reversals and idempotency
- [ ] P4 The append-only log
- [ ] P5 Stored balances, versions, reconciliation
- [ ] P6 Pending, settled, available
- [ ] P7 Journals, codes, and time
- [ ] P8 The hash chain
- [ ] P9 Concurrent writers and a network face
