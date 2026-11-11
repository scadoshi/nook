use thiserror::Error;

#[derive(Debug, Error)]
#[error("overflow over {0}")]
pub struct Overflow(usize);

#[derive(Debug, Clone)]
pub struct VariableCharacter<const N: usize>(String);

impl<const N: usize> VariableCharacter<N> {
    pub fn new(value: impl Into<String> + AsRef<str>) -> Result<Self, Overflow> {
        if value.as_ref().chars().count() > N {
            return Err(Overflow(N));
        }
        Ok(Self(value.into()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

pub type VarChar<const N: usize> = VariableCharacter<N>;
