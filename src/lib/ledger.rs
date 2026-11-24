use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Ledger {
    pub inner: HashMap<String, Vec<i32>>,
}

impl Ledger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn post(&mut self, account: impl Into<String>, amount: i32) {
        self.inner.entry(account.into()).or_default().push(amount);
    }

    pub fn balance(&self, account: impl AsRef<str>) -> i32 {
        self.inner
            .get(account.as_ref())
            .map_or(0, |v| v.iter().sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn post_adds_accounts() {
        let mut l = Ledger::new();
        assert_eq!(l.inner.keys().count(), 0);
        l.post("foo", 123);
        assert_eq!(l.inner.keys().count(), 1);
        l.post("bar", 456);
        assert_eq!(l.inner.keys().count(), 2);
    }

    #[test]
    fn balance() {
        let mut l = Ledger::new();
        l.post("foo", 1);
        assert_eq!(l.balance("foo"), 1);
        l.post("foo", 1);
        assert_eq!(l.balance("foo"), 2);
    }
}
