use crate::ledger::account_name::AccountName;

pub fn account_name() -> AccountName {
    "foo".parse().unwrap()
}
