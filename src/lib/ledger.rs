use std::collections::HashMap;

use thiserror::Error;

#[derive(Debug, Clone, Default)]
pub struct Ledger {
    pub inner: HashMap<String, Vec<i32>>,
}

const ACCOUNT_NAME_MAX_LEN: usize = 64;
#[derive(Debug, Error)]
pub enum CreateError {
    #[error("account already exists")]
    AlreadyExists,
    #[error("account name cannot exceed {ACCOUNT_NAME_MAX_LEN} characters")]
    ExceedsMaxLength,
}

#[derive(Debug, Error)]
pub enum PostError {
    #[error("account not found")]
    AccountNotFound,
    #[error("post would overflow balance")]
    WouldOverflow,
}

impl From<BalanceError> for PostError {
    fn from(value: BalanceError) -> Self {
        match value {
            BalanceError::AccountNotFound => Self::AccountNotFound,
        }
    }
}

#[derive(Debug, Error)]
pub enum BalanceError {
    #[error("account not found")]
    AccountNotFound,
}

impl Ledger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create(&mut self, account: impl Into<String> + AsRef<str>) -> Result<(), CreateError> {
        if account.as_ref().trim().chars().count() > ACCOUNT_NAME_MAX_LEN {
            return Err(CreateError::ExceedsMaxLength);
        }
        if self.inner.contains_key(account.as_ref()) {
            return Err(CreateError::AlreadyExists);
        }
        self.inner.insert(account.into(), Vec::new());
        Ok(())
    }

    pub fn post(
        &mut self,
        account: impl Into<String> + AsRef<str>,
        amount: i32,
    ) -> Result<(), PostError> {
        if self
            .balance(account.as_ref())?
            .checked_add(amount)
            .is_none()
        {
            return Err(PostError::WouldOverflow);
        }
        match self.inner.get_mut(account.as_ref()) {
            Some(history) => history.push(amount),
            None => return Err(PostError::AccountNotFound),
        }
        Ok(())
    }

    pub fn balance(&self, account: impl AsRef<str>) -> Result<i32, BalanceError> {
        Ok(self
            .inner
            .get(account.as_ref())
            .ok_or(BalanceError::AccountNotFound)?
            .iter()
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_works() {
        let mut l = Ledger::new();
        assert_eq!(l.inner.keys().count(), 0);
        l.create("foo").unwrap();
        assert_eq!(l.inner.keys().count(), 1);
    }

    #[test]
    fn cannot_create_long_account_names() {
        let mut l = Ledger::new();
        assert_eq!(l.inner.keys().count(), 0);
        let long_name = "a".repeat(ACCOUNT_NAME_MAX_LEN + 1);
        assert!(matches!(
            l.create(long_name),
            Err(CreateError::ExceedsMaxLength)
        ));
        assert_eq!(l.inner.keys().count(), 0);
    }

    #[test]
    fn create_trims_account_names() {
        let mut l = Ledger::new();
        l.create("  foo  ").unwrap();
        l.balance("  foo  ").unwrap();
        l.balance("foo").unwrap();
    }

    #[test]
    fn create_cannot_duplicate() {
        let mut l = Ledger::new();
        assert_eq!(l.inner.keys().count(), 0);
        l.create("foo").unwrap();
        assert_eq!(l.inner.keys().count(), 1);
        assert!(matches!(l.create("foo"), Err(CreateError::AlreadyExists)));
        assert_eq!(l.inner.keys().count(), 1);
    }

    #[test]
    fn post_cannot_create() {
        let mut l = Ledger::new();
        assert_eq!(l.inner.keys().count(), 0);
        assert!(matches!(l.post("foo", 1), Err(PostError::AccountNotFound)));
        assert_eq!(l.inner.keys().count(), 0);
    }

    #[test]
    fn post_works() {
        let mut l = Ledger::new();
        l.create("foo").unwrap();
        assert_eq!(l.balance("foo").unwrap(), 0);
        l.post("foo", 1).unwrap();
        assert_eq!(l.balance("foo").unwrap(), 1);
    }

    #[test]
    fn balance_works() {
        let mut l = Ledger::new();
        l.create("foo").unwrap();
        assert_eq!(l.balance("foo").unwrap(), 0);
        l.post("foo", 1).unwrap();
        assert_eq!(l.balance("foo").unwrap(), 1);
        l.post("foo", 1).unwrap();
        assert_eq!(l.balance("foo").unwrap(), 2);
    }

    #[test]
    fn cannot_check_balance_on_non_existent_account() {
        let l = Ledger::new();
        assert!(matches!(
            l.balance("foo"),
            Err(BalanceError::AccountNotFound)
        ));
    }
}
