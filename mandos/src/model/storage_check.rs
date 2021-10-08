use crate::{CheckStorageRaw, InterpreterContext};

use super::*;
#[derive(Debug)]
pub enum CheckStorage {
    Star,
    Equal(CheckStorageDetails),
}

impl CheckStorage {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckStorage::Star)
    }
}

impl InterpretableFrom<CheckStorageRaw> for CheckStorage {
    fn interpret_from(from: CheckStorageRaw, context: &InterpreterContext) -> Self {
        match from {
            CheckStorageRaw::Star => CheckStorage::Star,
            CheckStorageRaw::Equal(m) => {
                CheckStorage::Equal(CheckStorageDetails::interpret_from(m, context))
            },
        }
    }
}