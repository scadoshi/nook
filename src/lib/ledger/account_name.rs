use std::ops::Deref;
use std::str::FromStr;

use thiserror::Error;

const ACCOUNT_NAME_MAX_LEN: usize = 64;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum ParseAccountNameError {
    #[error("account name cannot be empty")]
    Empty,
    #[error("cannot exceed {ACCOUNT_NAME_MAX_LEN} chars")]
    ExceedsMaxLength,
    #[error("must not include whitespace")]
    IncludesWhitespace,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccountName(String);

impl Deref for AccountName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for AccountName {
    type Err = ParseAccountNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_lowercase();
        if s.is_empty() {
            return Err(ParseAccountNameError::Empty);
        }
        if s.chars().count() > ACCOUNT_NAME_MAX_LEN {
            return Err(ParseAccountNameError::ExceedsMaxLength);
        }
        if s.split_whitespace().count() > 1 {
            return Err(ParseAccountNameError::IncludesWhitespace);
        }
        Ok(Self(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_whitespace() {
        assert_eq!(
            " foo".parse::<AccountName>().unwrap(),
            "foo".parse::<AccountName>().unwrap()
        );
    }

    #[test]
    fn case_insensitive() {
        assert_eq!(
            "foo".parse::<AccountName>().unwrap(),
            "FOO".parse::<AccountName>().unwrap()
        );
    }

    #[test]
    fn rejects_whitespace() {
        let cases = ["", " ", "\n"];
        for str in cases {
            assert_eq!(
                str.parse::<AccountName>().unwrap_err(),
                ParseAccountNameError::Empty,
                "input = {str:?}"
            );
        }
    }

    #[test]
    fn cannot_exceed_max() {
        let cases = (ACCOUNT_NAME_MAX_LEN.saturating_sub(1)
            ..=ACCOUNT_NAME_MAX_LEN.saturating_add(1))
            .map(|n| "a".repeat(n));
        for str in cases {
            let char_count = str.chars().count();
            if char_count > ACCOUNT_NAME_MAX_LEN {
                assert_eq!(
                    str.parse::<AccountName>().unwrap_err(),
                    ParseAccountNameError::ExceedsMaxLength,
                    "input = {str:?}"
                );
            } else {
                assert!(
                    str.parse::<AccountName>().is_ok(),
                    "input = {str:?}, char_count = {char_count:?}",
                );
            }
        }
    }

    #[test]
    fn cannot_include_whitespace() {
        assert_eq!(
            "this contains whitespace"
                .parse::<AccountName>()
                .unwrap_err(),
            ParseAccountNameError::IncludesWhitespace
        );
    }
}
